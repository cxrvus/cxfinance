use anyhow::{Ok, Result};
use serde::Deserialize;

#[derive(Deserialize, Default)]
enum Grouping {
	#[default]
	Daily,
	Monthly,
}

#[derive(Deserialize, Default)]
enum Aggregation {
	#[default]
	Sum,
	Avg,
	Count,
	Median,
}

#[derive(Deserialize)]
pub struct Query {
	name: String,
	description: Option<String>,
	grouping: Option<Grouping>,
	aggregation: Option<Aggregation>,
	categories: Option<Vec<String>>,
}

impl Query {
	pub fn run_by_name(name: &str) -> Result<()> {
		Ok(())
	}
	pub fn run(&self) -> Result<()> {
		Ok(())
	}
}
