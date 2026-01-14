use std::collections::BTreeMap;

use qt_gen_common_no_types::multi_type_mapping::MultiTypeMapping;

use crate::generic_instantiation_decl::GenericInstantiationsList;

/// Map where
///   Key - generic param name (e.g. 'K')
///   Value - vector of concrete types that given generic param instantiated with
///
/// For example:
/// ```ignore
/// QHash<K, V>
/// Generic param | [ Instantiations                  ]
///       K       | [ QByteArray | QString  | i32     ]
///       V       | [ QVariant   | QVariant | QString ]
/// ```
pub struct GenericTypesInstantiations {
    map: BTreeMap<syn::Ident, Vec<syn::Path>>
}

impl<'a> GenericTypesInstantiations {
    pub fn new<'f>(idents: impl Iterator<Item = &'f syn::Ident>, insts: &GenericInstantiationsList) -> syn::Result<Self> {
        let idents = idents.cloned()
            .collect::<Vec<_>>();
        insts.check_size(idents.len())?;

        let map = idents.into_iter()
            .enumerate()
            .map(|(idx, ident)| {
                let ident_types = insts.list().iter()
                    .map(|inst_decl| inst_decl.types()
                        .list()[idx]
                        .clone())
                    .collect();
                (ident, ident_types)
            })
            .collect();

        Ok(Self {
            map
        })
    }

    pub fn iter_type_insts(&'a self) -> impl Iterator<Item = (&'a syn::Ident, &'a Vec<syn::Path>)> {
        self.map.iter()
    }

    pub fn iter_type_maps(&'a self) -> impl Iterator<Item = MultiTypeMapping> + 'a {
        let count = self.get_inst_count();
        (0..count)
            .map(|inst_idx| { self.get_type_map_num(inst_idx) })
    }

    pub fn get_inst_count(&self) -> usize {
        self.map.first_key_value()
            .map(|(_k, v)| v.len())
            .unwrap_or(0)
    }

    /// Get generic types instantiation for n-th type set
    pub fn get_type_map_num(&self, num: usize) -> MultiTypeMapping {
        let map = self.map.iter()
            .map(|(k, v)| (k.clone(), v.get(num).unwrap().clone()))
            .collect::<BTreeMap<_, _>>();
        MultiTypeMapping::from(map)
    }
}

