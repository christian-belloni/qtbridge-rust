#![cfg(test)]
use std::pin::Pin;
use cxx::UniquePtr;
use crate::{QObject, QVariantList};

// TODO: move from qt_type_lib to separate tests-related crate?

#[qt_gen::bridge]
mod qsignalspy {
    include_in_cpp!(<QSignalSpy>);
    include_in_cpp!("rustconv.h");

    /// The `QSignalSpy` is a struct that enables introspection of signal emission.
    ///
    /// A `QSignalSpy` can connect to a signal of a `QObject` and record each time the signal is emitted.
    /// `QSignalSpy` itself is a `QVariantList`. Each emission of the signal will append one item to the list, containing the arguments of the signal.
    ///
    /// See also: [QSignalSpy documentation](https://doc.qt.io/qt-6/qsignalspy.html#details).
    struct QSignalSpy;

    /// Creates a new `QSignalSpy` that listens for emissions of a signal of the `qobject`.
    ///
    /// # Arguments
    ///
    /// * `signal_name` - The name of the signal function to observe.
    pub fn new(qobject: &QObject, signal_name: &str) -> UniquePtr<Self> {
        let cpp = cpp_fn!(|qobject: &QObject, signal_name: &str| -> UniquePtr<Self> {
            const QByteArray signalNameBa = RustStrToQByteArray(signal_name);

            // Get the signature of the signal function
            const auto * mo = qobject.metaObject();
            const auto methodCount = mo->methodCount();
            for(int m = mo->methodOffset(); m < methodCount; ++m)
            {
                auto method = mo->method(m);
                if (method.nameView() == signalNameBa)
                {
                    auto sign = method.methodSignature();
                    // The input string must be formatted the same way as after call of `SIGNAL()` C++ macro.
                    // Otherwise, QSignalSpy::isValid() returns false and the signal won't be monitored.
                    const char signalCodeChar = '0' + QSIGNAL_CODE;
                    sign.insert(0, signalCodeChar);
                    return std::make_unique<QSignalSpy>(&qobject, sign.constData());
                }
            }

            return nullptr;
        });

        let ptr = cpp(qobject, signal_name);
        if ptr.is_null() {
            panic!("Failed to create QSignalSpy")
        }
        ptr
    }

    /// Returns the number of items currently stored in the underlying [QList][crate::QList].
    pub fn count(&self) -> isize {
        let cpp = cpp_fn!(|&self| -> isize {
            return self.count();
        });
        cpp(self)
    }

    /// Removes the element at position `idx` from the underlying [QList][crate::QList] and returns it.
    pub fn take_at(mut self: Pin<&mut Self>, idx: isize) -> QVariantList {
        let cpp = cpp_fn!(|&mut self, idx: isize| -> QVariantList {
            return self.takeAt(idx);
        });
        cpp(self.as_mut(), idx)
    }

    /// Removes the first item from the underlying [QList][crate::QList] and returns it.
    pub fn take_first(mut self: Pin<&mut Self>) -> QVariantList {
        let cpp = cpp_fn!(|&mut self| -> QVariantList {
            return self.takeFirst();
        });
        cpp(self.as_mut())
    }

    /// Removes the last item from the underlying [QList][crate::QList] and returns it.
    pub fn take_last(mut self: Pin<&mut Self>) -> QVariantList {
        let cpp = cpp_fn!(|&mut self| -> QVariantList {
            return self.takeLast();
        });
        cpp(self.as_mut())
    }
}
