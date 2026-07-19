#![windows_subsystem = "windows"]

use qtbridge::{QApp, qobject, QObjectHolder, invoke_method};

#[derive(Default)]
pub struct Backend {
    counter: u64,
    text: String
}

#[qobject(Singleton)]
impl Backend {
    qproperty!("counter", Member = counter, Notify = counter_changed);
    qproperty!("text", Member = text);

    #[qslot]
    fn startup(&self) {
        let invoker = self.get_qml_method_invoker();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                invoke_method!(invoker, "increment");
                println!("increment");
            }
        });
    }

    #[qslot]
    fn increment(&mut self) {
        self.counter += 1;
        self.counter_changed();
    }

    #[qsignal]
    fn counter_changed(&mut self);
}

fn main() {
    println!("starting qml app");
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("../qml/Main.qml"))
        .run();
}

