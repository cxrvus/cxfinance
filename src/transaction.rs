use crate::{database::Database, pattern::Pattern};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaggedTransaction {
	#[serde(flatten)]
	transaction: Transaction,
	category: Option<String>,
}

impl Database<Transaction> {
	pub fn tag(self) -> Result<Database<TaggedTransaction>> {
		let patterns = Database::<Pattern>::load("patterns.json")?.records;
		let transactions = self.records;
		let tagged = transactions
			.iter()
			.map(|transac| {
				let category = patterns
					.iter()
					.find(|ptn| ptn.is_match(&transac.description))
					.map(|ptn| ptn.category.clone());
				TaggedTransaction {
					transaction: transac.clone(),
					category,
				}
			})
			.collect();
		Ok(Database {
			path: self.path,
			records: tagged,
		})
	}
}
