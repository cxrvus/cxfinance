use serde::{Deserialize, Serialize};
use crate::pattern::Pattern;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String,
}

impl Transaction {
	pub fn is_match(&self, pattern: Pattern) -> bool { pattern.is_match(&self.description) }
}

// todo: group-by patterns
// todo: group-by date
// idea: fancy table display
