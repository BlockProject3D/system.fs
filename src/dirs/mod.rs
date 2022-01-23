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

pub mod system;

/// Represents all possible errors when requesting app directories.
pub enum Error
{
    /// The system is missing an application data directory.
    MissingDataDir,

    /// An io error has occurred while created some directory.
    Io(std::io::Error)
}

impl From<std::io::Error> for Error
{
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

/// Represents the application's directories.
///
/// Main entry point to obtain any directory for your application.
///
/// These APIs will fail as last resort. If they fail it usually means the system has a problem.
/// The system may also include specific configuration to break applications on purpose,
/// in which case these APIs will also fail.
pub struct App<'a>
{
    name: &'a str
}

impl<'a> App<'a>
{
    /// Creates a new application.
    ///
    /// # Arguments
    ///
    /// * `name`: the name of the application.
    ///
    /// returns: App
    pub fn new(name: &'a str) -> App<'a>
    {
        App {
            name
        }
    }

    /// Returns the path to this application's files.
    ///
    /// Use this directory to store any information not intended to be user accessible.
    ///
    /// # Errors
    ///
    /// Returns a [MissingDataDir](self::Error::MissingDataDir) if this system doesn't have any application
    /// writable location; this should never occur on any supported system except if such system is broken.
    ///
    /// Returns an [Io](self::Error::Io) if some directory couldn't be created.
    pub fn get_data(&self) -> Result<PathBuf, Error>
    {
        let data = system::get_app_data().ok_or(Error::MissingDataDir)?.join(self.name);
        if !data.is_dir() {
            std::fs::create_dir_all(&data)?;
        }
        Ok(data)
    }

    /// Returns the path to this application's cache.
    ///
    /// Use this directory to store cached files such as downloads, intermediate files, etc.
    ///
    /// # Errors
    ///
    /// Returns an [Io](self::Error::Io) if some directory couldn't be created.
    pub fn get_cache(&self) -> Result<PathBuf, Error>
    {
        let cache = match system::get_app_cache() {
            None => self.get_data()?.join("Cache"),
            Some(cache) => cache.join(self.name)
        };
        if !cache.is_dir() {
            std::fs::create_dir(&cache)?;
        }
        Ok(cache)
    }

    /// Returns the path to this application's public documents.
    ///
    /// Use this directory to store any content the user should see and alter.
    ///
    /// # Errors
    ///
    /// Returns an [Io](self::Error::Io) if some directory couldn't be created.
    pub fn get_documents(&self) -> Result<PathBuf, Error>
    {
        // If this is OK then we must be running from a sandboxed system
        // where the app has it's own public documents folder, otherwise
        // create a "public" Documents directory inside the application data directory.
        match system::get_app_documents() {
            Some(docs) => Ok(docs),
            None => {
                let docs = self.get_data()?.join("Documents");
                if !docs.is_dir() {
                    std::fs::create_dir(&docs)?;
                }
                Ok(docs)
            }
        }
    }

    /// Returns the path to this application's logs.
    ///
    /// Use this directory to store all logs. The user can view and alter this directory.
    ///
    /// # Errors
    ///
    /// Returns an [Io](self::Error::Io) if some directory couldn't be created.
    pub fn get_logs(&self) -> Result<PathBuf, Error>
    {
        // Logs should be public and not contain any sensitive information, so store that in
        // the app's public documents.
        let logs = match system::get_app_logs() {
            None => self.get_documents()?.join("Logs"),
            Some(logs) => logs.join(self.name)
        };
        if !logs.is_dir() {
            std::fs::create_dir(&logs)?;
        }
        Ok(logs)
    }

    /// Returns the path to this application's config.
    ///
    /// Use this directory to store all configs for the current user.
    /// This directory is not intended for direct user access.
    ///
    /// # Errors
    ///
    /// Returns an [Io](self::Error::Io) if some directory couldn't be created.
    pub fn get_config(&self) -> Result<PathBuf, Error>
    {
        let config = match system::get_app_config() {
            None => self.get_data()?.join("Config"),
            Some(config) => config.join(self.name)
        };
        if !config.is_dir() {
            std::fs::create_dir(&config)?;
        }
        Ok(config)
    }
}
