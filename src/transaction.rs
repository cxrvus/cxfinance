use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct Transaction {
	pub day: String,
	pub amount: i64,
	pub description: String,
	pub hash: String
}

impl Transaction {
	pub fn generate_hash<T: Hash>(item: &T) -> u64 {
		let mut hasher = DefaultHasher::new();
		item.hash(& mut hasher);
		hasher.finish()
	}
}
