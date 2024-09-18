use std::{fs::{read, read_to_string, write, File}, path::PathBuf};
use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder};
use serde_json::{Map, Value};
use crate::config::get_config;

use crate::transaction::Transaction;

pub fn import_transactions (path: PathBuf) -> Result<()> {
	fix_uft8(&path)?;

	let mut rdr = get_transaction_reader(&path)?;
	let new = convert_raw_transactions(&mut rdr)?;
	let existing = get_existing_transactions()?;
	let merged = merge_transactions(existing, new)?;

	Ok(())
}

fn get_transaction_reader(path: &PathBuf) -> Result<Reader<File>> {
	ReaderBuilder::new().delimiter(b';').from_path(path).context("failed to create CSV reader")
}

fn convert_raw_transactions (rdr: &mut Reader<File>) -> Result<Vec<Transaction>> { 
	let mut simple_transactions: Vec<Transaction> = vec![];
	let transactions = rdr.deserialize();

	for transaction in transactions {
		let transaction: Map<String, Value> = transaction.context("failed to parse transaction")?;

		let date = transaction.get("Buchungstag").expect("raw transaction is missing required field 'Buchungstag'");
		let date = date.as_str().unwrap_or_default().to_owned();
		let amount = transaction.get("Betrag").expect("raw transaction is missing required field 'Betrag'");
		let amount = amount.as_i64().unwrap_or_default();

		let description = ["Buchungstext", "Verwendungszweck", "Beguenstigter/Zahlungspflichtiger"]
		 	.into_iter()
			.map(|field| transaction.get(field).unwrap_or(&Value::Null).as_str().unwrap())
			.collect::<Vec<&str>>()
			.join(";\n")
		;

		let simple_transaction = Transaction::new(date, amount, description);
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

fn get_existing_transactions() -> Result<Vec<Transaction>> {
	let mut db_path = get_config()?.db_root.clone();
	db_path.push("/transactions.json");
	if !db_path.exists() { write(&db_path, "[]")?; }
	let db_content = read_to_string(db_path)?;
	let transactions: Vec<Transaction> = serde_json::from_str(&db_content)?;
	Ok(transactions)
}

fn merge_transactions(existing: Vec<Transaction>, new: Vec<Transaction>) -> Result<Vec<Transaction>> {
	Ok(existing)
}
