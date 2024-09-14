use std::{fs::{self, File}, path::PathBuf};
use anyhow::{Context, Result};
use csv::{self, Reader};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
	date: String,
	amount: f64,
	description: String,
}

pub fn import_transactions (path: PathBuf) -> Result<()> {
	fix_uft8(&path)?;

	let mut rdr = csv::ReaderBuilder::new()
		.delimiter(b';')
		.from_path(path)
		.context("failed to create CSV reader")?
	;

	let t = convert_transactions(&mut rdr);

	println!("{:#?}", t);
	Ok(())
}


pub fn convert_transactions (rdr: &mut Reader<File>) -> Result<Vec<Transaction>> { 
	let mut simple_transactions: Vec<Transaction> = vec![];
	let transactions = rdr.deserialize();

	for transaction in transactions {
		let transaction: Map<String, Value> = transaction.context("failed to parse transaction")?;

		let date = transaction.get("Buchungstag").expect("raw transaction is missing required field 'Buchungstag'");
		let date = date.as_str().unwrap_or_default().to_owned();
		let amount = transaction.get("Betrag").expect("raw transaction is missing required field 'Betrag'");
		let amount = amount.as_f64().unwrap_or_default();

		let description = ["Buchungstext", "Verwendungszweck", "Beguenstigter/Zahlungspflichtiger"]
		 	.into_iter()
			.map(|field| transaction.get(field).unwrap_or(&Value::Null).as_str().unwrap())
			.collect::<Vec<&str>>()
			.join(";\n")
		;

		let simple_transaction = Transaction { date, amount, description };
		simple_transactions.push(simple_transaction);
	}

	Ok(simple_transactions)
}

fn fix_uft8(path: &PathBuf) -> Result<()> {
	let text = fs::read(path).context("failed to read from file for sanitization")?;
	let sanitized_text = String::from_utf8_lossy(&text).to_string();
	if text != sanitized_text.as_bytes() { fs::write(path, sanitized_text).context("failed to write to file for sanitization")?; }
	Ok(())
}
