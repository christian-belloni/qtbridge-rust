// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QAbstractItemModelProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qhash/cpp/qhash_i32_qbytearray.h");
        type QHash_i32_QByteArray = qtbridge_type_lib::QHash_i32_QByteArray;
        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qtbridge_type_lib::QModelIndex;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
        include!("qtbridge-interfaces/src/generated/qabstract_item_model/proxy_rust_bridge.rs.h");
        type QAbstractItemModelProxyRust = super::QAbstractItemModelProxyRust;
    }
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qtbridge-interfaces/src/generated/qabstract_item_model/cpp/QAbstractItemModelProxyCpp.h");
        type QAbstractItemModelProxyCpp;
        # [rust_name = create_qabstract_item_model_proxy_cpp]
        unsafe fn create_QAbstractItemModelProxyCpp(rust_proxy: *mut QAbstractItemModelProxyRust) -> *mut QAbstractItemModelProxyCpp;
        # [rust_name = create_qabstract_item_model_proxy_cpp_at]
        unsafe fn create_QAbstractItemModelProxyCpp_At(addr: *mut u8, rust_proxy: *mut QAbstractItemModelProxyRust)
        -> *mut QAbstractItemModelProxyCpp;
        # [rust_name = static_qmeta_object_of_qabstract_item_model_proxy_cpp]
        fn staticQMetaObjectOf_QAbstractItemModelProxyCpp() -> &'static QMetaObject;
        # [rust_name = size_of_qabstract_item_model_proxy_cpp]
        fn sizeOf_QAbstractItemModelProxyCpp() -> usize;
        # [rust_name = align_of_qabstract_item_model_proxy_cpp]
        fn alignOf_QAbstractItemModelProxyCpp() -> usize;
        # [rust_name = qmetatype_list_of_qabstract_item_model_proxy_cpp]
        fn qmetaTypeListOf_QAbstractItemModelProxyCpp() -> QMetaType;
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
        # [rust_name = base_begin_insert_columns]
        fn beginInsertColumns(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_insert_columns]
        fn endInsertColumns(self: Pin<&mut Self>);
        # [rust_name = base_begin_insert_rows]
        fn beginInsertRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_insert_rows]
        fn endInsertRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_move_columns]
        fn beginMoveColumns(
            self: Pin<&mut Self>,
            source_parent: &QModelIndex,
            source_first: i32,
            source_last: i32,
            destination_parent: &QModelIndex,
            destination_child: i32,
        );
        # [rust_name = base_end_move_columns]
        fn endMoveColumns(self: Pin<&mut Self>);
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
        # [rust_name = base_begin_remove_columns]
        fn beginRemoveColumns(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_remove_columns]
        fn endRemoveColumns(self: Pin<&mut Self>);
        # [rust_name = base_begin_remove_rows]
        fn beginRemoveRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_remove_rows]
        fn endRemoveRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_reset_model]
        fn beginResetModel(self: Pin<&mut Self>);
        # [rust_name = base_end_reset_model]
        fn endResetModel(self: Pin<&mut Self>);
        # [rust_name = base_create_index]
        fn createIndex(&self, row: i32, column: i32, ptr: usize) -> QModelIndex;
    }
}
pub use ffi::QAbstractItemModelProxyCpp;
