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

use std::path::{Path, PathBuf};

pub fn get_app_cache() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .map(|v| v.into())
        .or_else(|| std::env::var_os("HOME")
            .map(|v| PathBuf::from(v).join(".cache")))

}

pub fn get_app_config() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(|v| v.into())
        .or_else(|| std::env::var_os("HOME")
            .map(|v| PathBuf::from(v).join(".config")))
}

pub fn get_app_data() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .map(|v| v.into())
        .or_else(|| std::env::var_os("HOME")
            .map(|v| PathBuf::from(v).join(".local/share")))
}

pub fn get_app_logs() -> Option<PathBuf> {
    None //Per-application logs are unsupported under linux
}

pub fn get_app_documents() -> Option<PathBuf> {
    None //Per-application documents are unsupported under linux
}

#[cfg(target_os = "freebsd")]
fn get_exe_path_freebsd() -> Option<PathBuf> {
    use libc::size_t;
    use libc::strlen;
    use libc::sysctl;
    use libc::CTL_KERN;
    use libc::KERN_PROC;
    use libc::KERN_PROC_PATHNAME;
    use libc::PATH_MAX;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let mut mib = [CTL_KERN, KERN_PROC, KERN_PROC_PATHNAME, -1];
    let mut buf: Vec<u8> = Vec::with_capacity(PATH_MAX);
    let mut cb: size_t = PATH_MAX;
    unsafe {
        let res = sysctl(
            mib.as_mut_ptr(),
            4,
            buf.as_mut_ptr() as *mut _,
            &mut cb as _,
            std::ptr::null_mut(),
            0,
        );
        if res == 0 {
            //FreeBSD without procfs.
            let len = strlen(buf.as_ptr() as _);
            //This is where we defer from process_path: we use std::os::unix::ffi::OsStrExt.
            let str = OsStr::from_bytes(&buf[..len]);
            let path = PathBuf::from(str);
            Some(path)
        } else {
            //FreeBSD with procfs.
            std::fs::read_link("/proc/curproc/file").ok()
        }
    }
}

fn get_exe_path() -> Option<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "freebsd")] {
            get_exe_path_freebsd().parent().map(|v| v.into())
        } else {
            //Try various paths to match as many unix systems as possible.
            let mut path = Path::new("/proc/self/exe");
            if !path.exists() {
                path = Path::new("/proc/curproc/exe");
            }
            if !path.exists() {
                path = Path::new("/proc/curproc/file");
            }
            let link = std::fs::read_link(path).ok()?;
            link.parent().map(|v| v.into())
        }
    }
}

pub fn get_app_bundled_asset(file_name: &str) -> Option<PathBuf> {
    //Locate app assets folder.
    let assets = get_exe_path()?.join("Assets");
    //Concat with file_name.
    let file = assets.join(file_name);
    Some(file)
}

pub fn get_user_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|v| v.into())
}

pub fn get_user_documents() -> Option<PathBuf> {
    std::env::var_os("XDG_DOCUMENTS_DIR")
        .map(|v| v.into())
        .or_else(|| std::env::var_os("HOME")
            .map(|v| PathBuf::from(v).join("Documents")))
}

pub fn get_user_downloads() -> Option<PathBuf> {
    std::env::var_os("XDG_DOWNLOAD_DIR")
        .map(|v| v.into())
        .or_else(|| std::env::var_os("HOME")
            .map(|v| PathBuf::from(v).join("Downloads")))
}
