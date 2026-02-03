mod some_module {
    #[derive(Default)]
    struct SomeStruct {}
    impl SomeStruct {
        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
            false
        }
        fn d_data_(&self, index: &QModelIndex, role: i32) -> QVariant {}
        fn row_count(&self, parent: &QModelIndex) -> i32 {
            1
        }
    }
    impl SomeStruct {
        fn index(
            &self,
            row: i32,
            column: i32,
            parent: &qtbridge::qt_type_lib::QModelIndex,
        ) -> qtbridge::qt_type_lib::QModelIndex {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy(self);
            proxy.base_index(row, column, parent)
        }
        fn role_names(
            &self,
        ) -> qtbridge::qt_type_lib::QHash<i32, qtbridge::qt_type_lib::QByteArray> {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy(self);
            proxy.base_role_names()
        }
        fn base_set_data(
            &mut self,
            index: &qtbridge::qt_type_lib::QModelIndex,
            value: &qtbridge::qt_type_lib::QVariant,
            role: i32,
        ) -> bool {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_set_data(index, value, role)
        }
        fn remove_rows(
            &mut self,
            first: i32,
            count: i32,
            parent: &qtbridge::qt_type_lib::QModelIndex,
        ) -> bool {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_remove_rows(first, count, parent)
        }
        fn sibling(
            &self,
            row: i32,
            column: i32,
            idx: &qtbridge::qt_type_lib::QModelIndex,
        ) -> qtbridge::qt_type_lib::QModelIndex {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy(self);
            proxy.base_sibling(row, column, idx)
        }
        fn data_changed(
            &mut self,
            topLeft: &qtbridge::qt_type_lib::QModelIndex,
            bottomRight: &qtbridge::qt_type_lib::QModelIndex,
        ) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_data_changed(topLeft, bottomRight)
        }
        fn begin_insert_rows(
            &mut self,
            parent: &qtbridge::qt_type_lib::QModelIndex,
            first: i32,
            last: i32,
        ) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_begin_insert_rows(parent, first, last)
        }
        fn end_insert_rows(&mut self) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_end_insert_rows()
        }
        fn begin_move_rows(
            &mut self,
            sourceParent: &qtbridge::qt_type_lib::QModelIndex,
            sourceFirst: i32,
            sourceLast: i32,
            destinationParent: &qtbridge::qt_type_lib::QModelIndex,
            destinationChild: i32,
        ) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_begin_move_rows(
                sourceParent,
                sourceFirst,
                sourceLast,
                destinationParent,
                destinationChild,
            )
        }
        fn end_move_rows(&mut self) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_end_move_rows()
        }
        fn begin_remove_rows(
            &mut self,
            parent: &qtbridge::qt_type_lib::QModelIndex,
            first: i32,
            last: i32,
        ) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_begin_remove_rows(parent, first, last)
        }
        fn end_remove_rows(&mut self) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_end_remove_rows()
        }
        fn begin_reset_model(&mut self) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_begin_reset_model()
        }
        fn end_reset_model(&mut self) {
            let proxy = <Self as qtbridge::qt_traits::QObjectHolder>::get_rust_proxy_mut(self);
            proxy.base_end_reset_model()
        }
    }
    impl qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModel for SomeStruct {
        fn index(
            &self,
            row: i32,
            column: i32,
            parent: &qtbridge::qt_type_lib::QModelIndex,
        ) -> qtbridge::qt_type_lib::QModelIndex {
            SomeStruct::index(self, row, column, parent)
        }
        fn row_count(&self, parent: &qtbridge::qt_type_lib::QModelIndex) -> i32 {
            SomeStruct::row_count(self, parent)
        }
        fn data(
            &self,
            index: &qtbridge::qt_type_lib::QModelIndex,
            role: i32,
        ) -> qtbridge::qt_type_lib::QVariant {
            SomeStruct::d_data_(self, index, role)
        }
        fn role_names(
            &self,
        ) -> qtbridge::qt_type_lib::QHash<i32, qtbridge::qt_type_lib::QByteArray> {
            SomeStruct::role_names(self)
        }
        fn set_data(
            &mut self,
            index: &qtbridge::qt_type_lib::QModelIndex,
            value: &qtbridge::qt_type_lib::QVariant,
            role: i32,
        ) -> bool {
            SomeStruct::set_data(self, index, value, role)
        }
        fn remove_rows(
            &mut self,
            first: i32,
            count: i32,
            parent: &qtbridge::qt_type_lib::QModelIndex,
        ) -> bool {
            SomeStruct::remove_rows(self, first, count, parent)
        }
        fn sibling(
            &self,
            row: i32,
            column: i32,
            idx: &qtbridge::qt_type_lib::QModelIndex,
        ) -> qtbridge::qt_type_lib::QModelIndex {
            SomeStruct::sibling(self, row, column, idx)
        }
    }
    impl SomeStruct {
        pub fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
            let instance = std::rc::Rc::new(std::cell::RefCell::new(Self::default()));
            Self::attach_qobject(&instance);
            instance
        }
        pub fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
            <Self as qtbridge::qt_traits::QObjectHolder>::register_instance_in_map(
                instance.clone(),
                false,
            );
            <Self as qtbridge::qt_traits::QObjectHolder>::set_dynamic_meta(instance);
        }
        pub fn detach_qobject(&self) {
            if let Some(qobj) = <Self as qtbridge::qt_traits::QObjectHolder>::try_get_qobject(self)
            {
                qtbridge::qt_type_lib::QObject::delete(std::ptr::from_mut(qobj));
            }
        }
        pub fn get_qobject(&self) -> &mut qtbridge::qt_type_lib::QObject {
            <Self as qtbridge::qt_traits::QObjectHolder>::get_qobject(self)
        }
        pub fn as_qvariant(&self) -> qtbridge::qt_type_lib::QVariant {
            let qobj_ref = <Self as qtbridge::qt_traits::QObjectHolder>::get_qobject(self);
            let qobj_ptr = std::ptr::from_mut(qobj_ref);
            qobj_ptr.into()
        }
    }
    impl Drop for SomeStruct {
        fn drop(&mut self) {
            self.detach_qobject();
        }
    }
    impl qtbridge::bridge::QMetaInfo for SomeStruct {
        fn class_name() -> &'static str {
            ::std::any::type_name::<SomeStruct>()
        }
        fn get_static_meta_object() -> &'static qtbridge::qt_type_lib::QMetaObject {
            <Self as qtbridge::qt_traits::QObjectHolder>::ProxyRust::get_static_meta_object()
        }
        fn register_meta(
            mut meta_obj: std::pin::Pin<&mut qtbridge::bridge::DynamicMetaObjectData>,
        ) {
            meta_obj.as_mut().end_meta_registration();
        }
        fn get_shared_dynamic_meta_object_data(
        ) -> &'static qtbridge::bridge::DynamicMetaObjectData {
            use std::any::TypeId;
            use std::cell::RefCell;
            use std::collections::HashMap;
            thread_local ! (static DYNAMIC_META_MAP : RefCell < HashMap < TypeId , * const qtbridge :: bridge :: DynamicMetaObjectData >> = RefCell :: new (HashMap :: new ()));
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
                qtbridge::bridge::create_dynamic_meta_object_data_for_type::<SomeStruct>();
            let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
            DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
                dynamic_meta_data_map.insert(type_id, meta_data_ptr);
            });
            meta_data_ref
        }
        fn get_list_meta_type() -> qtbridge::qt_type_lib::QMetaType {
            <Self as qtbridge::qt_traits::QObjectHolder>::ProxyRust::get_qmetatype_list_of_cpp_proxy(
            )
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
                let meta_obj_data = < SomeStruct as qtbridge :: bridge :: QMetaInfo > :: get_shared_dynamic_meta_object_data () ;
                meta_obj_data.get_dynamic_qmetaobject().cast_mut()
            }
            pub extern "C" fn default_ctor(
                _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                addr: *mut u8,
            ) {
                let instance =
                    std::rc::Rc::new(std::cell::RefCell::new(<SomeStruct as Default>::default()));
                < SomeStruct as qtbridge :: qt_traits :: QObjectHolder > :: register_instance_in_map_with_cpp_proxy_at (addr , instance) ;
            }
            pub extern "C" fn dtor(
                _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                obj: *mut u8,
            ) {
                qtbridge::qt_type_lib::QObject::destruct(obj.cast());
            }
            let iface = qtbridge::qt_type_lib::QMetaTypeInterface::fill_fields(
                <Self as qtbridge::qt_traits::QObjectHolder>::ProxyRust::get_align_of_cpp_proxy(),
                <Self as qtbridge::qt_traits::QObjectHolder>::ProxyRust::get_size_of_cpp_proxy(),
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
    impl qtbridge::qt_traits::QObjectHolder for SomeStruct {
        type ProxyRust = qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust;
        fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
        where
            F: FnOnce(&mut std::collections::HashMap<*const u8, *const Self::ProxyRust>) -> R,
        {
            Self::try_borrow_mut_proxies_map_impl(f)
        }
        fn try_get_qobject(&self) -> Option<&mut qtbridge::qt_type_lib::QObject> {
            let rust_proxy = Self::try_get_rust_proxy_mut(&self)?;
            let cpp_proxy = rust_proxy.get_cpp_proxy();
            let qobject_ptr: *const qtbridge::qt_type_lib::QObject = cpp_proxy.cast();
            unsafe { qobject_ptr.cast_mut().as_mut() }
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
                    RefCell<dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModel>,
                > = rust_obj_rc;
                let proxy_ptr = Self::ProxyRust::new(
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
                    RefCell<dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModel>,
                > = rust_obj_rc;
                let proxy_ptr = Self::ProxyRust::new_with_cpp_proxy_at(
                    addr,
                    &dyn_rc,
                    Self::unregister_instance_in_map,
                );
                proxies.insert(key, proxy_ptr);
            })
        }
    }
}
