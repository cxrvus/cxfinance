use crate::pattern::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
	pub date: String,
	pub amount: i64,
	pub description: String,
	pub hash: String,
}

impl Transaction {
	pub fn _is_match(&self, pattern: Pattern) -> bool {
		pattern._is_match(&self.description)
	}
}
