// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;

#[qt_gen::bridge]
mod qmetatypeinterface {
    include_in_cpp!(<QMetaType>);

    #[doc(hidden)]
    #[namespace = "QtPrivate"]
    struct QMetaTypeInterface {
        _revision:           MaybeUninit<u16>,
        _alignment:          MaybeUninit<u16>,
        _size:               MaybeUninit<u32>,
        _flags:              MaybeUninit<u32>,
        _type_id:            MaybeUninit<i32>,   // QBasicAtomicInt
        _meta_object_fn:     MaybeUninit<usize>, // MetaObjectFn = const QMetaObject *(*)(const QMetaTypeInterface *)
        _name:               MaybeUninit<usize>, // const char *
        _default_ctr:        MaybeUninit<usize>, // DefaultCtrFn = void (*)(const QMetaTypeInterface *, void *)
        _copy_ctr:           MaybeUninit<usize>, // CopyCtrFn = void (*)(const QMetaTypeInterface *, void *, const void *)
        _move_ctr:           MaybeUninit<usize>, // MoveCtrFn = void (*)(const QMetaTypeInterface *, void *, void *)
        _dtor:               MaybeUninit<usize>, // DtorFn = void (*)(const QMetaTypeInterface *, void *)
        _equals:             MaybeUninit<usize>, // EqualsFn = bool (*)(const QMetaTypeInterface *, const void *, const void *)
        _less_than:          MaybeUninit<usize>, // LessThanFn = bool (*)(const QMetaTypeInterface *, const void *, const void *)
        _debug_stream:       MaybeUninit<usize>, // DebugStreamFn = void (*)(const QMetaTypeInterface *, QDebug &, const void *)
        _data_stream_out:    MaybeUninit<usize>, // DataStreamOutFn = void (*)(const QMetaTypeInterface *, QDataStream &, const void *)
        _data_stream_in:     MaybeUninit<usize>, // DataStreamInFn = void (*)(const QMetaTypeInterface *, QDataStream &, void *)
        _legacy_register_op: MaybeUninit<usize>, // LegacyRegisterOp = void (*)()
    }

    pub fn fill_fields(
        align: usize, size: usize, flags: u32, name: &[u8],
        meta_obj_fn: usize, default_ctr_fn: usize, dtor_fn: usize,
    ) -> QMetaTypeInterface {

        let cpp = cpp_fn!(|align: u16, size: u32, flags: u32, name: &[u8],
                meta_obj_fn: usize, default_ctr_fn: usize, dtor_fn: usize
                | -> QMetaTypeInterface
            {
                auto metaObjFn = reinterpret_cast<QMetaTypeInterface::MetaObjectFn>(meta_obj_fn);
                auto defaultCtr = reinterpret_cast<QMetaTypeInterface::DefaultCtrFn>(default_ctr_fn);
                auto dtor = reinterpret_cast<QMetaTypeInterface::DtorFn>(dtor_fn);

                // Initialize value at return to enable copy elision
                // and avoid compilation error due to deleted copy constructor
                return QMetaTypeInterface {
                    /* revision */  QMetaTypeInterface::CurrentRevision,
                    /* alignment */ align,
                    /* size */ size,
                    /* flags */ flags,
                    /* typeId */ { 0 },
                    /* metaObjectFn */ metaObjFn,
                    /* name */ reinterpret_cast<const char*>(name.data()),
                    /* defaultCtr */ defaultCtr,
                    /* copyCtr */ nullptr,
                    /* moveCtr */ nullptr,
                    /* dtor */ dtor,
                    /* equals */ nullptr,
                    /* lessThan */ nullptr,
                    /* debugStream */ nullptr,
                    /* dataStreamOut */ nullptr,
                    /* dataStreamIn */ nullptr,
                    /* legacyRegisterOp */ nullptr
                };
            });
        cpp(align as u16, size as u32, flags, name, meta_obj_fn, default_ctr_fn, dtor_fn)
    }
}

#[doc(hidden)]
pub trait QMetaTypeInterfaceGet {
    /// Return reference to filled struct QMetaTypeInterface
    /// that has constant address in memory during the whole application run
    fn get_qmetatype_interface() -> &'static QMetaTypeInterface;
}
