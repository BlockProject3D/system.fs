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
use crate::system::apple_shared::{get_macos_dir, get_macos_dir_fail_if_sandbox, NS_APPLICATION_SUPPORT_DIRECTORY, NS_CACHES_DIRECTORY, NS_DOCUMENT_DIRECTORY, NS_DOWNLOADS_DIRECTORY, NS_LIBRARY_DIRECTORY, NS_USER_DIRECTORY};

impl AppDirs for App
{
    fn get_cache() -> Option<PathBuf> {
        get_macos_dir(NS_CACHES_DIRECTORY).map(|v| PathBuf::from(v))
    }

    fn get_config() -> Option<PathBuf> {
        get_macos_dir(NS_LIBRARY_DIRECTORY).map(|path| PathBuf::from(path).join("Preferences"))
    }

    fn get_data() -> Option<PathBuf> {
        get_macos_dir(NS_APPLICATION_SUPPORT_DIRECTORY).map(|v| PathBuf::from(v))
    }

    fn get_logs() -> Option<PathBuf> {
        get_macos_dir(NS_LIBRARY_DIRECTORY).map(|path| PathBuf::from(path).join("Logs"))
    }

    fn get_documents() -> Option<PathBuf> {
        get_macos_dir(NS_DOCUMENT_DIRECTORY).map(|v| PathBuf::from(v))
    }
}

impl UserDirs for User
{ //On iOS there exists no user directories as all applications are sandboxed
    fn get_home() -> Option<PathBuf> {
        None
    }

    fn get_documents() -> Option<PathBuf> {
        None
    }

    fn get_downloads() -> Option<PathBuf> {
        None
    }
}
