#![windows_subsystem = "windows"]

use qtbridge::{QApp, QObjectHolder, invoke_method, qobject};

#[derive(Default)]
pub struct Backend {
    counter: u64,
}

#[derive(Default)]
pub struct StyleOverrides {}

#[qobject(Singleton)]
impl Backend {
    qproperty!("counter", Member = counter, Notify = counter_changed);

    #[qslot]
    fn increment(&mut self) {
        self.counter += 1;
        self.counter_changed();
    }

    #[qslot]
    fn reset(&mut self) {
        self.counter = 0;
        self.counter_changed();
    }

    #[qsignal]
    fn counter_changed(&mut self);

    #[qslot]
    fn startup(&self) {
        let invoker = self.get_qml_method_invoker();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                invoke_method!(invoker, "increment");
            }
        });
    }
}

fn main() {
    println!("starting qml app");
    qtbridge::qresource::register_bytes_with_prefix(
        include_bytes!("../CustomColor/CustomColor.rcc"),
        "/qt/qml/CustomColor",
    );
    qtbridge::qresource::register_bytes_with_prefix(
        include_bytes!("../qml/qml.rcc"),
        "/qt/qml/Main",
    );
    QApp::new()
        .register::<Backend>()
        .add_import_path("qrc:/qt/qml")
        .add_import_path("qrc:/qt/qml/CustomColor")
        .load_qml_from_file("qrc:/qt/qml/Main/Main.qml")
        .run();
}
