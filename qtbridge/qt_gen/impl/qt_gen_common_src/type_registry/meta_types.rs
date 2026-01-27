use quote::ToTokens;
use syn::spanned::Spanned;

use crate::type_registry;
use type_registry::{QtType, StandardContainer, StandardType, StringType};
use type_registry::qt::generic::{QtGenericArg, QtGenericTypeWithoutArgs};
use type_registry::type_traits::{FindType, MetaTypeId, TypeInfo, TypeName};
use crate::type_utils::{get_angle_bracketed_generic_arguments_of_last_path_segment, path_to_type};


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
    if let syn::Type::Reference(src_ref) = src {
        src = src_ref.elem.as_ref();
    }

    match src {
        syn::Type::Path(type_path) => {
            let path = &type_path.path;
            let ty = type_registry::Type::find_by_partial_path_result(path)?;
            let meta_id = ty.metatype_id();

            match meta_id {
                MetaTypeId::None => { // Conversion to the intermediate type is needed
                    let ty = get_intermediate_type_for_not_metatype(&ty, path)?;
                    Ok(Some(ty))
                },
                _ => // Conversion is not needed
                    Ok(None),
            }
        },
        syn::Type::Reference(_) =>
            Err(syn::Error::new(src.span(), "Reference to reference is not supported")),
        // TODO: support more type categories (e.g. slices, arrays, etc.)?
        _ => Err(syn::Error::new(src.span(), format!("Type category ('{:?}') of type '{}' is currently unsupported", std::mem::discriminant(src), src.to_token_stream())))
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

    let args: Vec<QtGenericArg> = (0..src.generic_arg_count()).into_iter()
        .map(|arg_idx| {
            let src_arg_type = get_generic_arg_type_path(src_path, arg_idx)?;
            let inter_arg_type_ = get_qmetatype_support_for_type(&src_arg_type)?
                .unwrap_or_else(|| src_arg_type.clone());
            let syn::Type::Path(arg_type_path) = inter_arg_type_ else {
                return Err(syn::Error::new(inter_arg_type_.span(), format!("Type '{}' is unsupported as argument in {src_name} container", inter_arg_type_.to_token_stream())))
            };
            let arg_path = &arg_type_path.path;
            let arg_ty = type_registry::Type::find_by_partial_path_result(arg_path)?;
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

fn get_generic_arg_type_path(src: &syn::Path, index: usize) -> syn::Result<&syn::Type> {
    let args = &get_angle_bracketed_generic_arguments_of_last_path_segment(src)
        .ok_or_else(|| syn::Error::new(src.span(), "Failed to get generic arguments"))?
        .args;
    let arg = args.get(index)
        .ok_or_else(|| syn::Error::new(src.span(), format!("No generic argument #{index} in '{}'", src.to_token_stream())))?;
    let syn::GenericArgument::Type(gen_arg_type) = arg else {
        return Err(syn::Error::new(src.span(), "Expected generic type argument"))
    };
    Ok(gen_arg_type)
}

fn get_intermediate_type_qt(_src: &QtType, src_path: &syn::Path) -> syn::Result<QtType> {
    // Not supported ATM, but may be implemented in the future.
    Err(syn::Error::new(src_path.span(), "Qt types without metaTypeId are currently unsupported"))
}

