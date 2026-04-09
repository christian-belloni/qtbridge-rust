use quote::ToTokens;
use syn::spanned::Spanned;

use crate::type_registry;
use type_registry::{QtType, StandardContainer, StandardType, StringType};
use type_registry::qt::generic::{QtGenericArg, QtGenericTypeWithoutArgs};
use type_registry::type_traits::{FindType, GenericArgs, MetaTypeId, TypeInfo, TypeName};
use crate::signature_utils::{get_return_type, get_typed_args, is_arg_self_ref};
use crate::type_to_string::type_to_string_fallback;
use crate::type_utils::{is_mut_ref, path_from_type, path_to_type, remove_ref};

/// Checks whether the given signature can participate in meta-calls
/// (as slot callbacks or property getters/setters).
///
/// All argument and return types must be `QMetaType's.
pub fn check_meta_call_signature_types(src: &syn::Signature) -> syn::Result<()> {
    if !src.inputs.first().is_some_and(|arg| is_arg_self_ref(arg, None)) {
        return Err(syn::Error::new(src.ident.span(), "First argument must be &self"));
    }

    get_typed_args(src)
        .try_for_each(|arg| {
            check_meta_call_signature_type(arg.ty.as_ref())
                .map_err(|err| syn::Error::new(err.span(), format!("The function argument is not compatible with meta call.\n{err}")))
        })?;
    get_return_type(&src.output)
        .map(|ty| {
            check_meta_call_signature_type(ty)
                .map_err(|err| syn::Error::new(err.span(), format!("The function return type is not compatible with meta call.\n{err}")))
        })
        .unwrap_or(Ok(()))
}

/// Checks whether the given type can participate in meta-calls.
fn check_meta_call_signature_type(ty: &syn::Type) -> syn::Result<()> {
    let ty_wo_ref = remove_ref(ty);
    match ty_wo_ref {
        syn::Type::Path(_) => path_from_type(ty_wo_ref),
        syn::Type::Array(array) =>
            Err(syn::Error::new(array.span(), "Arrays are currently not supported")),
        syn::Type::Ptr(ptr) =>
            Err(syn::Error::new(ptr.span(), "Pointers are not supported")),
        syn::Type::Reference(type_ref) =>
            Err(syn::Error::new(type_ref.span(), "References to reference are not supported")),
        syn::Type::Slice(slice) =>
            Err(syn::Error::new(slice.span(), "Slices are currently not supported")),
        syn::Type::Tuple(tuple) =>
            Err(syn::Error::new(tuple.span(), "Tuples are not supported")),
        _ => Err(syn::Error::new(ty_wo_ref.span(), format!("Type category ('{:?}') of type '{}' is not supported", std::mem::discriminant(ty_wo_ref), type_to_string_fallback(ty_wo_ref))))
    }?;

    if is_mut_ref(ty) {
        return Err(syn::Error::new(ty.span(), format!("Mutable references are not supported. Found: '{}'", type_to_string_fallback(ty))))
    }

    if !is_type_mapped_to_qmetatype(ty) {
        return Err(syn::Error::new(ty.span(), format!("Type '{}' can't be converted to meta type", type_to_string_fallback(ty))))
    }

    Ok(())
}

/// Returns true if the type can be stored in QMetaType.
pub fn is_type_mapped_to_qmetatype(ty: &syn::Type) -> bool {
    get_qmetatype_support_for_type(ty).is_ok()
}

/// Checks whether the given type is supported by the `QMetaType` system.
///
/// # Returns
///
/// * `Ok(None)` if the input type is already supported by `QMetaType`.
/// * `Ok(Some(_))` if the input type is not directly supported but can be converted
///   to an intermediate Qt-specific type that *is* supported for use in Qt meta-calls.
/// * `Err(_)` if the input type is neither supported by `QMetaType` nor convertible.
///
pub fn get_qmetatype_support_for_type(mut src: &syn::Type) -> syn::Result<Option<syn::Type>> {
    // Unwrap if reference
    src = remove_ref(src);
    let path = path_from_type(src)?;
    let ty = type_registry::Type::find_by_path_checked(path)?;
    let meta_id = ty.metatype_id();

    match meta_id {
        MetaTypeId::None => { // Conversion to the intermediate type is needed
            let ty = get_intermediate_type_for_not_metatype(&ty, path)?;
            Ok(Some(ty))
        },
        _ => // Conversion is not needed
            Ok(None),
    }
}

