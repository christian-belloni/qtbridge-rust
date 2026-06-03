// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QMetaType;

#[qt_gen::bridge]
mod qmetatypeinterface {
    include_in_cpp!(<QMetaType>);
    include_in_cpp!(<QObject>);
    include_in_cpp!(<QtQml/QQmlListProperty>);

    #[doc(hidden)]
    #[namespace = "QtPrivate"]
    /// If a QMetaType is generated based on QMetaTypeInterface it must have a
    /// constant address in memory during the whole application run.
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
        meta_obj_fn: usize, default_ctr_fn: usize, copy_ctr_fn: usize, dtor_fn: usize,
    ) -> QMetaTypeInterface {

        let cpp = cpp_fn!(|align: u16, size: u32, flags: u32, name: &[u8],
                meta_obj_fn: usize, default_ctr_fn: usize, copy_ctr_fn: usize, dtor_fn: usize
                | -> QMetaTypeInterface
            {
                auto metaObjFn = reinterpret_cast<QMetaTypeInterface::MetaObjectFn>(meta_obj_fn);
                auto defaultCtr = reinterpret_cast<QMetaTypeInterface::DefaultCtrFn>(default_ctr_fn);
                auto copyCtr = reinterpret_cast<QMetaTypeInterface::CopyCtrFn>(copy_ctr_fn);
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
                    /* copyCtr */ copyCtr,
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
        cpp(align as u16, size as u32, flags, name, meta_obj_fn, default_ctr_fn, copy_ctr_fn, dtor_fn)
    }

    /// The list metatype interface for an element type: a clone of `QQmlListProperty<QObject>`
    /// (layout-identical to any `QQmlListProperty<T>`), named `QQmlListProperty<{element.name}>`
    /// with the cached typeId reset to 0 so it registers as a fresh, distinct type.
    ///
    /// The name is derived from `element`'s own registered name (so the list and element types
    /// can't drift apart) and leaked in C++.
    pub fn qqml_list_property_for(element: &QMetaType) -> QMetaTypeInterface {
        cpp_fn!(|element: &QMetaType| -> QMetaTypeInterface {
            const QtPrivate::QMetaTypeInterface* base =
                QMetaType::fromType<QQmlListProperty<QObject>>().iface();
            auto* name = new QByteArray(
                QByteArrayLiteral("QQmlListProperty<") + element.name() + '>');
            return QMetaTypeInterface {
                base->revision,
                base->alignment,
                base->size,
                base->flags,
                { 0 },
                base->metaObjectFn,
                name->constData(),
                base->defaultCtr,
                base->copyCtr,
                base->moveCtr,
                base->dtor,
                base->equals,
                base->lessThan,
                base->debugStream,
                base->dataStreamOut,
                base->dataStreamIn,
                base->legacyRegisterOp
            };
        })(element)
    }

}
