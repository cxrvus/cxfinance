use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
	date: String,
	amount: i64,
	description: String,
}

impl Transaction {
	pub fn new(date: String, amount: i64, description: String) -> Self {
		Transaction { date, amount, description }
	}
}
