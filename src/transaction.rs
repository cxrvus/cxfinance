use serde::{Deserialize, Serialize};

pub struct Transaction {
	pub day: String,
	pub amount: i64,
	pub description: String,
	pub hash: String
}

// todo: add RawTransaction enum that includes Transaction variations for all banks
#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct SkTransaction {
	pub day: String,
	pub amount: String,
	pub description: String,
}
