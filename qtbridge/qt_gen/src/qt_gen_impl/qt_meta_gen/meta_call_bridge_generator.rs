use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

use qt_gen_common::signature_utils::{get_typed_arg_ident, get_typed_args};
use qt_gen_common::type_registry::meta_types::get_qmetatype_support_for_type;
use qt_gen_common::type_utils::{ValuePass, get_type_pass, remove_ref};

/// Generates code to connect a Rust function to a metacall (e.g. signal or slot).
pub struct MetaCallBridgeGenerator<'a> {
    inputs: Vec<MetaCallArg<'a>>,
    output: Option<MetaCallType<'a>>
}

impl<'a> MetaCallBridgeGenerator<'a> {
    pub fn new(sign: &'a syn::Signature) -> syn::Result<Self> {
        let inputs = get_typed_args(sign)
            .map(MetaCallArg::try_from)
            .collect::<syn::Result<_>>()?;
        let output = match &sign.output {
            syn::ReturnType::Default =>
                None,
            syn::ReturnType::Type(_, ty) =>
                Some(MetaCallType::try_from(ty.as_ref())?)
        };

        Ok(Self {
            inputs,
            output,
        })
    }

    /// Return an iterator over meta-types corresponding to the input argument types.
    pub fn get_input_metatypes(&self) -> impl Iterator<Item = &syn::Type> {
        self.inputs.iter()
            .map(|arg| arg.ty.meta_type())
    }

    /// Return a meta-types corresponding to the function return type.
    pub fn get_output_metatype(&self) -> Option<&syn::Type> {
        self.output.as_ref()
            .map(|val| val.meta_type())
    }

    /// Generates bridge code for invoking a user defined function from a metacall (e.g., slot invocation),
    /// handling unpack and conversion of input arguments and return type.
    /// # Arguments
    ///
    /// * `fn_call` - The expression representing the call to the user-defined function.
    ///
    /// # Returns
    ///
    /// A `TokenStream`containing code block that:
    /// - Defines references to the input arguments.
    /// - Stores intermediate variables to be passed to the Rust function.
    /// - Invokes the Rust function (`fn_call`).
    /// - Stores the result in the metacall parameters array if needed.
    pub fn generate_bridge_metacall_to_user_fn(&self, mut fn_call: syn::ExprMethodCall) -> syn::Result<TokenStream> {
        let input_iter = self.inputs.iter()
            .map(|arg| &arg.ty)
            .enumerate();

        // Expressions used as arguments to the Rust function.
        let input_pass = input_iter.clone()
            .map(|(idx, arg)| arg.generate_pass_input_expression(idx))
            .collect::<syn::Result<Vec<_>>>()?;

        // Definitions of references to source parameters cast from input raw pointers.
        let input_refs = input_iter.clone()
            .map(|(idx, arg)| arg.generate_reference_to_input_meta_value(idx));

        // Intermediate variables converted from input meta types to be passed to the Rust function.
        let input_vars = input_iter
            .map(|(idx, arg)| arg.generate_store_argv_input_to_variable(idx));

        // Append user provided arguments (if any) with arguments taken from the input signature.
        fn_call.args.extend(input_pass);

        // Invoke the Rust function. Store its result in argv[0] if the function returns a value.
        let invoke_and_maybe_write_result = match &self.output {
            Some(output) => {
                let result_var = format_ident!("result");
                let output_ptr = output.generate_pointer_to_output_meta_value(0);
                let write_output = output.generate_write_output_ptr_expression(&result_var, 0);

                quote! {
                    let #result_var = #fn_call;
                    #output_ptr
                    #write_output
                }
            },
            None => fn_call.to_token_stream(),
        };

        // Putting it all together.
        let code = quote! {
            #(#input_refs)*
            #(#input_vars)*
            #invoke_and_maybe_write_result
        };
        Ok(code)
    }

