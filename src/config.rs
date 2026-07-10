// File        : idmrs/src/config.rs
// Author      : Hadi Cahyadi <cumulus13@gmail.com>
// Description : simple ini-backed config store equivalent to the Python `configset`
//               usage in idm.py (CONFIG.get_config / CONFIG.write_config)

use ini::Ini;
use std::path::PathBuf;

/// Config wraps an ini file located next to the executable (idm.ini),
/// matching the Python version's `Path(__file__).parent / 'idm.ini'`.
///
/// Keys such as `DOWNLOAD_PATH`, `DOWNLOAD_CONFIRM`, `DATA_USER_AGENT`,
/// `DEBUG_VERBOSE` are mapped to `section = first_segment`,
/// `option = rest`, both lower-cased, e.g. `DOWNLOAD_PATH` -> [download] path.
pub struct Config {
    path: PathBuf,
    ini: Ini,
}

fn split_key(key: &str) -> (String, String) {
    match key.split_once('_') {
        Some((section, option)) => (section.to_lowercase(), option.to_lowercase()),
        None => (key.to_lowercase(), key.to_lowercase()),
    }
}

impl Config {
    /// Load (or create empty) the config file next to the current executable.
    pub fn load() -> Self {
        let path = Self::config_path();
        let ini = Ini::load_from_file(&path).unwrap_or_default();
        Config { path, ini }
    }

    fn config_path() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("idm.ini")
    }

    /// Equivalent of `CONFIG.get_config('DOWNLOAD_PATH')` etc, returns None if unset.
    pub fn get(&self, key: &str) -> Option<String> {
        let (section, option) = split_key(key);
        self.ini
            .section(Some(section.as_str()))
            .and_then(|s| s.get(option.as_str()))
            .map(|v| v.to_string())
    }

    /// Boolean helper: treats "1", "true", "yes", "on" (case-insensitive) as true.
    pub fn get_bool(&self, key: &str) -> bool {
        match self.get(key) {
            Some(v) => matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on" | "ok"),
            None => false,
        }
    }

    /// Equivalent of `CONFIG.write_config(section, option, value)`.
    pub fn write_config(&mut self, section: &str, option: &str, value: &str) -> anyhow::Result<()> {
        self.ini
            .with_section(Some(section.to_lowercase()))
            .set(option.to_lowercase(), value);
        self.ini.write_to_file(&self.path)?;
        Ok(())
    }

    pub fn print_docs(&self) {
        use colored::*;
        println!("{}", "uppercase words is VALUE NAME".cyan());
        println!();
        println!("{}", "download:path:DIR_NAME".bright_yellow());
        println!("{}", "download:confirm:1 or 0".bright_green());
        println!("{}", "data:user_agent:STRING".bright_green());
        println!("{}", "debug:verbose:1 or 0".bright_green());
    }
}
