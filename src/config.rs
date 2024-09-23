use serde::Deserialize;
use serde_json;
use std::{
	env::current_dir,
	fs::read_to_string,
	path::{Path, PathBuf},
};

const CFG_PATH: &'static str = "~/cxconfig.json";

#[derive(Deserialize)]
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

pub fn get_config() -> Config {
	let path = Path::new(CFG_PATH);
	if !path.exists() {
		// todo: create a default config file instead of just using default settings (and change the warning's wording)
		println!("could not find config file\nnow using default settings instead\nyou can create a {} file in your home directory.", CFG_PATH);
		return Config::default();
	} else {
		let string = read_to_string(path).expect("failed to read config file");
		serde_json::from_str(&string).expect("invalid config file")
	}
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
