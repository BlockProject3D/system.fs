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

//! Low-level access to standard system directories.
//!
//! Unsupported directories are returned as None.

use std::path::PathBuf;

/// Provide access to per-platform global application directories.
pub trait AppDirs
{
    fn get_cache() -> Option<PathBuf>;
    fn get_config() -> Option<PathBuf>;
    fn get_data() -> Option<PathBuf>;
    fn get_logs() -> Option<PathBuf>;
    fn get_documents() -> Option<PathBuf>;
}

/// Provide access to per-platform current user directories.
pub trait UserDirs
{
    fn get_home() -> Option<PathBuf>;
    fn get_documents() -> Option<PathBuf>;
    fn get_downloads() -> Option<PathBuf>;
}

pub struct App
{
}

pub struct User
{
}

#[cfg(target_vendor = "apple")]
mod apple_shared;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
mod linux;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(windows)]
mod windows;

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
use linux as _impl;
#[cfg(target_os = "macos")]
use macos as _impl;
#[cfg(target_os = "ios")]
use ios as _impl;
#[cfg(windows)]
use windows as _impl;

pub fn get_app_cache() -> Option<PathBuf> {
    _impl::get_app_cache()
}

pub fn get_app_config() -> Option<PathBuf> {
    _impl::get_app_config()
}

pub fn get_app_data() -> Option<PathBuf> {
    _impl::get_app_data()
}

pub fn get_app_logs() -> Option<PathBuf> {
    _impl::get_app_logs()
}

pub fn get_app_documents() -> Option<PathBuf> {
    _impl::get_app_documents()
}

pub fn get_user_home() -> Option<PathBuf> {
    _impl::get_user_home()
}

pub fn get_user_documents() -> Option<PathBuf> {
    _impl::get_user_documents()
}

pub fn get_user_downloads() -> Option<PathBuf> {
    _impl::get_user_downloads()
}
