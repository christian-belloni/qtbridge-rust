use std::mem::MaybeUninit;
use crate::{QMetaTypeInterface, QObject};

#[qt_gen::bridge]
mod qmetatype {
    include_in_cpp!(<QMetaType>);
    include_in_cpp!(<QObject>);
    include_in_cpp!("rustconv.h");

    #[doc(hidden)]
    pub enum QMetaTypeFlag {
        NeedsConstruction = 0x1,
        NeedsDestruction = 0x2,
        RelocatableType = 0x4,
        PointerToQObject = 0x8,
        IsEnumeration = 0x10,
        SharedPointerToQObject = 0x20,
        WeakPointerToQObject = 0x40,
        TrackingPointerToQObject = 0x80,
        IsUnsignedEnumeration = 0x100,
        IsGadget = 0x200,
        PointerToGadget = 0x400,
        IsPointer = 0x800,
        IsQmlList = 0x1000, // used in the QML engine to recognize QQmlListProperty<T> and list<T>
        IsConst = 0x2000,
        // since 6.5:
        NeedsCopyConstruction = 0x4000,
        NeedsMoveConstruction = 0x8000,
    }

    #[derive_cpp(Default, PartialEq)]
    #[derive(Debug)]
    /// The QMetaType struct manages named types in the meta-object system.
    ///
    /// See also: [QMetaType documentation](https://doc.qt.io/qt-6/qmetatype.html).
    struct QMetaType {
        _d_ptr: MaybeUninit<usize>, // QMetaTypeInterface*
    }

    /// Constructs a QMetaType object specified by its Id.
    pub fn new(type_id: i32) -> Self {
        cpp_fn!(|type_id: i32| -> Self {
            return QMetaType(type_id);
        })(type_id)
    }

    /// Creates a `QMetaType` instance from the specified `QMetaTypeInterface`.
    pub fn new_with_interface(iface: *const QMetaTypeInterface) -> Self {
        let cpp = cpp_fn!(|iface: *const QMetaTypeInterface| -> Self {
            return QMetaType(iface);
        });
        unsafe { cpp (iface) }
    }

    /// Returns id type held by this QMetaType instance.
    pub fn id(&self) -> i32 {
        cpp_fn!(|&self| -> i32 {
            return self.id();
        })(self)
    }

    /// Returns true if this QMetaType object contains valid information about a type, false otherwise.
    pub fn is_valid(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isValid();
        })(self)
    }

    /// Returns the type name associated with this QMetaType, or an empty string if type is not valid.
    pub fn name(&self) -> String {
        let cpp = cpp_fn!(|&self| -> String {
            return CStrToRustString(self.name());
        });
        cpp(self)
    }

    /// Registers this QMetaType with the type registry so it can be found by name, using QMetaType::fromName().
    pub fn register_type(&self) {
        cpp_fn!(|&self| {
            self.registerType();
        })(self)
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize, *mut QObject]]
    impl<T> QMetaTypeGet for T {
        fn get_qmetatype() -> QMetaType {
            cpp_fn!(|| -> QMetaType {
                return QMetaType::fromType<T>();
            })()
        }
    }
}

#[doc(hidden)]
pub trait QMetaTypeGet {
    fn get_qmetatype() -> QMetaType;
}
