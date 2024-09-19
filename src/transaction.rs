use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String
}
