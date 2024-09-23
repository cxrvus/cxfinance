use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{database::Database, pattern::Pattern, transaction::Transaction};

#[derive(Debug, Deserialize, Default)]
enum Grouping {
	#[default]
	Daily,
	Monthly,
}

#[derive(Debug, Deserialize, Default)]
enum Aggregation {
	#[default]
	Sum,
	Avg,
	Count,
	Median,
}

#[derive(Debug, Default, Deserialize)]
pub struct Query {
	name: String,
	description: Option<String>,
	grouping: Option<Grouping>,
	aggregation: Option<Aggregation>,
	categories: Option<Vec<String>>,
	from_date: Option<String>,
	to_date: Option<String>,
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
		// fixme: unnecessary clone?
		println!("{:?}", self);
		let _categories = self.categories.clone().expect("TODO");
		// todo: filter for self.categories
		// todo: default categories to ALL

		let transactions = Database::<Transaction>::load("transactions.json")?;

		//temporary display for tagged transactions:
		let mut tagged = transactions.tag()?;
		tagged.path = "tagged.json".into();
		tagged.save()?;

		// todo: group by patterns (using summation)
		// todo: group by dates (daily)
		// todo: match for aggregation & grouping
		// todo: fill in empty date groupings

		Ok(())
	}
}

// todo: display as a table
