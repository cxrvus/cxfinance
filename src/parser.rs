use std::{fs::{read, write, File}, hash::{DefaultHasher, Hash, Hasher}, path::PathBuf};
use anyhow::{anyhow, Context, Result};
use csv::{Reader, ReaderBuilder};
use serde_json::{Map, Value};
use crate::transaction::Transaction;

pub fn parse_transactions (path: &PathBuf) -> Result<Vec<Transaction>> { 
	// idea: split up into sub-modules & match a Bank enum to support different banks formats
	parse_transactions_sk(path)
}

pub fn generate_hash<T: Hash>(item: &T) -> String {
	let mut hasher = DefaultHasher::new();
	item.hash(& mut hasher);
	hasher.finish().to_string()
	// todo: convert to hexadecimal
}

fn parse_transactions_sk (path: &PathBuf) -> Result<Vec<Transaction>> { 
	fix_uft8(path)?;

	let mut transactions: Vec<Transaction> = vec![];
	let mut rdr = get_csv_reader(path)?;

	for sk_transaction in rdr.deserialize() {
		let sk_transaction: Map<String, Value> = sk_transaction.context("failed to parse transaction")?;

		transactions.push(Transaction {
			date: parse_date_german(&sk_transaction)?,
			amount: parse_amount_german(&sk_transaction)?,
			description: parse_description_sk(&sk_transaction),
			hash: generate_hash(&sk_transaction),
		});
	}

	Ok(transactions)
}

fn get_csv_reader(path: &PathBuf) -> Result<Reader<File>> {
	ReaderBuilder::new().delimiter(b';').from_path(path).context("failed to create CSV reader")
}

fn fix_uft8(path: &PathBuf) -> Result<()> {
	let text = read(path).context("failed to read from file for sanitization")?;
	let sanitized_text = String::from_utf8_lossy(&text).to_string();
	if text != sanitized_text.as_bytes() { write(path, sanitized_text).context("failed to write to file for sanitization")?; }
	Ok(())
}

fn parse_date_german(transaction: &Map<String, Value>) -> Result<String> {
	let value = transaction.get("Valutadatum").expect("SK-transaction is missing required field 'Valutadatum'");
	let string = value.as_str().expect("cannot parse date to string");
	let parts = string.split('.').collect::<Vec<&str>>();

	let [d, m, y] = match parts.as_slice() {
		[d, m, y] => [d, m, y],
		_ => return Err(anyhow!("Invalid date format")),
	};

	let date = format!("{y}-{m}-{d}");

	Ok(date)
}

fn parse_amount_german(transaction: &Map<String, Value>) -> Result<i64> {
	let value = transaction.get("Betrag").expect("raw transaction is missing required field 'Betrag'");
	let string = value.as_str().expect("cannot parse amount to string");
	let amount: i64 = string.replace(",", "").parse().context("couldn't parse transaction amount")?;

	Ok(amount)
}

fn parse_description_sk(transaction: &Map<String, Value>) -> String {
	["Buchungstext", "Verwendungszweck", "Beguenstigter/Zahlungspflichtiger"]
		.into_iter()
		.map(|field| transaction.get(field).unwrap_or(&Value::Null).as_str().unwrap())
		.collect::<Vec<&str>>()
		.join(";")
}
