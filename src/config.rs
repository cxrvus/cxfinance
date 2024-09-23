use serde::{Deserialize, Serialize};
use serde_json;
use std::{
	env::current_dir,
	fs::{read_to_string, write},
	path::PathBuf,
};

const CFG_FILE: &'static str = "cxconfig.json";

#[derive(Deserialize, Serialize)]
pub struct Config {
	pub db_root: PathBuf,
}

impl Default for Config {
	fn default() -> Self {
		let current_dir = format!("{}/{}", current_dir().unwrap().to_str().unwrap(), "db");
		Self {
			db_root: PathBuf::from(current_dir),
		}
	}
}

fn get_config_path() -> PathBuf {
	let mut path = dirs::home_dir().unwrap();
	path.push(CFG_FILE);
	path
}

pub fn get_config() -> Config {
	let path = get_config_path();
	if !path.exists() {
		create_default();
		Config::default()
	} else {
		let string = read_to_string(path).expect("failed to read config file");
		serde_json::from_str(&string).expect("invalid config file")
	}
}

pub fn create_default() {
	let path = get_config_path();
	let default_config = serde_json::to_string(&Config::default()).expect("failed to serialize default config");
	write(path, default_config).expect("failed to create default config file in your home directory");
	println!("created a default config file ({CFG_FILE}) in your home directory.");
}

pub fn get_full_path(file_path: &PathBuf) -> PathBuf {
	if file_path.is_relative() {
		join_paths(get_config().db_root, file_path)
	} else {
		file_path.clone()
	}
}

fn join_paths(a: PathBuf, b: &PathBuf) -> PathBuf {
	let mut path = a;
	path.push(b);
	return path;
}
