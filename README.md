This library enables building modern Qt Quick user interfaces with a Rust backend,
without writing any C++ code. It allows you to run [QML](https://doc.qt.io/qt-6/qmlreference.html)
code and expose Rust types to QML using simple attribute macros.

## At a glance

Main.rs
```rust
use qtbridge::{qobject_impl, QApp};

#[derive(Default)]
pub struct Backend {
}

#[qobject_impl(Singleton)]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

fn main() {
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
```

Main.qml
```qml, ignore
import QtQuick
import QtQuick.Controls
import hello_world

ApplicationWindow {

    visible: true
    title: qsTr("Minimal QML app")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.sayHello()
    }
}
```

## Getting started

### Prerequisites

To use this library, you need:

- One of the supported platforms:
   - Linux (`x86_64`)
   - Windows (`x64`)
   - macOS (`arm64`) - experimental
- **Rust** (stable, version >= 1.87)
   - Visit [rustup.rs](https://rustup.rs) to install
- **Cargo and rustfmt** (comes with Rust)
- **C++ toolchain** (compiler, linker, etc)
- [**Qt 6**](#qt-installation)

### Qt installation

A Qt installation must be present in the system and `qmake` must be in the
system `PATH`. It can be built from source, downloaded from <https://download.qt.io/>,
or installed via your system's package manager.

QtBridge currently requires Qt 6.10 or later. The QtBridge 1.0 release is
expected to require Qt 6.11 or later.

To ensure `qmake` is available, add the Qt bin directory to your PATH.

- On Windows, add the Qt bin directory to PATH. Note that the exact path depends
  on your installation location, Qt version, and compiler:
  - Built from source:
```sh
    set PATH=%PATH%;D:\dev\qt_build\qtbase\bin\
```
  - From the official installer:
```sh
    set PATH=%PATH%;C:\Qt\6.10.1\msvc2022_64\bin\
```

- On Linux when building from source, add the Qt bin directory to `PATH` and the
  Qt libraries to `LD_LIBRARY_PATH`:
```sh
  export PATH=/home/john_doe/dev/qt_build/qtbase/bin:$PATH
  export LD_LIBRARY_PATH="/home/john_doe/dev/qt_build/lib:$LD_LIBRARY_PATH"
```

- On Linux when using the system package manager, install the required development
  packages. You will need `qtbase`, `qtdeclarative`, and `qtquickcontrols2` as a
  baseline for the provided examples, plus any additional modules you intend to
  use from QML. You will also need the private API of `qtbase`. On Fedora, e.g.:
```sh
  sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtquickcontrols2-devel
  sudo dnf install qt6-qtbase-private-devel
```
  Your system installs these to the correct paths, so no additional environment
  variables are needed.

- On macOS add the following directories to `PATH` and `DYLD_FRAMEWORK_PATH`:
```sh
export PATH=/Users/john_doe/dev/qt_build/qtbase/bin:$PATH
export DYLD_FRAMEWORK_PATH=/Users/john_doe/dev/qt_build/qtbase/lib:$DYLD_FRAMEWORK_PATH
```

### Dependency

QtBridge has a single crate with all public APIs:
```TOML
[dependencies]
qtbridge = "*"
```

## API overview

Running QML code and starting any Qt-based application is generally done through the
[`QApp`] type.

Rust types that should be used in QML must be annotated with either a [`qobject`] or
[`qobject_impl`] attribute macro. Within those blocks you can use the [`qproperty`],
[`qslot`], and [`qsignal`] attribute macros to define how the Rust types appear in QML.

The library provides some special traits that enable Rust types to fulfill specific
roles; see [special_traits].

All QObjects in Rust (implemented with [`qobject_impl`]) are held in
`Rc<RefCell<_>>`. This allows multiple owners to coexist: your Rust code,
the QML engine, and any number of QML components can all hold a reference
to the same object. QML references follow ordinary JavaScript copy semantics.

A Rust object is only borrowed for the duration of a signal, slot or property
invocation from QML. Outside of those calls the object is not borrowed, so
Rust code can freely take mutable borrows between QML interactions.

`RefCell` enforces at runtime that only one mutable borrow exists at a time.
QML requires a mutable borrow to invoke a slot or write a property. When an
object enters Rust, its borrow is cached and reused for any further calls on
the same object within that call chain, conforming to a stack-based model.
A panic occurs only if a second independent borrow of the same object is
attempted from a different point in the call chain.

## Provided examples

Note: In the preliminary version of this repository, the package dependency is given
as a relative path. With the final release, the package dependency can be resolved
with cargo and crates.io. Until then you need to adapt the dependency or check out
the whole repository.

### [Hello World!](https://github.com/qt/qtbridge-rust/tree/dev/apps/hello_world)

The classic "Hello World!" example showing the minimal building bricks for a
QtBridge application.

### [Minimal App](https://github.com/qt/qtbridge-rust/tree/dev/apps/minimal_app)

A working backend with data displayed in QML.

### [Host Monitor](https://github.com/qt/qtbridge-rust/tree/dev/apps/host_monitor)

This example shows how to combine a Qt UI with [tokio](https://tokio.rs/) runtime
in a multithreaded environment using [`QmlMethodInvoker`].

### [Color Palette](https://github.com/qt/qtbridge-rust/tree/dev/apps/color_palette)

Port of the C++ Color Palette example to Rust. Shows a moderately complex application
using tokio, reqwest, and serde_json, as well as many QML constructs, such as complex
and default properties, including nested property structures.

## Further information

QtBridge builds on [CXX](https://cxx.rs/) to access the required Qt interfaces.

If your project requires mixing Rust and C++ code, using Qt Widgets, or accessing Qt
modules that only provide a C++ API, consider using
[CXX-Qt](https://github.com/KDAB/cxx-qt) instead.

Internally, the library relies on Qt concepts such as
[QObjects](https://doc.qt.io/qt-6/qobject.html),
[properties](https://doc.qt.io/qt-6/properties.html),
[signals and slots](https://doc.qt.io/qt-6/signalsandslots.html),
and the [Model/View architecture](https://doc.qt.io/qt-6/model-view-programming.html).
While these are exposed through a Rust-friendly API, familiarity with these Qt concepts
will help you get the most out of building UIs with Qt Quick.

## Future plans

* Streamline API.
* Enable interoperability with [CXX-Qt](https://crates.io/crates/cxx-qt).
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
