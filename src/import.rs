use std::{fs::{create_dir, read, read_to_string, write, File}, path::PathBuf};
use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder};
use serde_json::{Map, Value};
use crate::config::get_config;

use crate::transaction::Transaction;

pub fn import_transactions (import_path: PathBuf) -> Result<()> {
	fix_uft8(&import_path)?;

	let mut rdr = get_csv_reader(&import_path)?;
	let mut db_path = get_config()?.db_root.clone();

	let imp_transacs = convert_csv_transactions(&mut rdr)?;
	let db_transacs = get_db_transactions(&mut db_path)?;
	let merged = merge_transactions(db_transacs, imp_transacs)?;
	let merged_str = serde_json::to_string_pretty(&merged)?;

	write(db_path, merged_str).context("failed to write to database file")?;

	Ok(())
}

fn get_csv_reader(path: &PathBuf) -> Result<Reader<File>> {
	ReaderBuilder::new().delimiter(b';').from_path(path).context("failed to create CSV reader")
}
 
// todo: extract to parser.rs
fn convert_csv_transactions (rdr: &mut Reader<File>) -> Result<Vec<Transaction>> { 
	let transactions = rdr.deserialize();
	let mut simple_transactions: Vec<Transaction> = vec![];

	for transaction in transactions {
		let transaction: Map<String, Value> = transaction.context("failed to parse transaction")?;
		let hash = Transaction::generate_hash(&transaction);

		let day_value = transaction.get("Valutadatum").expect("raw transaction is missing required field 'Valutadatum'");
		let day = day_value.as_str().unwrap_or_default().to_owned();
		// todo: parse with RegEx to turn into YYYY-MM-DD

		let amount_value = transaction.get("Betrag").expect("raw transaction is missing required field 'Betrag'");
		let amount_str = amount_value.as_str().unwrap_or_default();
		let amount: i64 = amount_str.replace(",", "").parse().context("couldn't parse transaction amount")?;

		let description = ["Buchungstext", "Verwendungszweck", "Beguenstigter/Zahlungspflichtiger"]
		 	.into_iter()
			.map(|field| transaction.get(field).unwrap_or(&Value::Null).as_str().unwrap())
			.collect::<Vec<&str>>()
			.join(";")
		;

		let simple_transaction = Transaction{ day, amount, description, hash: hash.to_string() };
		simple_transactions.push(simple_transaction);
	}

	Ok(simple_transactions)
}

fn fix_uft8(path: &PathBuf) -> Result<()> {
	let text = read(path).context("failed to read from file for sanitization")?;
	let sanitized_text = String::from_utf8_lossy(&text).to_string();
	if text != sanitized_text.as_bytes() { write(path, sanitized_text).context("failed to write to file for sanitization")?; }
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
			if !db.iter().any(|x| x.day == transac.day) {
				merged.push(transac);
			}
		}
		Ok(merged)
	}
}
