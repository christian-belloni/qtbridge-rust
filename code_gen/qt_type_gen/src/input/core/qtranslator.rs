use crate::QGuiApplication;
use std::pin::Pin;

#[qt_gen::bridge]
mod qtranslator {
    include_in_cpp!(<QTranslator>);
    include_in_cpp!("rustconv.h");

    struct QTranslator;

    pub fn new() -> cxx::UniquePtr<Self> {
        let cpp = cpp_fn!(|| -> UniquePtr<Self> {
            return std::make_unique<QTranslator>();
        });

        unsafe { cpp() }
    }

    pub fn load(self: Pin<&mut Self>, path: &str) -> bool {
        let cpp = cpp_fn!(|&mut self, path: &str| -> bool {
            return self.load(RustStrToQString(path));
        });
        cpp(self, path)
    }

    pub fn translate(&self, context: &str, source: &str) -> String {
        let cpp = cpp_fn!(|&self, context: &str, source: &str| -> String {
            return QStringToRustString(self.translate(RustStrToQString(context).toLocal8Bit(), RustStrToQString(source).toLocal8Bit()));
        });

        cpp(self, context, source)
    }

    pub fn install(mut self: Pin<&mut Self>, application: &QGuiApplication) {
        let cpp = cpp_fn!(|&mut self, application: &QGuiApplication| {
            QGuiApplication::installTranslator(&self);
        });
        cpp(self, application)
    }
}
