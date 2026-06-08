// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, QObjectHolder, QmlRegister, qobject_impl};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
pub struct Cat {
    pub legs: i32,
}
#[qobject_impl]
impl Cat {
    qproperty!("legs", Member = legs);
}

// Reporter is injected as an instance so QML can hand a result back to Rust for assertion.
#[derive(Default)]
pub struct Reporter {
    pub count: i32,
}
#[qobject_impl]
impl Reporter {
    #[qslot]
    fn report(&mut self, n: i32) {
        self.count = n;
    }
}

#[derive(Default)]
pub struct Container {
    kittens: Vec<Rc<RefCell<Cat>>>,
}
#[qobject_impl]
impl Container {
    qproperty!("kittens", Read = get_kittens, Write = set_kittens);

    pub fn get_kittens(&self) -> &Vec<Rc<RefCell<Cat>>> {
        &self.kittens
    }

    pub fn set_kittens(&mut self, kittens: Vec<Rc<RefCell<Cat>>>) {
        self.kittens = kittens;
    }
}

/// Getter exposure: Rust populates the list, QML reads `container.kittens`
/// through the getter and sums the `legs` of every element.
fn qml_reads_list_property_through_getter() {
    Cat::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let cat1 = Cat::default_with_attached_qobject();
    cat1.borrow_mut().legs = 4;
    let cat2 = Cat::default_with_attached_qobject();
    cat2.borrow_mut().legs = 3;

    let container = Container::default_with_attached_qobject();
    container.borrow_mut().set_kittens(vec![cat1, cat2]);
    let container_var = container.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            required property var container
            Component.onCompleted: {
                let total = 0;
                for (let i = 0; i < container.kittens.length; ++i) {
                    total += container.kittens[i].legs;
                }
                reporter.report(total);
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .add_initial_property("container", &container_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 7, "QML should read 4 + 3 = 7 legs through the getter");
}

/// Setter write-back: QML assigns a list of QML-constructed Cats to
/// `container.kittens`; the assignment goes through the QQmlListProperty view
/// of the getter-exposed field, so the Cats end up in the Rust-side Vec.
fn qml_writes_list_property_through_setter() {
    Cat::register();

    let container = Container::default_with_attached_qobject();
    let container_var = container.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var container
            property list<Cat> source: [ Cat { legs: 2 }, Cat { legs: 6 } ]
            Component.onCompleted: {
                container.kittens = source;
            }
        }
    "#;

    QApp::new()
        .add_initial_property("container", &container_var)
        .load_qml(qml.as_bytes());

    let borrowed = container.borrow();
    let kittens = borrowed.get_kittens();
    assert_eq!(kittens.len(), 2, "Rust Vec should hold the QML-assigned Cats");
    assert_eq!(kittens[0].borrow().legs, 2, "Cat 0 should have 2 legs after QML write");
    assert_eq!(kittens[1].borrow().legs, 6, "Cat 1 should have 6 legs after QML write");
}

/// Round-trip: Rust populates the list, QML reads it via the getter (and
/// reports the sum), then reassigns `container.kittens` via the setter view.
/// Both directions are verified after the QML run.
fn list_property_round_trips_through_getter_and_setter() {
    Cat::register();

    let reporter = Reporter::default_with_attached_qobject();
    let reporter_var = reporter.borrow().as_qvariant();

    let cat1 = Cat::default_with_attached_qobject();
    cat1.borrow_mut().legs = 4;
    let cat2 = Cat::default_with_attached_qobject();
    cat2.borrow_mut().legs = 3;

    let container = Container::default_with_attached_qobject();
    container.borrow_mut().set_kittens(vec![cat1, cat2]);
    let container_var = container.borrow().as_qvariant();

    let qml = r#"
        import QtQuick
        import tst_qobject_macro
        Item {
            required property var reporter
            required property var container
            property list<Cat> replacement: [ Cat { legs: 1 }, Cat { legs: 2 }, Cat { legs: 5 } ]
            Component.onCompleted: {
                let total = 0;
                for (let i = 0; i < container.kittens.length; ++i) {
                    total += container.kittens[i].legs;
                }
                reporter.report(total);
                container.kittens = replacement;
            }
        }
    "#;

    QApp::new()
        .add_initial_property("reporter", &reporter_var)
        .add_initial_property("container", &container_var)
        .load_qml(qml.as_bytes());

    assert_eq!(reporter.borrow().count, 7, "QML should first read 4 + 3 = 7 legs through the getter");

    let borrowed = container.borrow();
    let kittens = borrowed.get_kittens();
    assert_eq!(kittens.len(), 3, "Rust Vec should hold the 3 reassigned Cats");
    assert_eq!(kittens[0].borrow().legs, 1, "Cat 0 should have 1 leg after reassignment");
    assert_eq!(kittens[1].borrow().legs, 2, "Cat 1 should have 2 legs after reassignment");
    assert_eq!(kittens[2].borrow().legs, 5, "Cat 2 should have 5 legs after reassignment");
}

fn main() {
    qml_reads_list_property_through_getter();
    qml_writes_list_property_through_setter();
    list_property_round_trips_through_getter_and_setter();
}
