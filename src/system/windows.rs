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

use std::ffi::OsString;
use std::path::PathBuf;
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{MAX_PATH, PWSTR, S_OK};
use windows_sys::Win32::UI::Shell::SHGetKnownFolderPath;
use crate::system::{App, AppDirs};

fn get_windows_path(folder: GUID) -> Option<PathBuf> {
    unsafe {
        let str: [u16; MAX_PATH as _] = [0; MAX_PATH as _];
        let res = SHGetKnownFolderPath(&folder, 0, std::ptr::null(), &str as _);
        if res != S_OK {
            return None;
        }
        let mut count: usize = 0;
        while str[count] != 0 {
            count += 1;
        }
        let str = OsString::from_wide(&str[..count]);
        Some(str.into())
    }
}

impl AppDirs for App {
    fn get_cache() -> Option<PathBuf> {
        todo!()
    }

    fn get_config() -> Option<PathBuf> {
        todo!()
    }

    fn get_data() -> Option<PathBuf> {
        todo!()
    }

    fn get_logs() -> Option<PathBuf> {
        todo!()
    }

    fn get_documents() -> Option<PathBuf> {
        todo!()
    }
}
