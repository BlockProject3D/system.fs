// Copyright (c) 2021, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub use dunce::canonicalize;
use std::path::Path;

#[cfg(all(unix, not(any(target_vendor = "apple", target_os = "android"))))]
fn attempt_dbus_call(path: &Path) -> bool {
    use std::ffi::OsString;
    use zbus::{blocking::Connection, dbus_proxy, Result};
    #[dbus_proxy(
        default_service = "org.freedesktop.FileManager1",
        interface = "org.freedesktop.FileManager1",
        default_path = "/org/freedesktop/FileManager1"
    )]
    trait FileManager {
        fn show_folders(&self, uris: &[&str], startup_id: &str) -> Result<()>;
        fn show_items(&self, uris: &[&str], startup_id: &str) -> Result<()>;
    }
    let con = match Connection::session() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let proxy = match FileManagerProxyBlocking::new(&con) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let mut uri = OsString::from("file://");
    let f = match canonicalize(path.as_os_str()) {
        Ok(v) => v,
        Err(_) => return false,
    };
    uri.push(f);
    let res = if path.is_dir() {
        res = proxy.show_folders(&[&uri.to_string_lossy()], "test");
    } else {
        res = proxy.show_items(&[&uri.to_string_lossy()], "test");
    };
    res.is_ok()
}

#[cfg(all(unix, not(any(target_vendor = "apple", target_os = "android"))))]
fn attempt_xdg_open(path: &Path) -> bool {
    use std::ffi::OsString;
    use std::process::Command;
    let mut uri = OsString::from("file://");
    let f = match canonicalize(path.as_os_str()) {
        Ok(v) => v,
        Err(_) => return false,
    };
    uri.push(f);
    let res = Command::new("xdg-open")
        .args([&*uri.to_string_lossy()])
        .output();
    res.is_ok()
}

// Force link against AppKit on mac
#[cfg(target_os = "macos")]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

/// Open the given path in a file explorer on the current platform.
///
/// This is unsupported on iOS as iOS can already expose application files in the Files app since iOS 11.
pub fn open<T: AsRef<Path>>(path: T) -> bool {
    let path = path.as_ref();
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            unsafe {
                use std::os::windows::ffi::OsStrExt;
                use windows_sys::Win32::UI::Shell::ShellExecuteW;
                use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;
                use windows_sys::Win32::Foundation::PWSTR;
                let operation = ['o' as u16, 'p' as u16, 'e' as u16, 'n' as u16, 0x0000];
                let mut file: Vec<u16> = path.as_os_str().encode_wide().collect();
                file.push(0x0000);
                // Well windows-sys is badly designed it treats all strings as mutable
                // even though the official MS docs uses constant strings
                let file: PWSTR = std::mem::transmute(file.as_ptr());
                let operation: PWSTR = std::mem::transmute(operation.as_ptr());
                let res = ShellExecuteW(0, operation, file, std::ptr::null_mut(), std::ptr::null_mut(), SW_SHOW as _);
                res > 32
            }
        } else if #[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))] {
            let mut flag = attempt_dbus_call(path);
            if !flag {
                flag = attempt_xdg_open(path);
            }
            flag
        } else if #[cfg(target_os = "macos")] {
            use std::os::unix::ffi::OsStrExt;
            use std::os::raw::c_ulong;
            use objc::class;
            use objc::msg_send;
            use objc::sel;
            use objc::sel_impl;
            use objc::runtime::Object;
            const NS_UTF8_STRING_ENCODING: c_ulong = 4;
            let f = match canonicalize(path.as_os_str()) {
                Ok(v) => v,
                Err(_) => return false
            };
            let isdir = path.is_dir();
            unsafe {
                let nsstring = class!(NSString);
                let nsurl = class!(NSURL);
                let nsarray = class!(NSArray);
                let nsworkspace = class!(NSWorkspace);
                let mut str: *mut Object = msg_send![nsstring, alloc];
                str = msg_send![str,
                    initWithBytes: f.as_os_str().as_bytes().as_ptr()
                    length: f.as_os_str().len() as c_ulong
                    encoding: NS_UTF8_STRING_ENCODING
                ];
                let mut url: *mut Object = msg_send![nsurl, alloc];
                url = msg_send![url,
                    initFileURLWithPath: str
                    isDirectory: isdir
                ];
                if isdir {
                    let workspace: *mut Object = msg_send![nsworkspace, sharedWorkspace];
                    let _: () = msg_send![workspace, openURL: url];
                } else {
                    let arr: *mut Object = msg_send![nsarray, arrayWithObject: url];
                    let workspace: *mut Object = msg_send![nsworkspace, sharedWorkspace];
                    let _: () = msg_send![workspace, activateFileViewerSelectingURLs: arr];
                }
                // release objects
                // do not release the array as it's still owned by Foundation
                let _: () = msg_send![url, release]; // release url (we used alloc)
                let _: () = msg_send![str, release]; // release string (we used alloc)
                true
            }
        } else {
            false
        }
    }
}