    /// Generates bridge code for invoking a metacall from the Rust function (e.g., signal invocation),
    /// handling packing and conversion of input arguments and return type.
    /// # Arguments
    ///
    /// * `fn_call` - The expression representing the metacall (but using standard, non-metacall specific types).
    ///
    /// # Returns
    ///
    /// A `TokenStream`containing code block that:
    /// - Prepares the metacall parameters array.
    /// - Invokes the `fn_metacall` function.
    pub fn generate_bridge_user_fn_to_metacall(&self, mut fn_metacall: syn::ExprMethodCall) -> syn::Result<TokenStream> {
        let input_size = self.inputs.len();
        let argv_size = input_size + 1;

        // Iterate over the arguments of the function.
        // Define intermediate variables for arguments, if necessary.
        // Provide initializers for the corresponding elements in the metacall parameters array.
        let mut arg_vars = Vec::new();
        let mut argv_arr_init = Vec::with_capacity(argv_size);
        for (idx, arg) in self.inputs.iter().enumerate() {
            let arg_pass = get_type_pass(arg.ty.user_type);
            let arg_ident = &arg.ident;

            match arg.ty.intermediate_meta_type() {
                Some(meta_type) => {
                    // An intermediate variable is needed.
                    let var_ident = get_arg_intermediate_var_ident(idx);

                    let arg_maybe_borrow = match arg_pass {
                        ValuePass::ByValue => quote! { (&#arg_ident) },
                        _ => quote! { #arg_ident }
                    };
                    arg_vars.push(quote! {
                        let #var_ident: qtbridge::#meta_type = #arg_maybe_borrow.into();
                    });
                    argv_arr_init.push(quote! {
                        std::ptr::from_ref(&#var_ident).cast()
                    });
                }
                None => {
                    let arg_ref = match arg_pass {
                        ValuePass::ByValue => quote! { &#arg_ident },
                        _ => quote! { #arg_ident },
                    };
                    let init = quote! { std::ptr::from_ref(#arg_ref).cast() };
                    argv_arr_init.push(init);
                },
            }
        };

        // Append `argv` to the list of call arguments.
        fn_metacall.args.push(syn::parse_quote!{ argv.as_slice() });

        // Putting it all together.
        let code = quote! {
            #(#arg_vars)*
            let argv: [*const u8; #argv_size] = [
                std::ptr::null(),   // No value return.
                #(#argv_arr_init),*
            ];
            #fn_metacall
        };
        Ok(code)
    }
}

/// Encapsulates the type and the ident of an argument in a metacall.
struct MetaCallArg<'a> {
    ident: syn::Ident,
    ty: MetaCallType<'a>,
}

impl<'a> TryFrom<&'a syn::PatType> for MetaCallArg<'a> {
    type Error = syn::Error;

    fn try_from(arg: &'a syn::PatType) -> syn::Result<Self> {
        Ok(Self {
            ident: get_typed_arg_ident(arg)?,
            ty: MetaCallType::try_from(arg.ty.as_ref())?,
        })
    }
}

/// Encapsulates the type of an argument or return value in a metacall.
struct MetaCallType<'a> {
    /// The type of the argument in the signature of user-defined function.
    user_type: &'a syn::Type,

    /// The type used internally in metacall to pass the argument (if it is different from `user_type`).
    intermediate_meta_type: Option<syn::Type>,
}

impl<'a> TryFrom<&'a syn::Type> for MetaCallType<'a> {
    type Error = syn::Error;

    fn try_from(user_type: &'a syn::Type) -> syn::Result<Self> {
        Ok(Self {
            user_type,
            intermediate_meta_type: get_qmetatype_support_for_type(user_type)?,
        })
    }
}

impl<'a> MetaCallType<'a> {
    fn intermediate_meta_type(&self) -> Option<&syn::Type> {
        self.intermediate_meta_type.as_ref()
    }

    fn meta_type(&self) -> &syn::Type {
        self.intermediate_meta_type()
            .unwrap_or_else(|| remove_ref(self.user_type))
    }

    /// Generates a definition of a typed immutable reference to the given input parameter.
    fn generate_reference_to_input_meta_value(&self, idx: usize) -> syn::Stmt {
        let meta_type = self.intermediate_meta_type()
            .unwrap_or_else(|| remove_ref(self.user_type));

        let input_ref_ident = get_input_ref_ident(idx);
        let inputs_ident = get_inputs_ident();
        parse_quote! {
            let #input_ref_ident = unsafe {
                #inputs_ident[#idx].cast::<#meta_type>().as_ref()
            }.expect("Argument reference is null");
        }
    }

    /// Generates a definition of a mutable pointer to the given output parameter.
    fn generate_pointer_to_output_meta_value(&self, idx: usize) -> syn::Stmt {
        let meta_type = self.intermediate_meta_type()
            .unwrap_or_else(|| remove_ref(self.user_type));

        let output_ptr_ident = get_output_ptr_ident(idx);
        let outputs_ident = get_outputs_ident();
        parse_quote! {
            let #output_ptr_ident: *mut #meta_type = #outputs_ident[0].cast();
        }
    }

    /// Generates a definition of an intermediate variable containing a value copied/converted
    /// from a reference to the source data.
    ///
    /// The variable is initialized from the input `argv` array (given as `*const *const c_void`).
    /// The variable is needed when a type conversion is required
    /// or when the argument must be passed by value.
    fn generate_store_argv_input_to_variable(&self, idx: usize) -> Option<syn::Stmt> {
        let arg_var_ident = get_arg_intermediate_var_ident(idx);
        let arg_ref_ident = get_input_ref_ident(idx);
        let arg_type_wo_ref = remove_ref(self.user_type);

        // A variable is needed for the type conversion.
        if self.intermediate_meta_type().is_some() {
            return Some(parse_quote! {
                let #arg_var_ident: <#arg_type_wo_ref as ToOwned>::Owned = #arg_ref_ident.into();
            })
        }

        match get_type_pass(self.user_type) {
            ValuePass::ByValue =>
                // Variable is needed to hold value that will be passed to the user function.
                Some(parse_quote! {
                    let #arg_var_ident: #arg_type_wo_ref = #arg_ref_ident.clone();
                }),
            _ => None // Variable is not needed.
        }
    }

    // Produce the code passing an argument to the user function.
    fn generate_pass_input_expression(&self, idx: usize) -> syn::Result<syn::Expr> {
        let var_ident = get_arg_intermediate_var_ident(idx);
        let ref_ident = get_input_ref_ident(idx);

        let expr = match get_type_pass(self.user_type) {
            ValuePass::ByValue => // Pass the intermediate variable by value.
                parse_quote!{ #var_ident },
            ValuePass::ByConstReference => {
                match self.intermediate_meta_type() {
                    Some(_) => // Pass the intermediate variable by reference.
                        parse_quote! { &#var_ident },
                    None => // Pass the input argument reference as is.
                        parse_quote! { #ref_ident },
                }
            },
            ValuePass::ByMutReference =>
                return Err(syn::Error::new(self.user_type.span(),
                    "Arguments passed by mutable references are not supported"))
        };
        Ok(expr)
    }

    /// Produce the code storing a value in the metacall output.
    fn generate_write_output_ptr_expression(&self, var_ident: &syn::Ident, idx: usize) -> syn::Expr {
        let output_ptr_ident = get_output_ptr_ident(idx);
        let maybe_into = self.intermediate_meta_type().is_some()
            .then(|| quote!{ .into() });

        parse_quote! {
            unsafe {
                std::ptr::write(#output_ptr_ident, #var_ident #maybe_into)
            }
        }
    }
}

pub fn get_inputs_ident() -> syn::Ident {
    format_ident!("inputs")
}

pub fn get_outputs_ident() -> syn::Ident {
    format_ident!("outputs")
}

fn get_input_ref_ident(idx: usize) -> syn::Ident {
    format_ident!("arg_{idx}_ref")
}

fn get_output_ptr_ident(idx: usize) -> syn::Ident {
    format_ident!("output_{idx}_ptr")
}

fn get_arg_intermediate_var_ident(idx: usize) -> syn::Ident {
    format_ident!("arg_{idx}_var")
}
