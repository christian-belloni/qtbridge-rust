use cxx::UniquePtr;
use qtbridge_type_lib::QGuiApplication;

pub struct QTranslator {
    translator: UniquePtr<qtbridge_type_lib::QTranslator>,
}

impl QTranslator {
    pub fn new() -> Self {
        Self {
            translator: qtbridge_type_lib::QTranslator::new(),
        }
    }
    pub fn load(&mut self, path: &str) -> bool {
        self.translator.pin_mut().load(path)
    }

    pub(crate) fn install(&mut self, application: &QGuiApplication) {
        self.translator.pin_mut().install(application);
    }

    pub fn translate(&self, context: &str, source: &str) -> String {
        self.translator.translate(context, source)
    }
}
