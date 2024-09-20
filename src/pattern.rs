use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
	name: String,
	category: Option<String>,
	rx: String,
}

impl Pattern {
	pub fn is_match(self, string: &str) -> bool {
		let regex = Regex::new(&self.rx);
		if let Ok(regex) = regex {
			regex.is_match(&string)
		} else {
			false
		}
	}
}

// idea: sanitize patterns
