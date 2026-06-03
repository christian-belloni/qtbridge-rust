// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, QObjectHolder};
use qtbridge::{QmlRegister, qobject};


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
pub mod container {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::Cat;
    #[derive(Default)]
    pub struct Container {
        pub kittens: Vec<Rc<RefCell<Cat>>>,
    }
    impl Container {
        #[qsignal]
        fn kittens_changed(&mut self);

        qproperty!("kittens", Member = kittens, Notify = kittens_changed);
    }
}
pub use container::Container;

// Reporter is injected as an instance so QML can hand a result back to Rust for assertion.
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

fn qml_reads_rust_owned_list_property() {
    Cat::register();
    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let container = Container::default_with_attached_qobject();
    container.borrow_mut().kittens.push(Cat::default_with_attached_qobject());
    let container_var = container.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            required property var container
            property list<Cat> source: [ Cat { legs: 4 }, Cat { legs: 3 } ]
            Component.onCompleted: {
                container.kittens = source;
                let total = 0;
                for (let i = 0; i < container.kittens.length; ++i) {
                    let cat = container.kittens[i];
                    total += cat.legs;
                }
                reporter.report(total);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .add_initial_property("container", &container_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 7, "container.kittens should have 7 legs in total");
    assert_eq!(container.borrow().kittens.len(), 2, "Rust Vec should hold the QML-appended Cats");
    assert_eq!(container.borrow().kittens[0].borrow().legs, 4, "Cat 0 should have 4 legs after QML");
    assert_eq!(container.borrow().kittens[1].borrow().legs, 3, "Cat 1 should have 3 legs after QML");
}

fn list_property_member_notify_signal_fires_when_qml_appends() {
    Cat::register();
    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let container = Container::default_with_attached_qobject();
    let container_var = container.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            required property var container
            property list<Cat> source: [ Cat { legs: 4 }, Cat { legs: 3 } ]

            property int kittenCount: container.kittens.length
            onKittenCountChanged: reporter.report(kittenCount)

            Component.onCompleted: {
                container.kittens = source;
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .add_initial_property("container", &container_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 2, "kittenCount binding should update to 2 after QML appends");
    assert_eq!(container.borrow().kittens.len(), 2, "Rust Vec should hold the 2 QML-appended cats");
}

fn main() {
    qml_reads_rust_owned_list_property();
    list_property_member_notify_signal_fires_when_qml_appends();
}
