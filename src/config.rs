use anyhow::Result;
use std::{fs::read_to_string, path::{Path, PathBuf}};
use serde::Deserialize;
use serde_json;

const CFG_PATH: &'static str = "~/cxconfig.json";

#[derive(Deserialize)]
pub struct Config {
	pub db_root: PathBuf
}

impl Default for Config {
	fn default() -> Self {
		Self {
			db_root: PathBuf::from("./db")
		}
	}
}

pub fn get_config() -> Result<Config> {
	let path = Path::new(CFG_PATH);
	if !path.exists() {
		println!("Config file not found, using default settings.\ncreate a {} file in your home directory.", CFG_PATH);
		return Ok(Config::default())
	}
	else {
		let string = read_to_string(path)?;
		return serde_json::from_str(&string).map_err(|e| anyhow::Error::new(e));
	}
}