/// Handle cases where the given type is not a QMetaType
/// and requires conversion to an intermediate, supported type.
fn get_intermediate_type_for_not_metatype(src: &type_registry::Type, src_path: &syn::Path) -> syn::Result<syn::Type> {
    let ty = match src {
        type_registry::Type::Standard(standard) =>
            get_intermediate_type_standard(standard, src_path)?,
        type_registry::Type::Cxx(_) =>
            return Err(syn::Error::new(src_path.span(), "Cxx types are not supported")),
        type_registry::Type::Qt(qt) =>
            get_intermediate_type_qt(qt, src_path)?,
    };
    let path = ty.complement_partially_qualified_path(&syn::parse_str(ty.full_name())?)?;
    Ok(path_to_type(path))
}

fn get_intermediate_type_standard(src: &StandardType, src_path: &syn::Path) -> syn::Result<QtType> {
    match src {
        StandardType::String(string) => get_intermediate_type_string(string, src_path),
        StandardType::Container(container) => get_intermediate_type_container(container, src_path),
        _ => Err(syn::Error::new(src_path.span(), format!("Type '{}' is currently unsupported in metacalls", src.full_name())))
    }
}

fn get_intermediate_type_string(src: &StringType, src_path: &syn::Path) -> syn::Result<QtType> {
    let name = src.name();
    match src.name() {
        "String" | "str" => {
            let ty = QtType::find_by_name("QString")
                .ok_or_else(|| syn::Error::new(src_path.span(), "Failed to find QString"))?;
            Ok(ty)
        },
        _ => Err(syn::Error::new(src_path.span(), format!("Unsupported string type '{name}'")))
    }
}

fn get_intermediate_type_container(src: &StandardContainer, src_path: &syn::Path) -> syn::Result<QtType> {
    let src_name = src.name();
    let qt_container = match src_name {
        "Vec" => "QList",
        _ =>
            return Err(syn::Error::new(src_path.span(), format!("Standard container: '{src_name}' is not supported for metacalls")))
    };

    let args: Vec<QtGenericArg> = (0..src.generic_arg_count())
        .map(|arg_idx| {
            let src_arg_type = src.generic_arg_syn(arg_idx)
                .ok_or_else(|| syn::Error::new(src_path.span(), "Failed to get generic argument"))?;
            let inter_arg_type = get_qmetatype_support_for_type(&src_arg_type)?
                .unwrap_or_else(|| src_arg_type.clone());
            let arg_path = path_from_type(&inter_arg_type)
                .map_err(|err| syn::Error::new(inter_arg_type.span(), format!("Type '{}' is unsupported as argument in {src_name} container. {err}", inter_arg_type.to_token_stream())))?;
            let arg_ty = type_registry::Type::find_by_path_checked(arg_path)?;
            QtGenericArg::try_from(&arg_ty)
                .map_err(|_| syn::Error::new(arg_path.span(), format!("Unsupported type of argument for Qt collection: '{}'", arg_ty.name())))
        })
        .collect::<syn::Result<_>>()?;

    let generic_wo_args = QtGenericTypeWithoutArgs::find_by_name(qt_container)
        .ok_or_else(|| syn::Error::new(src_path.span(), format!("Failed to find in collection type '{qt_container}'")))?;
    let generic_w_args = generic_wo_args.set_args(args)
        .map_err(|err| syn::Error::new(src_path.span(), format!("Failed to set argument types for container type '{qt_container}'.\nError: {err}")))?;
    let mono = generic_w_args.get_monomorphed_type()
        .ok_or_else(|| syn::Error::new(src_path.span(), format!("Unsupported instantiation of generic type: '{}'", generic_w_args.full_name())))?;
    Ok(mono.into())
}

fn get_intermediate_type_qt(_src: &QtType, src_path: &syn::Path) -> syn::Result<QtType> {
    // Not supported ATM, but may be implemented in the future.
    Err(syn::Error::new(src_path.span(), "Qt types without metaTypeId are currently unsupported"))
}
