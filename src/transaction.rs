use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawTransaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	#[serde(flatten)]
	pub data: RawTransaction,
	pub category: Option<String>,
}
