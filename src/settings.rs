
use std::io;
use std::sync::Mutex;
use lazy_static::lazy_static;
use home::home_dir;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;

pub mod settings_tui;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub settings_file: String,
    pub workdir: String,
    pub db_file: String,
    pub context_file: String,
    pub tobi_command: String,
    pub context_changes_dir: bool,
}

impl Settings {
    pub fn new_default() -> Self {
        Settings {
            settings_file: home_dir().unwrap().join(".tobi").to_str().unwrap().to_string(),
            workdir: "Not set".to_string(),
            db_file: "Not set".to_string(),
            context_file: "Not set".to_string(),
            tobi_command: "ctf".to_string(),
            context_changes_dir: true,
        }
    }
}

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new_default());
}

pub fn load_settings_from_file() -> io::Result<()> {
    let mut settings = SETTINGS.lock().unwrap();
    let settings_path = Path::new(&settings.settings_file);
    // check if settings file exists
    if !settings_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Settings file not found"))
    }

    let settings_file = fs::File::open(settings_path)?;
    *settings = serde_json::from_reader::<_, Settings>(settings_file)?;

    // // check if settings paths are reachable with read_dir
    // if  Path::new(&settings.workdir).read_dir().is_ok() &&
    //     Path::new(&settings.db_file).read_dir().is_ok() &&
    //     Path::new(&settings.context_file).read_dir().is_ok() {
    //     return Ok(())
    // }
    if settings.workdir != "Not set" &&
       settings.db_file != "Not set" &&
       settings.context_file != "Not set" {
        return Ok(())
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "Settings paths not found"))
}

pub fn save_settings_to_file() -> io::Result<()> {
    let settings = SETTINGS.lock().unwrap();
    let settings_file = fs::File::create(&settings.settings_file)?;
    serde_json::to_writer(settings_file, &*settings)?;

    Ok(())
}

pub fn reset_settings() {
    let mut settings = SETTINGS.lock().unwrap();
    *settings = Settings::new_default();
}

pub fn show_settings_menu() -> io::Result<()> {
    settings_tui::run_setting_menu()
}