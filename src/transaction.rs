use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String,
}

// todo: match for patterns
// todo: group-by patterns
// todo: group-by date
// idea: fancy table display
