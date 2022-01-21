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

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
fn attempt_dbus_call(path: &Path) -> bool
{
    use std::ffi::OsString;
    use zbus::{blocking::Connection, dbus_proxy, Result};
    #[dbus_proxy(
        default_service = "org.freedesktop.FileManager1",
        interface = "org.freedesktop.FileManager1",
        default_path = "/org/freedesktop/FileManager1"
    )]
    trait FileManager
    {
        fn show_folders(&self, uris: &[&str], startup_id: &str) -> Result<()>;
        fn show_items(&self, uris: &[&str], startup_id: &str) -> Result<()>;
    }
    let con = match Connection::session() {
        Ok(v) => v,
        Err(_) => return false
    };
    let proxy = match FileManagerProxyBlocking::new(&con) {
        Ok(v) => v,
        Err(_) => return false
    };
    let mut uri = OsString::from("file://");
    let f = match canonicalize(path.as_os_str()) {
        Ok(v) => v,
        Err(_) => return false
    };
    uri.push(f);
    let res;
    if path.is_dir() {
        res = proxy.show_folders(&[&uri.to_string_lossy()], "test");
    } else {
        res = proxy.show_items(&[&uri.to_string_lossy()], "test");
    }
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
fn attempt_xdg_open(path: &Path) -> bool
{
    use std::process::Command;
    use std::ffi::OsString;
    let mut uri = OsString::from("file://");
    let f = match canonicalize(path.as_os_str()) {
        Ok(v) => v,
        Err(_) => return false
    };
    uri.push(f);
    let res = Command::new("xdg-open")
        .args([&*uri.to_string_lossy()])
        .output();
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

/// Open the given path in a file explorer on the current platform.
///
/// This is unsupported on iOS as iOS can already expose application files in the Files app since iOS 11.
pub fn open<T: AsRef<Path>>(path: T) -> bool
{
    let path = path.as_ref();
    #[cfg(windows)]
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
            return res > 32
        }
    #[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
        {
            let mut flag = attempt_dbus_call(path);
            if !flag {
                flag = attempt_xdg_open(path);
            }
            return flag;
        }
}

/// Hides the given path in the current platform's file explorer.
pub fn hide<T: AsRef<Path>>(path: T)
{

}

/// Un-hides the given path in the current platform's file explorer.
pub fn unhide<T: AsRef<Path>>(path: T)
{

}

#[cfg(test)]
mod tests
{
    use crate::utils::open;
    use crate::dirs::system::get_app_data;

    #[test]
    fn open_test()
    {
        assert!(open(get_app_data().unwrap()));
    }
}
