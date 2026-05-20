// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::env::args_os;
use std::sync::LazyLock;

static ARGC_ARGV: LazyLock<(Vec<u8>, Vec<usize>, i32)> = LazyLock::new(|| {
    let args = args_os();
    let arg_count = args.len();
    let mut arg_offsets = Vec::with_capacity(arg_count);
    let mut args_joined = Vec::new();
    for arg in args {
        arg_offsets.push(args_joined.len());
        args_joined.extend(arg.to_string_lossy().to_string().as_bytes());
        args_joined.push(0);
    }
    let arg_ptrs: Vec<usize> = arg_offsets.iter()
        .map(|offset| (args_joined.as_ptr() as usize) + *offset)
        .collect();
    (args_joined, arg_ptrs, arg_count as i32)
});

#[qt_gen::bridge]
mod qguiapplication {
    include_in_cpp!(<QGuiApplication>);
    include_in_cpp!("rustconv.h");

    /// The QGuiApplication struct manages the GUI application's control flow and main settings.
    ///
    /// QGuiApplication contains the main event loop, where all events from the window system
    /// and other sources are processed and dispatched. It also handles the application's
    /// initialization and finalization, and provides session management.
    ///
    /// See also: [QGuiApplication documentation](https://doc.qt.io/qt-6/qguiapplication.html).
    struct QGuiApplication;

    /// Initializes the window system and constructs an application object.
    /// Reads command line arguments and passes them further to the instance of QGuiApplication.
    pub fn new() -> cxx::UniquePtr<Self> {
        let (_, argv, argc) = &*ARGC_ARGV;

        let cpp = cpp_fn!(|argc: &i32, argv: *const usize| -> UniquePtr<Self> {
            return std::make_unique<QGuiApplication>(
                const_cast<int&>(argc),
                const_cast<char**>(reinterpret_cast<char*const*>(argv)));
        });
        unsafe { cpp(argc, argv.as_ptr()) }
    }

    /// Enters the main event loop and waits until the application finishes execution.
    /// It is necessary to call this function to start event handling.
    /// The main event loop receives events from the window system and dispatches these to the application components.
    /// See also: [QGuiApplication::exec()](https://doc.qt.io/qt-6/qguiapplication.html#exec).
    pub fn exec() -> i32{
        let cpp = cpp_fn!(|| -> i32 {
            return QGuiApplication::exec();
        });
        cpp()
    }

    pub fn process_events(&self) {
        cpp_fn!(|&self| {
            self.processEvents();
        })(self);
    }

    /// Sets the name of this application.
    pub fn set_application_name(name: &str) {
        let cpp = cpp_fn!(|name: &str| {
            QGuiApplication::setApplicationName(RustStrToQString(name));
        });
        cpp(name)
    }
}
