use crate::tui::loading;
use crate::database::Database;
use crate::parser::parse_transactions;
use crate::pattern::Pattern;
use crate::transaction::{RawTransaction, Transaction};
use anyhow::{Ok, Result};
use std::path::PathBuf;

pub fn import_transactions(import_path: PathBuf) -> Result<()> {
	let mut db = Database::load("transactions.json")?;
	let imp_transacs = parse_transactions(&import_path)?;
	let db_transacs = db.records;
	let merged = merge_transactions(db_transacs, imp_transacs)?;

	db.records = merged;
	db.save()?;

	Ok(())
}

fn merge_transactions(
	db_transacs: Vec<Transaction>,
	imp_transacs: Vec<RawTransaction>,
) -> Result<Vec<Transaction>> {
	if imp_transacs.len() == 0 {
		Ok(db_transacs)
	} else if db_transacs.len() == 0 {
		categorize(imp_transacs)
	} else {
		let new_transacs = imp_transacs
			.into_iter()
			.filter(|imp_transac| {
				!db_transacs
					.iter()
					.any(|db_transac| db_transac.data.hash == imp_transac.hash)
			})
			.collect();
		let new_transacs = categorize(new_transacs)?;
		let merged_transacs = [db_transacs, new_transacs].concat();
		Ok(merged_transacs)
	}
}

pub fn categorize(raw_transacs: Vec<RawTransaction>) -> Result<Vec<Transaction>> {
	let patterns = Database::<Pattern>::load("patterns.json")?.records;
	let len = raw_transacs.len();
	let mut transacs = vec![];

	for (i, raw_transac) in raw_transacs.iter().enumerate() {
		loading("categorizing transactions", i, len);
		let category = patterns
			.iter()
			.find(|ptn| ptn.is_match(&raw_transac.description))
			.map(|ptn| ptn.category.clone());
		let transac = Transaction {
			data: raw_transac.clone(),
			category,
		};
		transacs.push(transac);
	}
	Ok(transacs)
}

pub fn recategorize() -> Result<()> {
	let mut db = Database::<Transaction>::load("transactions.json")?;
	let raw_transacs = db.records.into_iter().map(|t| t.data).collect();
	db.records = categorize(raw_transacs)?;
	db.save()
}
