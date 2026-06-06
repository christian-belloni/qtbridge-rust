use qtbridge::qobject;
use quicktest::run_quick_test;

#[qobject(ConvertToCamelCase)]
mod test_qobject {

    #[derive(Default)]
    pub struct TestQObject {
        bool_prop: bool,
        int_prop64: i64,
        int_prop32: i32,
        int_prop8: i8,
        int_getter_prop: i32,
        int_const: i32,
        float_prop: f64,
        string_prop: String,
    }

    impl TestQObject {

        qproperty!("boolProp", Member = bool_prop, Notify = bool_prop_changed);
        qproperty!("intProp64", Member = int_prop64, Notify = int_prop64_changed);
        qproperty!("intProp32", Member = int_prop32, Notify = int_prop32_changed);
        qproperty!("intProp8", Member = int_prop8, Notify = int_prop8_changed);
        qproperty!("intGetterProp", Read = get_int_prop, Write = set_int_prop, Notify = int_getter_prop_changed);
        qproperty!("intConstProp", Member = int_const, Constant);
        qproperty!("floatProp", Member = float_prop, Notify = float_prop_changed);
        qproperty!("stringProp", Member = string_prop, Notify = string_prop_changed);

        #[qslot]
        fn trigger(&mut self) {
            self.triggered();
        }

        #[qslot]
        fn string_slot(&mut self, value: String) {
            self.string_signal(value);
        }

        fn get_int_prop(&self) -> i32 {
            self.int_getter_prop
        }

        fn set_int_prop(&mut self, value: i32) {
            if self.int_getter_prop != value {
                self.int_getter_prop = value;
                self.int_getter_prop_changed();
            }
        }

        #[qsignal]
        fn triggered(&mut self);

        #[qsignal]
        fn string_signal(&mut self, value: String);

        #[qsignal(qml_name = "boolPropChanged")]
        fn bool_prop_changed(&mut self);

        #[qsignal(qml_name = "intProp64Changed")]
        fn int_prop64_changed(&mut self);

        #[qsignal(qml_name = "intProp32Changed")]
        fn int_prop32_changed(&mut self);

        #[qsignal(qml_name = "intProp8Changed")]
        fn int_prop8_changed(&mut self);

        #[qsignal(qml_name = "intGetterPropChanged")]
        fn int_getter_prop_changed(&mut self);

        #[qsignal(qml_name = "stringPropChanged")]
        fn string_prop_changed(&mut self);

        #[qsignal(qml_name = "floatPropChanged")]
        fn float_prop_changed(&mut self);
    }
}

#[run_quick_test(Class = test_qobject::TestQObject, Name = "testObject", Input = "qml", Harness = false)]
fn test_qobject() {}
