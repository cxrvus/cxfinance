use std::{
	fs::{create_dir, read_to_string, write},
	path::PathBuf,
};

use anyhow::{Context, Ok, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::config::get_config;

pub struct Database<T: DeserializeOwned + Serialize> {
	file_path: PathBuf,
	pub records: Vec<T>,
}

impl<T: DeserializeOwned + Serialize> Database<T> {
	pub fn load(file_path: &str) -> Result<Self> {
		let folder_path = get_config()?.db_root.clone();

		if !folder_path.exists() {
			create_dir(&folder_path).context("failed to create database folder")?;
			println!(
				"created new database folder at {}",
				folder_path.to_str().unwrap()
			);
		}

		let mut full_path = folder_path.clone();
		full_path.push(&file_path);

		if !full_path.exists() {
			write(&full_path, "[]\n").context("failed to create empty database")?;
			println!(
				"created new database file {}",
				full_path.file_name().unwrap().to_str().unwrap()
			);
		}

		let db_content = read_to_string(&full_path).context("failed to read database file")?;
		let records: Vec<T> =
			serde_json::from_str(&db_content).context("failed to parse database")?;

		Ok(Self {
			file_path: full_path,
			records,
		})
	}

	pub fn save(&self) -> Result<()> {
		let stringified = serde_json::to_string_pretty(&self.records)?;
		write(&self.file_path, stringified).context("failed to write to database file")?;
		Ok(())
	}
}
