#![windows_subsystem = "windows"]

use qtbridge::{QApp, qobject};

#[derive(Default)]
pub struct Backend {}

#[qobject(Singleton)]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

fn main() {
    println!("starting qml app");
    // unsafe { std::env::set_var("QML_IMPORT_PATH", format!("{}/qml", std::env::current_dir().unwrap().display())) };
    // unsafe { std::env::set_var("QML2_IMPORT_PATH", format!("{}/qml", std::env::current_dir().unwrap().display())) };
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("../qml/Main.qml"))
        .run();
}

