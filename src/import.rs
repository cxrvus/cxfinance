use std::path::PathBuf;
use anyhow::{Context, Result};
use csv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
	date: String,
	amount: f64,
	description: String,
}

pub fn import_transactions (path: PathBuf) -> Result<()> {
	let t = convert_transactions(path);
	println!("{:#?}", t);
	Ok(())
}


pub fn convert_transactions (path: PathBuf) -> Result<Vec<Transaction>> { 
	let mut rdr = csv::ReaderBuilder::new()
		.delimiter(b';')
		.from_path(path)?;
	let transactions = rdr.deserialize();
	let mut simple_transactions: Vec<Transaction> = vec![];

	for transaction in transactions {
		// todo: parse to serde first, then convert
		// todo: also refactor to make convert_transactions pure
		// todo: add anyhow contexts (here and in main)

		let transaction: Map<String, Value> = transaction.context("failed to parse transaction")?;
		// println!("{:?}", transaction);
		// break;

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
