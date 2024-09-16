use std::{fs::read, path::Path};
use serde::Deserialize;

pub const CFG_PATH: &'static str = "~/cxconfig.json";

#[derive(Deserialize)]
pub struct Config {
	db_root: Path
}

impl Default for Config {
	fn default() -> Self {
		Self {
			db_root: "./db"
		}
	}
}

pub fn get_config() {
	todo!("get config file")
}
