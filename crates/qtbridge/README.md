This library enables building modern Qt Quick user interfaces with a Rust backend.
It allows you to run [QML](https://doc.qt.io/qt-6/qmlreference.html) code and use
Rust types in QML. This combines a declarative UI with Rust-based business logic.

Running QML code and starting any Qt-based application is generally done through the
[`QApp`] type.

Rust types that should be used in QML must be annotated with either a [`qobject`] or
[`qobject_impl`] attribute macro. Within those blocks you can use the [`qproperty`],
[`qslot`], and [`qsignal`] attribute macros to define how the Rust types appear in QML.

The library provides some special traits that enable Rust types to fulfill specific
roles. These are [`QListModel`] and [`QTableModel`].

## API Example:

Exposing a Rust type to QML and starting the declarative UI is as simple as:

Main.rs
```
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
    <Backend as qtbridge::QmlRegister>::register();
    QApp::new()
        .load_qml(include_bytes!("qml/Main.qml"))
        .run();
}
```

The UI can then be described in a declarative way in QML:

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
- **Rust** (stable, version >= 1.87)
   - Visit [rustup.rs](https://rustup.rs) to install
- **Cargo and rustfmt** (comes with Rust)
- **C++ toolchain** (compiler, linker, etc)
- [**Qt 6**](#qt-installation)

### Qt installation

A Qt installation must be present in the system and `qmake` must be in the
system PATH. There are no special requirements for the Qt installation. It
can be built from source or downloaded from <https://download.qt.io/>.
To ensure qmake is available, add the Qt bin directory to your PATH.

- On Windows:
```sh
set PATH=%PATH%;D:\dev\qt_build\qtbase\bin\
```

- On Linux:
```sh
export PATH=/home/john_doe/dev/qt_build/qtbase/bin:$PATH
export LD_LIBRARY_PATH="/home/john_doe/dev/qt_build/lib:$LD_LIBRARY_PATH"
```

### Dependency

QtBridge has a single crate with all public APIs:
```TOML
[dependencies]
qtbridge
```

### Provided Examples

Note: In the preliminary version of this repository, the package dependency is given
as a relative path. With the final release, the package dependency can be resolved
with cargo and crates.io. Until then you need to adapt the dependency or check out
the whole repository.

#### [Hello World!](https://github.com/qt/qtbridge-rust/tree/dev/apps/hello_world)

The classic "Hello World!" example showing the minimal building bricks for a
QtBridge application.

#### [Minimal App](https://github.com/qt/qtbridge-rust/tree/dev/apps/minimal_app)

This example shows the "minimum viable example" with working backend and data
visualisation in QML.


## API Overview

### Borrow checker and rules

QtBridge aims to respect the borrowing rules of Rust. All QObjects in Rust (implemented
with [`qobject_impl`]) are designed to be held in `Rc<RefCell<_>>` structs. The QML
engine also holds Rust objects through such references. With this construct, many
references can be held and borrowed at runtime. Their QML representation can be copied
following the ordinary JavaScript rules. A Rust object is only borrowed when a slot or
property defined in Rust is invoked. If QML is unable to borrow the Rust object, it
will panic. We are currently investigating alternative ways to handle this situation
more gracefully.

## Further information:

Qt itself is written in C++, and QtBridge builds on [CXX](https://cxx.rs/) to access
the required Qt interfaces. As a user, you do not need to write any C++ code.
Instead, Rust structs and data can be exposed to QML using attribute macros provided
by the library.

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

## Future Plans

* Enable interoperability with [CXX-Qt](https://crates.io/crates/cxx-qt)
* Enable interoperability with other libraries such as [tokio](https://tokio.rs/)
* Enable QObjects as properties (complex properties).
* Provide Qt dependencies as a crate to enable a streamlined installation through cargo.
* Remove the dependency on Qt binaries like qmake and maybe a C++ tool chain.
* Extend IDE support in particular for VS Code. Enable the QML language server to understand
  types generated in Rust.
