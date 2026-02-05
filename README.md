# Qt Bridge - Rust - Pre Release

> Copyright (C) 2025 The Qt Company Ltd.
> SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

## Introduction

This library enables building modern Qt Quick user interfaces with a Rust backend. It allows you
to run [QML](https://doc.qt.io/qt-6/qmlreference.html) code and expose Rust data structures
directly to the [QML engine](https://doc.qt.io/qt-6/qqmlengine.html), combining a declarative UI
with Rust-based business logic.

Qt itself is written in C++, and QtBridge builds on [CXX](https://cxx.rs/) to access the required
Qt interfaces. As a user, you do not need to write any C++ code. Instead, Rust structs and data
can be exposed to QML using attribute macros provided by the library.

If your project requires mixing Rust and C++ code, using Qt Widgets, or accessing Qt modules that
only provide a C++ API, consider using [CXX-Qt](https://github.com/KDAB/cxx-qt) instead.

Internally, the library relies on Qt concepts such as [QObjects](https://doc.qt.io/qt-6/qobject.html),
[properties](https://doc.qt.io/qt-6/properties.html), [signals and slots](https://doc.qt.io/qt-6/signalsandslots.html),
and the [Model/View architecture](https://doc.qt.io/qt-6/model-view-programming.html). While these
are exposed through a Rust-friendly API, familiarity with these Qt concepts will help you get the
most out of building UIs with Qt Quick.

## Project structure

This repos consists both of QtBridge itself under `qtbridge` directory
as well as application examples in `apps`.

## Bridge Building and Development

### Prerequisites

To build this project, you’ll need:

- One of the supported platforms:
   - Linux (`x86_64`)
   - Windows (`x64`)
- **Rust** (stable, version >= 1.87)
   - Visit https://rustup.rs to install
- **Cargo and rustfmt** (comes with Rust)
- [**Qt 6 installation**](#qt-installation)
- **C++ toolchain** (compiler, linker etc)

Optional:
- **clang-format** available in build environment

### Qt installation

The Qt installation must be present in the system and `qmake` must be in the system PATH. There
are no special requirements to the Qt installation, it can be built from source or downloaded
from https://download.qt.io/.
To ensure qmake is available, add the Qt bin directory to your PATH, for example:

Example (Linux):
```
export PATH=<PATH_TO_QT>/bin:$PATH
```

Example (Windows PowerShell):
```
$env:Path = "<PATH_TO_QT>\bin;" + $env:Path
```

#### NOTE (Linux only)
On Linux it might be also required to add `Qt/lib` directory to `LD_LIBRARY_PATH`.

Example:
```
export LD_LIBRARY_PATH=<PATH_TO_QT>/lib:$LD_LIBRARY_PATH
```

### Build using cargo

```
cargo build
```

### Run using cargo

```
cargo run --bin <binary_name>
```

### Run tests:
```
cargo test
```

### Environment variables

Folder containing qmake must be in PATH. For example:

- On Windows:
```
set PATH=%PATH%;D:\dev\qt_build\qtbase\bin\
```

- On Linux:
```
export PATH=/home/john_doe/dev/qt_build/qtbase/bin:$PATH
export LD_LIBRARY_PATH="/home/john_doe/dev/qt_build/lib:$LD_LIBRARY_PATH"
```

## Examples

### Example code

Running a QML application is as easy as creating a QApp instance and loading the respective QML files:

```
use qtbridge::QApp;

fn main() {
    QApp::new()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
```

A Rust backend can be added with the `#[qobject_impl]` and `#[qml_element]` macros:

```
use qtbridge::{qobject_impl, qml_element};

#[derive(Default)]
pub struct Backend {
}

#[qobject_impl]
#[qml_element]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}
```

QML can use the backend as follows:

```
import QtQuick
import QtQuick.Controls
import hello_world

ApplicationWindow {
    visible: true

    Backend {
        id: backend
    }

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: backend.sayHello()
    }
}

```

### Provided Examples

There are two examples:
 - hello_world
 - minimal_app

To run those applications, you can use `cargo run` from its directory.
There are also following aliases defined, which can be executed from root dir:

Hello world example:
```
cargo run-hello-world
```

Minimal app example:
```
cargo run-minimal-app
```

## API Overview

### Borrow checker and rules

QtBridges aims to respect the borrowing rules of Rust. All QObjects in Rust (implemented with
`qobject_impl`) are designed to be held in `Rc<RefCell<_>>` structs. The QML engine also hold
Rust objects through such references. With this construct, many references can be held and
borrowed at runtime. Their QML representation can be copied following the ordinary JavaScript
rules. A Rust object is only borrowed when a slot or property defined in Rust is invoked. If
QML is unable to borrow the Rust object, it will panic. We are currently investigating
alternative ways to handle this situation more gracefully.

It is important that you do not hold a reference to any object that is accessed from QML when
`QApp::run()` is executed. For example, the following code will fail when the QML engine tries to
call into a slot defined in Rust:
```
    let qobject1: Rc<RefCell<Backend>> = Backend::default_with_attached_qobject();
    let qobject2: Rc<RefCell<Backend>> = Backend::default_with_attached_qobject();

    QApp::new()
        .with_initial_properties(& [
            ("rustmodel1", qobject1.borrow().as_qvariant()),
            ("rustmodel2", qobject2.borrow().as_qvariant()),
         ])
        .load_qml(include_bytes!("main.qml"))
        .run();
```
Isolating the borrowing into separate calls resolves the issue:
```
    let qobject1: Rc<RefCell<Backend>> = Backend::default_with_attached_qobject();
    let qobject2: Rc<RefCell<Backend>> = Backend::default_with_attached_qobject();

    let initial_properties = [
        ("rustmodel1", qobject1.borrow().as_qvariant()),
        ("rustmodel2", qobject2.borrow().as_qvariant()),
    ];
    QApp::new()
    .with_initial_properties(&initial_properties)
    .load_qml(include_bytes!("main.qml"))
    .run();
 ```
This is an evolving API and we aim to make it safer in the future.

### QApp

`QApp` represents a Qt QML application and acts as the entry point for all applications.
It allows running QML code and injecting Rust objects into it.

### qobject_impl

The `#[qobject_impl]` macro allows user-defined Rust structs to integrate with Qt and QML
by supporting **signals, slots, and properties**. When a struct is annotated with #[qobject_impl],
the macro analyzes the contents of the struct’s impl block and generates the required glue code to
connect the struct to Qt’s meta-object system.

Using this macro, a developer can:
- Notify QML about events related to a Rust object (through **signals**).
- Define **slots** that can connect to signals or be invoked directly from QML.
- Expose object values to QML as **properties**, supporting read and optionally write access.

For more details, see the Rust documentation for `#[qobject_impl]`.

### qml_element

The `#[qml_element]` macro makes Rust structures instantiable from QML. It has to be used together
with `#[qobject_impl]`. This allows you to write idiomatic QML code.

### QVec

`QVec` provides an API similar to Rust’s standard [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
In addition, it is recognized by QML as a model and can be used with UI elements such as
[ListView](https://doc.qt.io/qt-6/qml-qtquick-listview.html),
[Repeater](https://doc.qt.io/qt-6/qml-qtquick-repeater.html), and other components that require a model.

Currently, `QVec` can only be exposed to QML through the initial properties of the QML engine.

### Resource system

The best way to get artifacts such as icons into a QML application is through the
[Qt Resource System](https://doc.qt.io/qt-6/resources.html). To inject data into the Qt Resource System
you can use the macro `include_bytes_qml!` or the function `qresource::register_bytes`.

## Future Plans

* Enable interoperability with [CXX-Qt](https://crates.io/crates/cxx-qt)
* Enable interoperability with other libraries such as [tokio](https://tokio.rs/)
* Enable QObjects as properties (complex properties).
* Currently we provide QVec as an abstraction for QAbstractItemModel. We aim to provide a
  trait based API with more flexibility in the future.
* The minimal_app shows an API that resembles CXX-Qt. This will be replaced with an API that
  looks more native.
* Merge macros qobject_impl and qml_element.
* Remove all C++ Qt-classes from the API: Currently, some operations require the usage of
  QVariant, QModelIndex and there like. We are working on an API that does not require those.
* Provide Qt dependencies as a crate to enable a streamlined installation through cargo.
* Remove the dependency on Qt binaries like qmake and maybe a C++ tool chain.
* Extend IDE support in particular for VS Code. Enable the QML language server to understand
  types generated in Rust.

## Terms and Conditions

If you, your employer, or the legal entity you act on behalf of hold commercial license(s) with a Qt
Group entity, Qt Bridges constitutes Pre-Release Code under the Qt License/Frame Agreement governing
those licenses, and that agreement's terms and conditions relating to Pre-Release Code apply to your
use of Qt Bridges as found in this repo.
This Qt Bridges repo may provide links or access to third-party libraries or code (collectively
"Third-Party Software") to implement various functions. Use or distribution of Third-Party Software
is discretionary and in all respects subject to applicable license terms of applicable third-party
right holders.

### Additional Terms and Conditions

Qt Bridge for Rust is built using the Rust language and SDK, which is maintained by the Rust
Foundation.

Qt Bridge for Rust resides on top of Rust and does not modify it in any form. Rust is a trademark of
the Rust Foundation. This project is not affiliated with or endorsed by the Rust Foundation.

An application built with Qt Bridge for Rust will include code from other crates.
The main dependency is CXX, "Safe interop between Rust and C++", licensed under the Apache Version
2.0 License or MIT license.
