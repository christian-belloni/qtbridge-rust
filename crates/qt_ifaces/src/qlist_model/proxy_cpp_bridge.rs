// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QListModelProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qt_type_lib/src/generated/core/qhash/cpp/qhash_i32_qbytearray.h");
        type QHash_i32_QByteArray = qt_type_lib::QHash_i32_QByteArray;
        include!("qt_type_lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qt_type_lib::QMetaObject;
        include!("qt_type_lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qt_type_lib::QMetaType;
        include!("qt_type_lib/src/generated/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qt_type_lib::QModelIndex;
        include!("qt_type_lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qt_type_lib::QVariant;
        include!("qt_ifaces/src/qlist_model/proxy_rust_bridge.rs.h");
        type QListModelProxyRust = super::QListModelProxyRust;
    }
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qt_ifaces/src/qlist_model/cpp/QListModelProxyCpp.h");
        type QListModelProxyCpp;
        # [rust_name = create_qlist_model_proxy_cpp]
        unsafe fn create_QListModelProxyCpp(rust_obj: *mut u8, rust_proxy: *mut QListModelProxyRust) -> *mut QListModelProxyCpp;
        # [rust_name = create_qlist_model_proxy_cpp_at]
        unsafe fn create_QListModelProxyCpp_At(addr: *mut u8, rust_obj: *mut u8, rust_proxy: *mut QListModelProxyRust)
        -> *mut QListModelProxyCpp;
        # [rust_name = static_qmeta_object_of_qlist_model_proxy_cpp]
        fn staticQMetaObjectOf_QListModelProxyCpp() -> &'static QMetaObject;
        # [rust_name = size_of_qlist_model_proxy_cpp]
        fn sizeOf_QListModelProxyCpp() -> usize;
        # [rust_name = align_of_qlist_model_proxy_cpp]
        fn alignOf_QListModelProxyCpp() -> usize;
        # [rust_name = qmetatype_list_of_qlist_model_proxy_cpp]
        fn qmetaTypeListOf_QListModelProxyCpp() -> QMetaType;
        # [rust_name = base_index]
        fn base_index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
        # [rust_name = base_role_names]
        fn base_roleNames(&self) -> QHash_i32_QByteArray;
        # [rust_name = base_set_data]
        fn base_setData(self: Pin<&mut Self>, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
        # [rust_name = base_remove_rows]
        fn base_removeRows(self: Pin<&mut Self>, first: i32, count: i32, parent: &QModelIndex) -> bool;
        # [rust_name = base_sibling]
        fn base_sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
        # [rust_name = base_data_changed]
        fn dataChanged(self: Pin<&mut Self>, top_left: &QModelIndex, bottom_right: &QModelIndex);
        # [rust_name = base_begin_insert_rows]
        fn beginInsertRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_insert_rows]
        fn endInsertRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_move_rows]
        fn beginMoveRows(
            self: Pin<&mut Self>,
            source_parent: &QModelIndex,
            source_first: i32,
            source_last: i32,
            destination_parent: &QModelIndex,
            destination_child: i32,
        );
        # [rust_name = base_end_move_rows]
        fn endMoveRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_remove_rows]
        fn beginRemoveRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_remove_rows]
        fn endRemoveRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_reset_model]
        fn beginResetModel(self: Pin<&mut Self>);
        # [rust_name = base_end_reset_model]
        fn endResetModel(self: Pin<&mut Self>);
    }
}
pub use ffi::QListModelProxyCpp;
