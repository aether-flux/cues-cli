use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

// Define config structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub current_project: String,
    pub current_project_id: u32,
    pub expires_at: String,
}

// Returns config directory
pub fn get_config_path () -> Option<PathBuf> {
    ProjectDirs::from("dev", "aetheros", "cues").map(|proj_dirs| proj_dirs.config_dir().join("config.json"))
}

// Loads and returns data from config file
pub fn load_config () -> Option<Config> {
    let path = get_config_path()?;
    let data = fs::read_to_string(path).ok()?;

    serde_json::from_str(&data).ok()
}
