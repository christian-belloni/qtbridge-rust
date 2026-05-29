// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::Once;
use linkme::distributed_slice;
use crate::{QMetaObject, QMetaType};

// Storage for callbacks performing QML registration of user defined types
#[doc(hidden)]
#[distributed_slice]
pub static QML_REGISTER_CALLBACKS: [fn()];

#[doc(hidden)]
pub fn call_qml_register_callbacks() {
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        for reg_fn in QML_REGISTER_CALLBACKS {
            reg_fn();
        }
    });
}

#[qt_gen::bridge]
mod qmlprivate {
    include_in_cpp!(<qqmlprivate.h>);
    include_in_cpp!("rustconv.h");

    #[doc(hidden)]
    pub fn qml_register_element(type_id: QMetaType, list_id: QMetaType, object_size: u32, parser_status_cast: i32,
                        create_fn: usize, uri: &[u8], version_major: u8, version_minor: u8,
                        elm_name: &[u8], meta_object: &'static QMetaObject) {

            let cpp = cpp_fn!(|
                    type_id: QMetaType, list_id: QMetaType, object_size: u32, parser_status_cast: i32,
                    create_fn: usize, uri: &[u8], version_major: u8, version_minor: u8,
                    elm_name: &[u8], meta_object: &QMetaObject,
                |
            {
                const QByteArray uriBa = RustByteSliceToQByteArray(uri);
                const QByteArray elmNameBa = RustByteSliceToQByteArray(elm_name);

                QQmlPrivate::RegisterType rt = {};
                rt.structVersion = QQmlPrivate::RegisterType::CurrentVersion;
                rt.typeId = type_id;
                rt.listId = list_id;
                rt.objectSize = object_size;
                rt.create = reinterpret_cast<void(*)(void*, void*)>(create_fn);
                rt.userdata = nullptr;
                rt.noCreationReason = QString();
                rt.createValueType = nullptr;
                rt.uri = uriBa.data();
                rt.version = QTypeRevision::fromVersion(version_major, version_minor);
                rt.elementName = elmNameBa;
                rt.metaObject = &meta_object;
                rt.attachedPropertiesFunction = nullptr;
                rt.attachedPropertiesMetaObject = nullptr;
                rt.parserStatusCast = parser_status_cast;
                rt.valueSourceCast = -1;
                rt.valueInterceptorCast = -1;
                rt.extensionObjectCreate = nullptr;
                rt.extensionMetaObject = nullptr;
                rt.customParser = nullptr;
                rt.revision = QTypeRevision::fromVersion(0, 0);
                rt.finalizerCast = -1;
                rt.creationMethod = QQmlPrivate::ValueTypeCreationMethod::None;

                QQmlPrivate::qmlregister(QQmlPrivate::TypeRegistration, &rt);
            });
            cpp(type_id, list_id, object_size, parser_status_cast, create_fn, uri, version_major, version_minor, elm_name, meta_object)
    }

    #[doc(hidden)]
    pub fn qml_register_singleton(
        type_id: QMetaType,
        create_fn: usize,
        uri: &[u8],
        version_major: u8,
        version_minor: u8,
        elm_name: &[u8],
        meta_object: &'static QMetaObject,
    ) {
        let cpp = cpp_fn!(|
                type_id: QMetaType, create_fn: usize,
                uri: &[u8], version_major: u8, version_minor: u8,
                elm_name: &[u8], meta_object: &QMetaObject,
            |
        {
            auto createQmlSingletonType = [create_fn](QQmlEngine *, QJSEngine *) -> QObject* {
                auto ctr = reinterpret_cast<QObject* (*)()>(create_fn);
                return ctr();
            };

            const QByteArray uriBa = RustByteSliceToQByteArray(uri);
            const QByteArray elmNameBa = RustByteSliceToQByteArray(elm_name);
            QQmlPrivate::RegisterSingletonType api = {};
            api.structVersion = 0;
            api.uri = uriBa.data();
            api.version = QTypeRevision::fromVersion(version_major, version_minor);
            api.typeName = elmNameBa;
            api.scriptApi = nullptr;
            api.qObjectApi = createQmlSingletonType;
            api.instanceMetaObject = &meta_object;
            api.typeId = type_id;
            api.extensionObjectCreate = nullptr;
            api.extensionMetaObject = nullptr;
            api.revision = QTypeRevision::fromVersion(0, 0);

            QQmlPrivate::qmlregister(QQmlPrivate::SingletonRegistration, &api);
        });
        cpp(type_id, create_fn, uri, version_major, version_minor, elm_name, meta_object)
    }
}
