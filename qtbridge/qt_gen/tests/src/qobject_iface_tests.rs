// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use proc_macro2::TokenStream;
use qt_gen_impl::QObjectModuleBuilder;
use quote::{ToTokens, quote};
use crate::tst_assert::assert_tokens_eq;
use qt_gen_common::type_qualified_mapping::CallOrigin;

#[test]
pub fn require_that_qobject_macro_generates_interface_impl_code_that_agrees_with_reference() {
    let input = quote! {
        mod some_module {
            #[derive(Default)]
            struct SomeStruct {
            }

            impl SomeStruct {
                #[overridden]
                fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32 ) -> bool {
                    false
                }

                #[overridden(cpp_name="data")]
                fn d_data_(&self, index: &QModelIndex, role: i32) -> QVariant {
                    //QVariant::default()
                }

                #[overridden]
                fn row_count(&self, parent: &QModelIndex) -> i32 {
                    1
                }
            }
        }
    };

    let input_params = quote!{
        Base = QAbstractListModel
    };

    let expected_struct = quote!{
        #[derive(Default)]
        struct SomeStruct {
        }
    };

    let expected_new_impl = quote!{
        impl SomeStruct {
            fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
                false
            }
            fn d_data_(&self, index: &QModelIndex, role: i32) -> QVariant {}
            fn row_count(&self, parent: &QModelIndex) -> i32 {
                1
            }
        }
    };

    let expected_iface_base_impl = quote!{
        /// Impl block with base functions
        impl SomeStruct {
            fn index(
                &self,
                row: i32,
                column: i32,
                parent: &qtbridge::qt_type_lib::QModelIndex,
            ) -> qtbridge::qt_type_lib::QModelIndex {
                let proxy = some_struct_impl_details::get_rust_proxy(self);
                proxy.base_index(row, column, parent)
            }
            fn role_names(&self) -> qtbridge::qt_type_lib::QHash<i32, qtbridge::qt_type_lib::QByteArray> {
                let proxy = some_struct_impl_details::get_rust_proxy(self);
                proxy.base_role_names()
            }
            fn base_set_data(
                &mut self,
                index: &qtbridge::qt_type_lib::QModelIndex,
                value: &qtbridge::qt_type_lib::QVariant,
                role: i32,
            ) -> bool {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_set_data(index, value, role)
            }
            fn remove_rows(
                &mut self,
                first: i32,
                count: i32,
                parent: &qtbridge::qt_type_lib::QModelIndex,
            ) -> bool {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_remove_rows(first, count, parent)
            }
            fn sibling(
                &self,
                row: i32,
                column: i32,
                idx: &qtbridge::qt_type_lib::QModelIndex,
            ) -> qtbridge::qt_type_lib::QModelIndex {
                    let proxy = some_struct_impl_details::get_rust_proxy(self);
                proxy.base_sibling(row, column, idx)
            }
            fn data_changed(
                &mut self,
                topLeft: &qtbridge::qt_type_lib::QModelIndex,
                bottomRight: &qtbridge::qt_type_lib::QModelIndex,
            ) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_data_changed(topLeft, bottomRight)
            }
            fn begin_insert_rows(&mut self, parent: &qtbridge::qt_type_lib::QModelIndex, first: i32, last: i32) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_begin_insert_rows(parent, first, last)
            }
            fn end_insert_rows(&mut self) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
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
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_begin_move_rows(
                    sourceParent,
                    sourceFirst,
                    sourceLast,
                    destinationParent,
                    destinationChild,
                )
            }
            fn end_move_rows(&mut self) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_end_move_rows()
            }
            fn begin_remove_rows(
                &mut self,
                parent: &qtbridge::qt_type_lib::QModelIndex,
                first: i32,
                last: i32,
            ) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_begin_remove_rows(parent, first, last)
            }
            fn end_remove_rows(&mut self) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_end_remove_rows()
            }
            fn begin_reset_model(&mut self) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_begin_reset_model()
            }
            fn end_reset_model(&mut self) {
                let proxy = some_struct_impl_details::get_rust_proxy_mut(self);
                proxy.base_end_reset_model()
            }
        }
    };

    let expected_iface_trait = quote! {
        /// Rust implementation of C++ interface methods
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
            fn data(&self, index: &qtbridge::qt_type_lib::QModelIndex, role: i32) -> qtbridge::qt_type_lib::QVariant {
                SomeStruct::d_data_(self, index, role)
            }
            fn role_names(&self) -> qtbridge::qt_type_lib::QHash<i32, qtbridge::qt_type_lib::QByteArray> {
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
    };

    let expected_qobject_funcs = quote! {
        /// Impl block with functions needed to attach, detach and reference QObject corresponding to given Rust object
        impl SomeStruct {
            /// Create a new default-initialized object of this type with a `QObject` already attached.
            /// The object must remain at its original heap location and must not be moved out of `Rc<RefCell<T>>`.
            /// TODO: rename it so that 'qobject' is not exposed to the user.
            /// TODO: or attach a qobject on demand/when sending the object to QML engine?.
            pub fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
                let instance = std::rc::Rc::new(std::cell::RefCell::new(Self::default()));
                Self::attach_qobject(&instance);
                instance
            }
            /// Attach a dedicated `QObject` to the Rust object given as an argument.
            /// Rust object must remain at its original heap location and must not be moved out of `Rc<RefCell<T>>`.
            /// TODO: rename it so that 'qobject' is not exposed to the user.
            /// TODO: or attach a qobject on demand/when sending the object to QML engine?.
            pub fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
                some_struct_impl_details::register_instance_in_map(instance.clone(), false);
                some_struct_impl_details::set_dynamic_meta(instance);
            }
            /// Detach and remove the dedicated `QObject` from the specified object.
            /// This function is intended to be called during the `Drop` of this type.
            /// TODO: Rename it so that 'qobject' is not exposed to the user.
            /// TODO: Document somewhere (in the documentation of #[qobject_impl]?) that this function must be called from the `Drop` implementation of the user-defined type.
            pub fn detach_qobject(&self) {
                if let Some(qobj) = some_struct_impl_details::try_get_qobject(self) {
                    qtbridge::qt_type_lib::QObject::delete(std::ptr::from_mut(qobj));
                }
            }
            /// Return a reference to the QObject attached to 'self'.
            pub fn get_qobject(&self) -> &mut qtbridge::qt_type_lib::QObject {
                some_struct_impl_details::get_qobject(self)
            }
            /// Return a QVariant containing a pointer to the QObject proxy
            /// corresponding to 'self'.
            pub fn as_qvariant(&self) -> qtbridge::qt_type_lib::QVariant {
                let qobj_ref = some_struct_impl_details::get_qobject(self);
                let qobj_ptr = std::ptr::from_mut(qobj_ref);
                qobj_ptr.into()
            }
        }
    };

    let expected_qmeta_info_impl = quote! {
        impl qtbridge::bridge::QMetaInfo for SomeStruct {
            fn class_name() -> &'static str {
                ::std::any::type_name::<SomeStruct>()
            }
            fn get_static_meta_object() -> &'static qtbridge::qt_type_lib::QMetaObject {
                some_struct_impl_details::ProxyRust::get_static_meta_object()
            }
            fn register_meta(mut meta_obj: std::pin::Pin<&mut qtbridge::bridge::DynamicMetaObjectData_Rust>) {
                meta_obj.as_mut().end_meta_registration();
            }
            fn get_shared_dynamic_meta_object_data() -> &'static qtbridge::bridge::DynamicMetaObjectData_Rust {
                use std::any::TypeId;
                use std::cell::RefCell;
                use std::collections::HashMap;
                thread_local! (static DYNAMIC_META_MAP : RefCell < HashMap < TypeId , * const qtbridge :: bridge :: DynamicMetaObjectData_Rust >> = RefCell :: new (HashMap :: new ()));
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
                let meta_data_ptr = qtbridge::bridge::create_dynamic_meta_object_data_for_type::<SomeStruct>();
                let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
                DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
                    dynamic_meta_data_map.insert(type_id, meta_data_ptr);
                });
                meta_data_ref
            }
            fn get_list_meta_type() -> qtbridge::qt_type_lib::QMetaType {
                some_struct_impl_details::ProxyRust::get_qmetatype_list_of_cpp_proxy()
            }
        }
    };

    let expected_qmetatype_iface_get_impl = quote! {
        impl qtbridge::qt_type_lib::QMetaTypeInterfaceGet for SomeStruct {
            fn get_qmetatype_interface() -> &'static qtbridge::qt_type_lib::QMetaTypeInterface {
                use qtbridge::qt_type_lib::{QMetaTypeFlag, QMetaTypeInterface};
                use std::any::TypeId;
                use std::cell::RefCell;
                use std::collections::HashMap;
                thread_local! (static IFACE_MAP : RefCell < HashMap < TypeId , * const QMetaTypeInterface >> = RefCell :: new (HashMap :: new ()));
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
                ) -> *mut qtbridge::qt_type_lib::QMetaObject
                {
                    let meta_obj_data =
                        <SomeStruct as qtbridge::bridge::QMetaInfo>::get_shared_dynamic_meta_object_data();
                    meta_obj_data.get_dynamic_qmetaobject().cast_mut()
                }
                pub extern "C" fn default_ctor(
                    _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                    addr: *mut u8,
                ) {
                    let instance =
                        std::rc::Rc::new(std::cell::RefCell::new(<SomeStruct as Default>::default()));
                    some_struct_impl_details::register_instance_in_map_with_cpp_proxy_at(addr, instance);
                }
                pub extern "C" fn dtor(
                    _iface: *const qtbridge::qt_type_lib::QMetaTypeInterface,
                    obj: *mut u8,
                ) {
                    qtbridge::qt_type_lib::QObject::destruct(obj.cast());
                }
                let iface = qtbridge::qt_type_lib::QMetaTypeInterface::fill_fields(
                    some_struct_impl_details::ProxyRust::get_align_of_cpp_proxy(),
                    some_struct_impl_details::ProxyRust::get_size_of_cpp_proxy(),
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
    };

    let expected_impl_details = quote! {
        /// Functionality called from implementation internals that makes sense to place in a separate module
        /// rather than overwhelm impl block of struct or add yet another trait impl.
        mod some_struct_impl_details {
            use std::cell::{BorrowError, BorrowMutError, RefCell};
            use std::rc::Rc;
            /// Alias for the user-defined struct annotated with `#[qobject]` for which this module is generated.
            type RustObj = super::SomeStruct;
            /// Alias for the Rust proxy type corresponding to the user-defined type.
            /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
            /// forwarding calls in both directions and managing borrowing of the Rust object
            /// during QAIM calls (and TBD for meta calls as well).
            pub(super) type ProxyRust = qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModelProxyRust;
            /// Type alias for a map containing all instances of the given user-defined type.
            type ProxiesMap = std::collections::HashMap<*const u8, *const ProxyRust>;
            /// Shared map containing all registered instances of given user-defined type (multiton).
            thread_local ! (static SOMESTRUCT_PROXY_INSTANCES : std :: cell :: RefCell < ProxiesMap > = std :: cell :: RefCell :: new (ProxiesMap :: default ()));
            /// Invoke the provided function if immutable borrowing succeeds.
            fn try_borrow_proxies_map<F, R>(f: F) -> R
            where
                F: FnOnce(&ProxiesMap) -> R,
            {
                SOMESTRUCT_PROXY_INSTANCES
                    .try_with(|proxies_map_cell| -> Result<R, BorrowError> {
                        let proxies_map_ref = proxies_map_cell.try_borrow()?;
                        Ok(f(&proxies_map_ref))
                    })
                    .unwrap()
                    .expect("Failed to borrow map of proxies")
            }
            /// Invoke the provided function if mutable borrowing succeeds.
            fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
            where
                F: FnOnce(&mut ProxiesMap) -> R,
            {
                SOMESTRUCT_PROXY_INSTANCES
                    .try_with(|proxies_map_cell| -> Result<R, BorrowMutError> {
                        let mut proxies_map_ref_mut = proxies_map_cell.try_borrow_mut()?;
                        Ok(f(&mut proxies_map_ref_mut))
                    })
                    .unwrap()
                    .expect("Failed to borrow_mut map of proxies")
            }
            /// Return an immutable reference to the Rust proxy linked to the Rust object specified in the argument.
            pub(super) fn get_rust_proxy(rust_obj_ref: &RustObj) -> &ProxyRust {
                get_rust_proxy_mut(rust_obj_ref)
            }
            /// Return a mutable reference to the Rust proxy linked to the Rust object specified in the argument.
            pub(super) fn get_rust_proxy_mut(rust_obj_ref: &RustObj) -> &mut ProxyRust {
                try_get_rust_proxy_mut(rust_obj_ref).expect("No proxy registered for given rust object")
            }
            /// Return a Result wrapping mutable reference to the Rust proxy associated with the specified object.
            pub(super) fn try_get_rust_proxy_mut(rust_obj_ref: &RustObj) -> Option<&mut ProxyRust> {
                let ptr = try_borrow_mut_proxies_map(|proxies| {
                    let rust_obj_ptr = std::ptr::from_ref(rust_obj_ref).cast();
                    match proxies.get(&rust_obj_ptr) {
                        Some(ptr) => ptr.cast_mut(),
                        None => std::ptr::null_mut(),
                    }
                });
                unsafe { ptr.as_mut() }
            }
            /// Return Result with QObject linked to the Rust object provided as an argument.
            pub(crate) fn try_get_qobject(this: &RustObj) -> Option<&mut qtbridge::qt_type_lib::QObject> {
                let rust_proxy = try_get_rust_proxy_mut(this)?;
                let cpp_proxy = rust_proxy.get_cpp_proxy();
                let qobject_ptr: *const qtbridge::qt_type_lib::QObject = cpp_proxy.cast();
                unsafe { qobject_ptr.cast_mut().as_mut() }
            }
            /// Return QObject linked to the Rust object provided as an argument.
            pub(crate) fn get_qobject(this: &RustObj) -> &mut qtbridge::qt_type_lib::QObject {
                try_get_qobject(this).expect("QObject is not attached")
            }
            /// Register the given Rust object instance in the multiton.
            /// Create Rust and C++ proxies and links them to the Rust object.
            pub(crate) fn register_instance_in_map(
                rust_obj_rc: Rc<RefCell<RustObj>>,
                register_strong: bool,
            ) {
                let key = (*rust_obj_rc).as_ptr() as *const u8;
                try_borrow_mut_proxies_map(|proxies| {
                    let dyn_rc: Rc<RefCell<dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModel>> =
                        rust_obj_rc;
                    let proxy_ptr = ProxyRust::new(&dyn_rc, register_strong, unregister_instance_in_map);
                    proxies.insert(key, proxy_ptr);
                })
            }
            /// Register the given Rust object instance in the multiton.
            /// Create Rust and C++ proxies and links them to the Rust object.
            /// C++ proxy created using placement new operator at the memory address provided as the first argument.
            pub(super) fn register_instance_in_map_with_cpp_proxy_at(
                addr: *mut u8,
                rust_obj_rc: Rc<RefCell<RustObj>>,
            ) {
                let key = (*rust_obj_rc).as_ptr() as *const u8;
                try_borrow_mut_proxies_map(|proxies| {
                    let dyn_rc: Rc<RefCell<dyn qtbridge::qt_ifaces::qabstract_list_model::QAbstractListModel>> =
                        rust_obj_rc;
                    let proxy_ptr =
                        ProxyRust::new_with_cpp_proxy_at(addr, &dyn_rc, unregister_instance_in_map);
                    proxies.insert(key, proxy_ptr);
                })
            }
            /// Removes the entry associated with the specified Rust object from the multiton map.
            pub(super) fn unregister_instance_in_map(rust_obj_ptr: *const u8) {
                try_borrow_mut_proxies_map(|proxies| proxies.remove(&rust_obj_ptr))
                    .expect("Proxy object for rust object is not registered")
                    .cast_mut();
            }
            /// Configure the QObject associated with the given Rust object to use
            /// the dynamic metaobject specific to this Rust type.
            pub(crate) fn set_dynamic_meta(instance: &Rc<RefCell<RustObj>>) {
                let dynamic_meta = <RustObj as qtbridge::bridge::QMetaInfo>::get_shared_dynamic_meta_object_data();
                let instance_ref = &instance.borrow();
                let qobject_ref = get_qobject(instance_ref);
                dynamic_meta.set_to_qobject(qobject_ref);
            }
        }
    };

    let mut builder = QObjectModuleBuilder::new(CallOrigin::External);
    let output = builder.build(input, input_params).unwrap();
    let items: Vec<TokenStream> = output.content.unwrap().1.iter()
        .map(ToTokens::to_token_stream)
        .collect();
    let new_struct = &items[0];
    let new_impl = &items[1];
    let iface_base_impl = &items[2];
    let iface_trait = &items[3];
    let qobject_funcs = &items[4];
    let qmeta_info_impl = &items[5];
    let qmetatype_iface_get_impl = &items[6];
    let impl_details = &items[7];
    assert_tokens_eq(new_struct, &expected_struct);
    assert_tokens_eq(new_impl, &expected_new_impl);
    assert_tokens_eq(iface_base_impl, &expected_iface_base_impl);
    assert_tokens_eq(iface_trait, &expected_iface_trait);
    assert_tokens_eq(qobject_funcs, &expected_qobject_funcs);
    assert_tokens_eq(qmeta_info_impl, &expected_qmeta_info_impl);
    assert_tokens_eq(qmetatype_iface_get_impl, &expected_qmetatype_iface_get_impl);
    assert_tokens_eq(impl_details, &expected_impl_details);
}
