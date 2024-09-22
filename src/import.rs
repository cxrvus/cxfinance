use crate::database::Database;
use crate::transaction::Transaction;
use crate::parser::parse_transactions;
use anyhow::Result;
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

fn merge_transactions(db: Vec<Transaction>, imp: Vec<Transaction>) -> Result<Vec<Transaction>> {
	if imp.len() == 0 {
		Ok(db)
	} else if db.len() == 0 {
		Ok(imp)
	} else {
		let mut merged = db.clone();
		for transac in imp {
			if !db.iter().any(|x| x.hash == transac.hash) {
				merged.push(transac);
			}
		}
		Ok(merged)
	}
}
