mod some_module {
    #[derive(Default)]
    struct SomeStruct {}
    impl SomeStruct {
        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
            false
        }
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {}
        fn row_count(&self, parent: &QModelIndex) -> i32 {
            1
        }
    }
    impl qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelBase for SomeStruct {}
    impl qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyGet for SomeStruct {
        fn get_rust_proxy(
            &self,
        ) -> &qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust {
            <Self as qtbridge::bridge::QObjectHolder>::get_rust_proxy(self)
        }
        fn get_rust_proxy_mut(
            &self,
        ) -> &mut qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust {
            <Self as qtbridge::bridge::QObjectHolder>::get_rust_proxy_mut(self)
        }
        fn get_trait(
            &self,
        ) -> &dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelAdapter {
            self
        }
        fn get_trait_mut(
            &mut self,
        ) -> &mut dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelAdapter {
            self
        }
    }
    impl Drop for SomeStruct {
        fn drop(&mut self) {
            <Self as qtbridge::bridge::QObjectHolder>::detach_qobject(self);
        }
    }
    impl qtbridge::bridge::QMetaInfo for SomeStruct {
        fn register_meta(
            mut meta_obj: std::pin::Pin<&mut qtbridge::bridge::DynamicMetaObjectBuilder>,
        ) {
            meta_obj.as_mut().end_meta_registration();
        }
        fn get_shared_dynamic_meta_object() -> &'static qtbridge::bridge::DynamicMetaObjectBuilder {
            use std::any::TypeId;
            use std::cell::RefCell;
            use std::collections::HashMap;
            thread_local ! (static DYNAMIC_META_MAP : RefCell < HashMap < TypeId , * const qtbridge :: bridge :: DynamicMetaObjectBuilder >> = RefCell :: new (HashMap :: new ()));
            let type_id = TypeId::of::<SomeStruct>();
            {
                let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_data_map| {
                    dynamic_meta_data_map
                        .get(&type_id)
                        .copied()
                        .unwrap_or_default()
                });
                if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
                    return meta_data_ref;
                }
            }
            let meta_data_ptr =
                qtbridge::bridge::create_dynamic_meta_object_builder_for_type::<SomeStruct>();
            let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
            DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
                dynamic_meta_data_map.insert(type_id, meta_data_ptr);
            });
            meta_data_ref
        }
    }
    impl qtbridge::qt_type_lib::QMetaTypeInterfaceGet for SomeStruct {
        fn get_qmetatype_interface() -> &'static qtbridge::qt_type_lib::QMetaTypeInterface {
            use qtbridge::qt_type_lib::{QMetaTypeFlag, QMetaTypeInterface};
            use std::any::TypeId;
            use std::cell::RefCell;
            use std::collections::HashMap;
            thread_local ! (static IFACE_MAP : RefCell < HashMap < TypeId , * const QMetaTypeInterface >> = RefCell :: new (HashMap :: new ()));
            let type_id = TypeId::of::<SomeStruct>();
            {
                let iface_ptr = IFACE_MAP
                    .with_borrow(|iface_map| iface_map.get(&type_id).copied().unwrap_or_default());
                if let Some(iface_ref) = unsafe { iface_ptr.as_ref() } {
                    return iface_ref;
                }
            }
            let flags: u32 = (QMetaTypeFlag::NeedsConstruction as u32)
                | (QMetaTypeFlag::NeedsDestruction as u32)
                | (QMetaTypeFlag::NeedsCopyConstruction as u32)
                | (QMetaTypeFlag::NeedsMoveConstruction as u32)
                | (QMetaTypeFlag::PointerToQObject as u32);
            let class_name = std::ffi::CString::new(std::any::type_name::<SomeStruct>())
                .expect("CString::new failed")
                .into_bytes_with_nul()
                .leak();
            pub extern "C" fn meta_object_fn(
                _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
            ) -> *mut qtbridge::qt_type_lib::QMetaObject {
                let meta_obj_data =
                    <SomeStruct as qtbridge::bridge::QMetaInfo>::get_shared_dynamic_meta_object();
                meta_obj_data.get_dynamic_qmetaobject().cast_mut()
            }
            pub extern "C" fn default_ctor(
                _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                addr: *mut u8,
            ) {
                let instance =
                    std::rc::Rc::new(std::cell::RefCell::new(<SomeStruct as Default>::default()));
                < SomeStruct as qtbridge :: bridge :: QObjectHolder > :: register_instance_in_map_with_cpp_proxy_at (addr , instance) ;
            }
            pub extern "C" fn dtor(
                _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                obj: *mut u8,
            ) {
                qtbridge::qt_type_lib::QObject::destruct(obj.cast());
            }
            let iface = qtbridge::qt_type_lib::QMetaTypeInterface::fill_fields(
                <Self as qtbridge::bridge::QObjectHolder>::get_align_of_cpp_proxy(),
                <Self as qtbridge::bridge::QObjectHolder>::get_size_of_cpp_proxy(),
                flags,
                class_name,
                meta_object_fn as usize,
                default_ctor as usize,
                dtor as usize,
            );
            let iface_ref = Box::leak(Box::new(iface));
            let iface_ptr = std::ptr::from_ref(iface_ref);
            IFACE_MAP.with_borrow_mut(|iface_map| iface_map.insert(type_id, iface_ptr));
            iface_ref
        }
    }
    impl SomeStruct {
        fn try_borrow_mut_proxies_map_impl<F, R>(f: F) -> R
        where
            F: FnOnce(
                &mut std::collections::HashMap<
                    *const u8,
                    *const qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust,
                >,
            ) -> R,
        {
            use std::cell::BorrowMutError;
            use std::cell::RefCell;
            use std::collections::HashMap;
            thread_local ! (static INSTANCES : RefCell < HashMap < * const u8 , * const qtbridge :: qt_ifaces :: qabstract_list_model :: QAbstractListModelProxyRust > > = RefCell :: new (HashMap :: new ()));
            INSTANCES
                .try_with(|proxies_map_cell| -> Result<R, BorrowMutError> {
                    let mut proxies_map_ref_mut = proxies_map_cell.try_borrow_mut()?;
                    Ok(f(&mut proxies_map_ref_mut))
                })
                .unwrap()
                .expect("Failed to borrow_mut map of proxies")
        }
    }
    impl qtbridge::bridge::QObjectHolder for SomeStruct {
        type ProxyRust = qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust;
        fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
        where
            F: FnOnce(&mut std::collections::HashMap<*const u8, *const Self::ProxyRust>) -> R,
        {
            Self::try_borrow_mut_proxies_map_impl(f)
        }
        fn register_instance_in_map(
            rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>,
            register_strong: bool,
        ) {
            use std::cell::RefCell;
            use std::rc::Rc;
            let key = (*rust_obj_rc).as_ptr() as *const u8;
            Self::try_borrow_mut_proxies_map(|proxies| {
                let dyn_rc: Rc<
                    RefCell<
                        dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyGet,
                    >,
                > = rust_obj_rc;
                let proxy_ptr = <Self::ProxyRust as qtbridge::bridge::qrustproxy::QRustProxy>::new(
                    &dyn_rc,
                    register_strong,
                    Self::unregister_instance_in_map,
                );
                proxies.insert(key, proxy_ptr);
            })
        }
        fn register_instance_in_map_with_cpp_proxy_at(
            addr: *mut u8,
            rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>,
        ) {
            use std::cell::RefCell;
            use std::rc::Rc;
            let key = (*rust_obj_rc).as_ptr() as *const u8;
            Self::try_borrow_mut_proxies_map(|proxies| {
                let dyn_rc: Rc<
                    RefCell<
                        dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyGet,
                    >,
                > = rust_obj_rc;
                let proxy_ptr = < Self :: ProxyRust as qtbridge :: bridge :: qrustproxy :: QRustProxy > :: new_with_cpp_proxy_at (addr , & dyn_rc , Self :: unregister_instance_in_map) ;
                proxies.insert(key, proxy_ptr);
            })
        }
    }
}
