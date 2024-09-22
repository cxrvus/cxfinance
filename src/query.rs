use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{database::Database, transaction::Transaction};

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

#[derive(Serialize)]
pub struct QueryResult {
	date: String,
	#[serde(flatten)]
	values: HashMap<String, i64>,
}

impl Query {
	pub fn run_by_name(name: &str) -> Result<()> {
		let db = Database::<Query>::load("queries.json")?;
		let query = db
			.records
			.iter()
			.find(|q| q.name == name)
			.ok_or(anyhow!("could not find query '{name}'"))?;
		query.run()
	}
	pub fn run(&self) -> Result<()> {
		let transactions = Database::<Transaction>::load("transactions.json")?;
		let patterns = Database::<Transaction>::load("patterns.json")?;

		// todo: group by patterns (using summation)
		// todo: group by dates (daily)
		// todo: match for aggregation & grouping

		Ok(())
	}
}

// todo: display as a table
