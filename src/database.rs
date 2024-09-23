use std::{
	fs::{create_dir, read_to_string, write},
	path::PathBuf,
};

use anyhow::{Context, Ok, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::config::{get_config, get_full_path};

pub struct Database<T> {
	pub path: PathBuf,
	pub records: Vec<T>,
}

impl<T: DeserializeOwned> Database<T> {
	pub fn load(file_path: &str) -> Result<Self> {
		let folder_path = get_config().db_root.clone();

		if !folder_path.exists() {
			create_dir(&folder_path).expect("failed to create database folder");
			println!(
				"created new database folder at {}",
				folder_path.to_str().unwrap()
			);
		}

		let file_path = PathBuf::from(file_path);

		if file_path.extension().is_none() {
			panic!(
				"specified database file is missing a file extension\n(needs to be JSON or CSV)"
			);
		}

		let full_path = get_full_path(&file_path);

		let records: Vec<T> = if !full_path.exists() {
			write(&full_path, "[]\n").expect("failed to create empty database");
			println!(
				"created new database file {}",
				full_path.file_name().unwrap().to_str().unwrap()
			);
			Ok(vec![])
		} else {
			let content = read_to_string(&full_path).expect("failed to read database file");
			serde_json::from_str(&content).context("failed to parse database")
		}?;

		Ok(Self {
			path: file_path,
			records,
		})
	}
}

impl<T: Serialize + DeserializeOwned> Database<T> {
	pub fn save(self) -> Result<()> {
		let full_path = get_full_path(&self.path);
		let content = serde_json::to_string_pretty(&self.records)?;
		write(full_path, content).expect("failed to write to database file");
		Ok(())
	}
}
