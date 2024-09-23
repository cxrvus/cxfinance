use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pattern {
	pub name: Option<String>,
	pub category: String,
	pattern: String,
}

impl Pattern {
	pub fn is_match(&self, string: &str) -> bool {
		let regex = Regex::new(&self.pattern);
		if let Ok(regex) = regex {
			regex.is_match(&string.to_lowercase())
		} else {
			panic!("invalid RegEx")
		}
	}
}

// idea: sanitize patterns
