use std::{fs::{create_dir, read_to_string, write}, path::PathBuf};
use anyhow::{Context, Result};
use crate::{config::get_config, parser::parse_transactions};

use crate::transaction::Transaction;

pub fn import_transactions (import_path: PathBuf) -> Result<()> {
	let mut db_path = get_config()?.db_root.clone();

	let imp_transacs = parse_transactions(&import_path)?;
	let db_transacs = get_db_transactions(&mut db_path)?;
	let merged = merge_transactions(db_transacs, imp_transacs)?;
	let merged_str = serde_json::to_string_pretty(&merged)?;

	write(db_path, merged_str).context("failed to write to database file")?;

	Ok(())
}

fn get_db_transactions(db_path: &mut PathBuf) -> Result<Vec<Transaction>> {
	if !db_path.exists() {
		create_dir(&db_path).context("failed to create database folder")?;
		println!("created new database folder at {}", db_path.to_str().unwrap() );
	}

	db_path.push("transactions.json");

	if !db_path.exists() {
		write(&db_path, "[]\n").context("failed to create empty database")?;
		println!("created new database file {}", db_path.file_name().unwrap().to_str().unwrap() );
	}

	let db_content = read_to_string(db_path).context("failed to read database file")?;
	let transactions: Vec<Transaction> = serde_json::from_str(&db_content).context("failed to parse database")?;
	Ok(transactions)
}

fn merge_transactions(db: Vec<Transaction>, imp: Vec<Transaction>) -> Result<Vec<Transaction>> {
	if imp.len() == 0 { Ok(db) }
	else if db.len() == 0 { Ok(imp) }
	else {
		let mut merged = db.clone();
		for transac in imp {
			if !db.iter().any(|x| x.hash == transac.hash) {
				merged.push(transac);
			}
		}
		Ok(merged)
	}
}
