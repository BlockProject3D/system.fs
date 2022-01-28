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
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, GetLastError, MAX_PATH, PWSTR, S_OK};
use windows_sys::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_LocalAppData, FOLDERID_RoamingAppData, FOLDERID_Documents, FOLDERID_Downloads, FOLDERID_Profile};
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;

fn get_windows_path(folder: GUID) -> Option<PathBuf> {
    unsafe {
        let mut str: PWSTR = std::ptr::null_mut();
        let res = SHGetKnownFolderPath(&folder, 0, std::ptr::null_mut(), &mut str as _);
        if res != S_OK {
            return None;
        }
        debug_assert_eq!(str.align_offset(2), 0);
        let mut count: usize = 0;
        while std::ptr::read(str.add(count)) != 0 {
            count += 1;
        }
        let slice = std::slice::from_raw_parts(str, count);
        let str1 = OsString::from_wide(&slice);
        CoTaskMemFree(str as _);
        Some(str1.into())
    }
}

pub fn get_app_cache() -> Option<PathBuf> {
    get_windows_path(FOLDERID_LocalAppData)
}

pub fn get_app_config() -> Option<PathBuf> {
    None //There's no dedicated app config folder under windows.
}

pub fn get_app_data() -> Option<PathBuf> {
    get_windows_path(FOLDERID_RoamingAppData)
}

pub fn get_app_logs() -> Option<PathBuf> {
    None //There's no dedicated app logs folder under windows.
}

pub fn get_app_documents() -> Option<PathBuf> {
    None //There's no dedicated app documents (public files) folder under windows.
}

fn get_exe_path() -> Option<PathBuf>
{
    unsafe {
        //Try fast path with MAX_PATH which should work for most windows versions.
        let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let res = GetModuleFileNameW(0, &mut buf as _, MAX_PATH);
        if res == 0 {
            return None; //System error.
        } else if res == MAX_PATH {
            //We might have a problem where the buffer is not large enough...
            let err = GetLastError();
            if err == ERROR_INSUFFICIENT_BUFFER {
                //We definitely have a buffer length problem!
                let mut len = MAX_PATH as usize * 2;
                loop {
                    //Start allocating twice buffer size.
                    let mut v = Vec::with_capacity(len);
                    //Attempt reading module file name again.
                    let res = GetModuleFileNameW(0, v.as_mut_ptr(), len);
                    if res == 0 {
                        return None; //System error.
                    } else if res == len {
                        let err = GetLastError();
                        if err != ERROR_INSUFFICIENT_BUFFER {
                            break;
                        }
                    } else {
                        break;
                    }
                    //If this reaches, well it's still not looking good, and we need more re-allocations.
                    len *= 2;
                }
            }
        }
        //We finally found the executable file name!
        let str1 = OsString::from_wide(&buf[..res as usize]);
        Some(str1.into())
    }
}

pub fn get_app_bundled_asset(file_name: &str) -> Option<PathBuf>
{
    //Locate app assets folder.
    let assets = get_exe_path()?.parent().ok()?.join("Assets");
    //Concat with file_name.
    let file = assets.join(file_name);
    Some(file)
}

pub fn get_user_home() -> Option<PathBuf> {
    get_windows_path(FOLDERID_Profile)
}

pub fn get_user_documents() -> Option<PathBuf> {
    get_windows_path(FOLDERID_Documents)
}

pub fn get_user_downloads() -> Option<PathBuf> {
    get_windows_path(FOLDERID_Downloads)
}
