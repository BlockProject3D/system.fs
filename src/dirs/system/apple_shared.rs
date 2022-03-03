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

use std::ffi::OsStr;
use std::os::raw::{c_char, c_int, c_ulong};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use libc::{PATH_MAX, strlen};
use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::runtime::Object;
use objc_foundation::{INSArray, INSString, NSArray, NSObject, NSString};

pub const NS_LIBRARY_DIRECTORY: c_ulong = 5;
pub const NS_USER_DIRECTORY: c_ulong = 7;
pub const NS_DOCUMENT_DIRECTORY: c_ulong = 9;
pub const NS_CACHES_DIRECTORY: c_ulong = 13;
pub const NS_APPLICATION_SUPPORT_DIRECTORY: c_ulong = 14;
pub const NS_DOWNLOADS_DIRECTORY: c_ulong = 15;

const NS_USER_DOMAIN_MASK: c_ulong = 1;

pub fn get_macos_dir(directory: c_ulong) -> Option<String>
{
    unsafe {
        let nsfilemanager = class!(NSFileManager);
        let instance: *mut Object = msg_send![nsfilemanager, defaultManager];
        let directories: *const NSArray<NSObject> = msg_send![instance, URLsForDirectory:directory inDomains:NS_USER_DOMAIN_MASK];
        if let Some(obj) = (*directories).first_object() {
            let str: *const NSString = msg_send![obj, path];
            if str.is_null() {
                return None;
            }
            let data = (*str).as_str();
            let copy = String::from(data);
            // do not release array as array is still owned by Foundation
            Some(copy)
        } else {
            None
        }
    }
}

pub fn get_macos_dir_fail_if_sandbox(directory: c_ulong) -> Option<PathBuf>
{
    if let Some(dir) = get_macos_dir(directory) {
        if dir.contains("Library/Containers/") { //Running in a sandbox
            None
        } else {
            Some(PathBuf::from(dir))
        }
    } else {
        None
    }
}

extern "C" {
    pub fn _NSGetExecutablePath(buf: *mut c_char, bufsize: *mut u32) -> c_int;
}

pub fn get_exe_path() -> Option<PathBuf>
{
    let mut buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let mut size: u32 = PATH_MAX as u32;
    unsafe {
        let res = _NSGetExecutablePath(&mut buf as _, &mut size as _);
        if res == -1 {
            //path is too large
            let mut v = Vec::with_capacity(size as usize);
            let res = _NSGetExecutablePath(v.as_mut_ptr(), &mut size as _);
            if res != 0 { //Something really bad happened.
                return None;
            }
            let str = OsStr::from_bytes(std::mem::transmute(&v[..size as usize]));
            return Some(PathBuf::from(str));
        }
        if res != 0 {
            return None;
        }
        let len = strlen(buf.as_ptr());
        let str = OsStr::from_bytes(std::mem::transmute(&buf[..len as usize]));
        Some(PathBuf::from(str))
    }
}

pub fn get_bundled_asset(name: &str) -> Option<PathBuf>
{
    let (file_path, file_name) = name.rfind('/')
        .map(|v| (Some(&name[..v]), &name[v + 1..]))
        .unwrap_or_else(|| (None, name));
    let (res_name, res_ext) = file_name.rfind('.')
        .map(|v| (&file_name[..v], &file_name[v + 1..]))
        .unwrap_or_else(|| (file_name, ""));
    unsafe {
        const NS_UTF8_STRING_ENCODING: c_ulong = 4;
        let nsstring = class!(NSString);
        let nsbundle = class!(NSBundle);
        let bundle: *mut Object = msg_send![nsbundle, mainBundle];
        if bundle.is_null() {
            return None;
        }
        let mut ns_res_name: *mut Object = msg_send![nsstring, alloc];
        let mut ns_res_ext: *mut Object = msg_send![nsstring, alloc];
        ns_res_name = msg_send![ns_res_name,
                    initWithBytes: res_name.as_bytes().as_ptr()
                    length: res_name.len() as c_ulong
                    encoding: NS_UTF8_STRING_ENCODING
        ];
        ns_res_ext = msg_send![ns_res_ext,
                    initWithBytes: res_ext.as_bytes().as_ptr()
                    length: res_ext.len() as c_ulong
                    encoding: NS_UTF8_STRING_ENCODING
        ];
        let str: *const NSString = match file_path {
            None => msg_send![bundle, pathForResource: ns_res_name ofType: ns_res_ext],
            Some(subpath) => {
                let mut ns_subpath: *mut Object = msg_send![nsstring, alloc];
                ns_subpath = msg_send![ns_subpath,
                    initWithBytes: subpath.as_bytes().as_ptr()
                    length: subpath.len() as c_ulong
                    encoding: NS_UTF8_STRING_ENCODING
                ];
                let str = msg_send![bundle, pathForResource: ns_res_name ofType: ns_res_ext inDirectory: ns_subpath];
                let _: () = msg_send![ns_subpath, release]; //release subpath as we're not gonna use it again anymore
                str
            }
        };
        let _: () = msg_send![ns_res_ext, release]; //release res_ext as we're not gonna use it again anymore
        let _: () = msg_send![ns_res_name, release]; //release res_name as we're not gonna use it again anymore
        if str.is_null() { //Asset wasn't found.
            return None;
        }
        let data = (*str).as_str();
        Some(PathBuf::from(data))
    }
}
