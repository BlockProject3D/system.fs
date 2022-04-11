// Copyright (c) 2022, BlockProject 3D
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

use std::path::Path;

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
