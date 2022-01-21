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

use std::path::PathBuf;
use crate::system::{App, AppDirs, User, UserDirs};

impl AppDirs for App
{
    fn get_cache() -> Option<PathBuf> {
        if let Some(dir) = std::env::var_os("XDG_CACHE_HOME") {
            Some(dir.into())
        } else if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home).join(".cache"))
        } else {
            None
        }
    }

    fn get_config() -> Option<PathBuf> {
        if let Some(dir) = std::env::var_os("XDG_CONFIG_HOME") {
            Some(dir.into())
        } else if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home).join(".config"))
        } else {
            None
        }
    }

    fn get_data() -> Option<PathBuf> {
        if let Some(dir) = std::env::var_os("XDG_DATA_HOME") {
            Some(dir.into())
        } else if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home).join(".local/share"))
        } else {
            None
        }
    }

    fn get_logs() -> Option<PathBuf> {
        None //Per-application logs are unsupported under linux
    }

    fn get_documents() -> Option<PathBuf> {
        None //Per-application documents are unsupported under linux
    }
}

impl UserDirs for User
{
    fn get_home() -> Option<PathBuf> {
        if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home))
        } else {
            None
        }
    }

    fn get_documents() -> Option<PathBuf> {
        if let Some(dir) = std::env::var_os("XDG_DOCUMENTS_DIR") {
            Some(dir.into())
        } else if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home).join("Documents"))
        } else {
            None
        }
    }

    fn get_downloads() -> Option<PathBuf> {
        if let Some(dir) = std::env::var_os("XDG_DOWNLOAD_DIR") {
            Some(dir.into())
        } else if let Some(home) = std::env::var_os("HOME") {
            Some(PathBuf::from(home).join("Downloads"))
        } else {
            None
        }
    }
}
