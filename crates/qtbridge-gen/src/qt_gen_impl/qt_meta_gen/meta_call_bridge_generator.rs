use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

use qtbridge_gen_common::signature_utils::{get_typed_arg_ident, get_typed_args};
use qtbridge_gen_common::type_utils::{ValuePass, get_type_pass, is_ref, remove_ref, remove_refs};

/// Generates code to connect a Rust function to a metacall (e.g. signal or slot).
pub struct MetaCallBridgeGenerator<'a> {
    inputs: Vec<MetaCallArg<'a>>,
    output: Option<&'a syn::Type>,
}

impl<'a> MetaCallBridgeGenerator<'a> {
    pub fn new(sign: &'a syn::Signature) -> syn::Result<Self> {
        let inputs = get_typed_args(sign)
            .map(|arg| Ok(MetaCallArg {
                ident: get_typed_arg_ident(arg)?,
                user_type: arg.ty.as_ref(),
            }))
            .collect::<syn::Result<_>>()?;
        let output = match &sign.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty.as_ref()),
        };
        Ok(Self { inputs, output })
    }

    /// Return an iterator over the user argument types (refs stripped) for meta-type registration.
    pub fn get_input_metatypes(&self) -> impl Iterator<Item = &syn::Type> {
        self.inputs.iter().map(|arg| remove_refs(arg.user_type))
    }

    /// Return the user return type (refs stripped) for meta-type registration.
    pub fn get_output_metatype(&self) -> Option<&syn::Type> {
        self.output.map(remove_refs)
    }

    /// Generates bridge code for invoking a user-defined function from a metacall (e.g., slot invocation).
    /// Handles unpacking and conversion of arguments and return values.
    /// `fn_call` is the expression representing the call to the user-defined function.
    ///
    /// Returns a code block that:
    /// - Defines references to the arguments.
    /// - Stores intermediate variables to be passed to the Rust function.
    /// - Invokes the Rust function (`fn_call`).
    /// - Stores the result in the metacall parameter array if needed.
    pub fn generate_bridge_metacall_to_user_fn(&self, mut fn_call: syn::ExprMethodCall) -> syn::Result<TokenStream> {
        // Generate code: Cast arguments to the proper wire type
        let input_refs: Vec<_> = self.inputs.iter().enumerate()
            .map(|(idx, arg)| gen_input_ref(arg.user_type, idx))
            .collect();
        // Generate code: Convert the wire type to the proper type
        let input_vars: Vec<_> = self.inputs.iter().enumerate()
            .map(|(idx, arg)| gen_input_var(arg.user_type, idx))
            .collect();
        // Generate code: Make a reference if required
        let input_pass: Vec<_> = self.inputs.iter().enumerate()
            .map(|(idx, arg)| gen_pass_expr(arg.user_type, idx))
            .collect::<syn::Result<_>>()?;

        // Append user provided arguments (if any) with arguments taken from the input signature.
        fn_call.args.extend(input_pass);

        // Invoke the Rust function. Store its result in argv[0] if the function returns a value.
        let invoke_and_maybe_write_result = match self.output {
            Some(output) => {
                let result_var = format_ident!("result");
                let result_conv_var = format_ident!("result_conv");
                let result_conv = gen_to_wire(output, &result_var, &result_conv_var);
                let output_ptr = gen_output_ptr(output);
                let write_output = gen_write_output(&result_conv_var);
                quote! {
                    let #result_var = #fn_call;
                    #result_conv
                    #output_ptr
                    #write_output
                }
            },
            None => fn_call.to_token_stream(),
        };

        Ok(quote! {
            #(#input_refs)*
            #(#input_vars)*
            #invoke_and_maybe_write_result
        })
    }

    /// Generates the argv array for signal emission, converting args to their wire types.
    pub fn generate_argv_setup_for_signals(&self) -> TokenStream {
        let argv_size = self.inputs.len() + 1;
        let mut arg_vars = Vec::new();
        let mut argv_arr_init = Vec::with_capacity(argv_size);

        for (idx, arg) in self.inputs.iter().enumerate() {
            let var_ident = arg_var_ident(idx);
            let arg_ident = &arg.ident;
            let user_type_no_ref = remove_refs(arg.user_type);
            let arg_ref = match get_type_pass(arg.user_type) {
                ValuePass::ByValue => quote! { &#arg_ident },
                _ => quote! { #arg_ident },
            };
            arg_vars.push(quote! {
                let #var_ident = <#user_type_no_ref as QMetaCallArg>::to_wire(#arg_ref);
            });
            argv_arr_init.push(quote! { std::ptr::from_ref(&#var_ident).cast() });
        }

        quote! {
            #(#arg_vars)*
            let argv: [*const u8; #argv_size] = [
                std::ptr::null(),   // No value return.
                #(#argv_arr_init),*
            ];
        }
    }
}

struct MetaCallArg<'a> {
    ident: syn::Ident,
    user_type: &'a syn::Type,
}

fn gen_input_ref(user_type: &syn::Type, idx: usize) -> syn::Stmt {
    let user_type_no_ref = remove_refs(user_type);
    let ref_ident = input_ref_ident(idx);
    let inputs = get_inputs_ident();
    parse_quote! {
        let #ref_ident = unsafe {
            #inputs[#idx]
                .cast::<<#user_type_no_ref as QMetaCallArg>::WireType>()
                .as_ref()
        }.expect("Argument reference is null");
    }
}

fn gen_input_var(user_type: &syn::Type, idx: usize) -> syn::Stmt {
    let var_ident = arg_var_ident(idx);
    let ref_ident = input_ref_ident(idx);
    let user_type_no_ref = remove_refs(user_type);
    parse_quote! {
        let #var_ident = <#user_type_no_ref as QMetaCallArg>::from_wire(#ref_ident);
    }
}

fn gen_pass_expr(user_type: &syn::Type, idx: usize) -> syn::Result<syn::Expr> {
    let var_ident = arg_var_ident(idx);
    match get_type_pass(user_type) {
        ValuePass::ByValue => Ok(parse_quote! { #var_ident }),
        ValuePass::ByConstReference => Ok(parse_quote! { &#var_ident }),
        ValuePass::ByMutReference =>
            Err(syn::Error::new(user_type.span(),
                "Arguments passed by mutable references are not supported")),
    }
}

fn gen_to_wire(user_type: &syn::Type, from: &syn::Ident, to: &syn::Ident) -> syn::Stmt {
    let user_type_no_ref = remove_ref(user_type);
    let ref_from = match is_ref(user_type) {
        true => quote! { #from },
        false => quote! { &#from },
    };
    parse_quote! {
        let #to = <#user_type_no_ref as QMetaCallArg>::to_wire(#ref_from);
    }
}

fn gen_output_ptr(user_type: &syn::Type) -> syn::Stmt {
    let user_type_no_ref = remove_ref(user_type);
    let outputs = get_outputs_ident();
    parse_quote! {
        let output_ptr: *mut <#user_type_no_ref as QMetaCallArg>::WireType = #outputs[0].cast();
    }
}

fn gen_write_output(var_ident: &syn::Ident) -> syn::Expr {
    parse_quote! {
        unsafe { std::ptr::write(output_ptr, #var_ident) }
    }
}

pub fn get_inputs_ident() -> syn::Ident {
    format_ident!("inputs")
}

pub fn get_outputs_ident() -> syn::Ident {
    format_ident!("outputs")
}

fn input_ref_ident(idx: usize) -> syn::Ident {
    format_ident!("arg_{idx}_ref")
}

fn arg_var_ident(idx: usize) -> syn::Ident {
    format_ident!("arg_{idx}_var")
}
