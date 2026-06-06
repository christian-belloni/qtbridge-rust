// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, QObjectHolder, QmlRegister, qobject};

#[qobject]
pub mod cat {
    #[derive(Default)]
    pub struct Cat {
        pub legs: i32,
    }
    impl Cat {
        qproperty!("legs", Member = legs);
    }
}
pub use cat::Cat;

#[qobject]
pub mod reporter {
    #[derive(Default)]
    pub struct Reporter {
        pub count: i32,
    }
    impl Reporter {
        #[qslot]
        fn report(&mut self, n: i32) {
            self.count = n;
        }
    }
}
pub use reporter::Reporter;

#[qobject]
pub mod backend {
    use std::rc::Rc;
    use std::cell::RefCell;
    use qtbridge::QObjectHolder;
    use super::Cat;

    #[derive(Default)]
    pub struct Backend {
        pub received_legs: Vec<i32>,
    }

    impl Backend {
        #[qsignal]
        fn cats_out(&mut self, cats: Vec<Rc<RefCell<Cat>>>);

        #[qslot]
        fn receive_cats(&mut self, cats: Vec<Rc<RefCell<Cat>>>) {
            self.received_legs = cats.iter().map(|c| c.borrow().legs).collect();
        }

        #[qslot]
        fn emit_cats(&mut self) {
            let cat1 = Cat::default_with_attached_qobject();
            cat1.borrow_mut().legs = 4;
            let cat2 = Cat::default_with_attached_qobject();
            cat2.borrow_mut().legs = 3;
            self.cats_out(vec![cat1, cat2]);
        }
    }
}
pub use backend::Backend;

/// Round-trip: QML connects `cats_out` signal to `receive_cats` slot on the same backend,
/// then triggers `emit_cats`. The Vec<Rc<RefCell<Cat>>> is serialised to QObjectList on
/// emission and deserialised back on slot invocation.
fn signal_vec_of_nested_types_round_trips_through_qt_metacall() {
    Cat::register();

    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            required property var backend

            Component.onCompleted: {
                backend.cats_out.connect(backend.receive_cats)
                backend.emit_cats()
            }
        }
    "#;

    QApp::new()
        .add_initial_property("backend", &backend_var)
        .load_qml(qml.as_bytes());

    let received = &backend.borrow().received_legs;
    assert_eq!(received.len(), 2, "two cats should arrive in the slot");
    assert_eq!(received[0], 4, "first cat should have 4 legs");
    assert_eq!(received[1], 3, "second cat should have 3 legs");
}

/// Signal emission: emit `cats_out` from Rust, verify that QML receives the list and can
/// inspect each element's `legs` property.
fn signal_vec_of_nested_types_can_be_received_in_qml() {
    Cat::register();

    let backend = Backend::default_with_attached_qobject();
    let reporter = Reporter::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();
    let reporter_var = reporter.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            required property var backend
            required property var reporter

            Connections {
                target: backend
                function onCats_out(cats) {
                    let total = 0;
                    for (let i = 0; i < cats.length; ++i) {
                        total += cats[i].legs;
                    }
                    reporter.report(total);
                }
            }

            Component.onCompleted: backend.emit_cats()
        }
    "#;

    QApp::new()
        .add_initial_property("backend", &backend_var)
        .add_initial_property("reporter", &reporter_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 7, "QML should sum 4 + 3 = 7 legs from emitted cats");
}

/// Slot invocation: QML passes a JavaScript array of QML-constructed Cats to `receive_cats`.
fn slot_with_vec_of_nested_types_can_be_called_from_qml() {
    Cat::register();

    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            id: root
            required property var backend

            Component.onCompleted: {
                let cat1 = Qt.createQmlObject(
                    `import tst_qobject_macro; Cat { legs: 2 }`, root);
                let cat2 = Qt.createQmlObject(
                    `import tst_qobject_macro; Cat { legs: 6 }`, root);
                backend.receive_cats([cat1, cat2]);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("backend", &backend_var)
        .load_qml(qml.as_bytes());

    let received = &backend.borrow().received_legs;
    assert_eq!(received.len(), 2, "two cats should arrive in the slot");
    assert_eq!(received[0], 2, "first cat should have 2 legs");
    assert_eq!(received[1], 6, "second cat should have 6 legs");
}

fn main() {
    signal_vec_of_nested_types_round_trips_through_qt_metacall();
    signal_vec_of_nested_types_can_be_received_in_qml();
    slot_with_vec_of_nested_types_can_be_called_from_qml();
}
