// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use std::mem::MaybeUninit;

#[qt_gen::bridge]
mod qutf8stringview {
    include_in_cpp!(<QUtf8StringView>);

    #[doc(hidden)]
    #[derive_cpp(Default)]
    #[qmetatype]
    struct QUtf8StringView {
        _data: MaybeUninit<usize>,
        _size: MaybeUninit<usize>,
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(self.as_bytes())
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let mut ptr = std::ptr::null();
        let mut size = 0isize;
        cpp_fn!(|&self, ptr: &mut *const u8, size: &mut isize| {
            ptr = reinterpret_cast<const uint8_t*>(self.data());
            size = self.size();
        })(self, &mut ptr, &mut size);
        unsafe { std::slice::from_raw_parts(ptr, size as usize) }
    }
}
