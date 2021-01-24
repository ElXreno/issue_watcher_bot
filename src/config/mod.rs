/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use directories::ProjectDirs;
use ruma_client::Session;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub matrix_server: String,
    pub matrix_login: String,
    pub matrix_password: String,
    pub matrix_session: Option<Session>,
    pub redmine_server: String,
    pub redmine_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            matrix_server: String::from("https://matrix.org"),
            matrix_login: String::from("user"),
            matrix_password: String::from("password"),
            matrix_session: None,
            redmine_server: String::from("https://redmine-server.com"),
            redmine_token: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Config::get_settings_path();

        if let Ok(file) = fs::File::open(&path) {
            match serde_yaml::from_reader(file) {
                Ok(s) => return s,
                Err(e) => {
                    println!("Failed to parse config file! Fallback to default. {}", e);
                    // Rename the corrupted settings file
                    let mut new_path = path.to_owned();
                    new_path.pop();
                    new_path.push("config.yaml.invalid");
                    if let Err(err) = fs::rename(path, new_path) {
                        println!("Failed to rename config file. {}", err);
                    }
                }
            }
        }

        let default_settings = Self::default();
        default_settings.backup_old_config();
        default_settings.save_to_file_warn();
        default_settings
    }

    pub fn save_to_file_warn(&self) {
        if let Err(err) = self.save_to_file() {
            panic!("Failed to save settings: {:?}", err);
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let path = Config::get_settings_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let mut config_file = fs::File::create(path)?;

        let s: &str = &serde_yaml::to_string(self).unwrap();
        config_file.write_all(s.as_bytes()).unwrap();

        Ok(())
    }

    pub fn backup_old_config(&self) -> &Self {
        let settings_file = Config::get_settings_path();

        if !settings_file.exists() {
            return self;
        }

        let settings_file_old = format!("{}.old", &settings_file.display());
        if Path::new(&settings_file).exists() {
            match fs::rename(&settings_file, &settings_file_old) {
                Ok(_o) => {
                    println!(
                        "Moved old settings file to {} successfully",
                        &settings_file_old
                    );
                }
                Err(e) => panic!("Error {}", e),
            }
        }

        self
    }

    pub fn get_settings_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com.github", "elxreno", "issue-watcher-bot")
            .expect("System's $HOME directory path not found!");

        proj_dirs.config_dir().join("config").with_extension("yaml")
    }
}
