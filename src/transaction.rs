use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub timestamp: i64,
	pub amount: i64,
	pub description: String,
}

impl Transaction {
	pub fn new(timestamp: i64, amount: i64, description: String) -> Self {
		Transaction { timestamp, amount, description }
	}
}