/// Hides the given path in the current platform's file explorer.
pub fn hide<T: AsRef<Path>>(path: T) -> bool {
    let path = path.as_ref();
    if !path.exists() {
        return false;
    }
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            use os_str_bytes::OsStrBytes;
            use os_str_bytes::OsStringBytes;
            use std::ffi::OsString;
            use std::path::PathBuf;
            if let Some(str) = path.file_name() {
                let bytes = str.to_raw_bytes();
                if bytes[0] == b'.' {
                    return true; //path is already hidden.
                }
                let mut vec = bytes.to_vec();
                vec.insert(0, b'.');
                let mut copy: PathBuf = path.into();
                copy.set_file_name(OsString::from_raw_vec(vec).unwrap());
                return std::fs::rename(path, copy).is_ok();
            }
            false //the path does not have a valid file name; can't do anything.
        } else {
            use std::os::windows::ffi::OsStrExt;
            use windows_sys::Win32::Storage::FileSystem::SetFileAttributesW;
            use windows_sys::Win32::Storage::FileSystem::GetFileAttributesW;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;
            use windows_sys::Win32::Storage::FileSystem::INVALID_FILE_ATTRIBUTES;
            use windows_sys::Win32::Foundation::PWSTR;
            let mut file: Vec<u16> = path.as_os_str().encode_wide().collect();
            file.push(0x0000);
            unsafe {
                // Well windows-sys is badly designed it treats all strings as mutable
                // even though the official MS docs uses constant strings
                let file: PWSTR = std::mem::transmute(file.as_ptr());
                let attrs = GetFileAttributesW(file);
                if attrs == INVALID_FILE_ATTRIBUTES {
                    return false;
                }
                SetFileAttributesW(file, attrs | FILE_ATTRIBUTE_HIDDEN) != 0
            }
        }
    }
}

/// Un-hides the given path in the current platform's file explorer.
pub fn unhide<T: AsRef<Path>>(path: T) -> bool {
    let path = path.as_ref();
    if !path.exists() {
        return false;
    }
    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            use os_str_bytes::OsStrBytes;
            use os_str_bytes::OsStringBytes;
            use std::ffi::OsString;
            use std::path::PathBuf;
            if let Some(str) = path.file_name() {
                let bytes = str.to_raw_bytes();
                if bytes[0] != b'.' {
                    return true; //path is already visible.
                }
                let mut vec = bytes.to_vec();
                vec.remove(0); //remove the '.' character from the file name.
                let mut copy: PathBuf = path.into();
                copy.set_file_name(OsString::from_raw_vec(vec).unwrap());
                return std::fs::rename(path, copy).is_ok();
            }
            false //the path does not have a valid file name; can't do anything.
        } else {
            use std::os::windows::ffi::OsStrExt;
            use windows_sys::Win32::Storage::FileSystem::SetFileAttributesW;
            use windows_sys::Win32::Storage::FileSystem::GetFileAttributesW;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;
            use windows_sys::Win32::Storage::FileSystem::INVALID_FILE_ATTRIBUTES;
            use windows_sys::Win32::Foundation::PWSTR;
            let mut file: Vec<u16> = path.as_os_str().encode_wide().collect();
            file.push(0x0000);
            unsafe {
                // Well windows-sys is badly designed it treats all strings as mutable
                // even though the official MS docs uses constant strings
                let file: PWSTR = std::mem::transmute(file.as_ptr());
                let attrs = GetFileAttributesW(file);
                if attrs == INVALID_FILE_ATTRIBUTES {
                    return false;
                }
                SetFileAttributesW(file, attrs & !FILE_ATTRIBUTE_HIDDEN) != 0
            }
        }
    }
}
