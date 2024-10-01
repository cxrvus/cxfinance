use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{database::Database, parser::convert_to_csv_str, transaction::Transaction, tui};

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

#[derive(Default)]
pub enum ResultFormat {
	#[default]
	Table,
	Csv,
}

impl TryFrom<String> for ResultFormat {
	type Error = anyhow::Error;

	fn try_from(value: String) -> Result<Self> {
		use ResultFormat::*;

		match value.to_lowercase().as_str() {
			"table" => Ok(Table),
			"csv" => Ok(Csv),
			"" => Ok(Self::default()),
			invalid => Err(anyhow!("invalid format option: {invalid}")),
		}
	}
}

impl Query {
	pub fn run_by_name(name: &str, fmt: ResultFormat) -> Result<()> {
		let db = Database::<Query>::load("queries.json")?;
		let query = db
			.records
			.iter()
			.find(|q| q.name == name)
			.ok_or(anyhow!("could not find query '{name}'"))?;
		query.run(fmt)
	}
	pub fn run(&self, fmt: ResultFormat) -> Result<()> {
		// fixme: unnecessary clone?
		let _categories = self.categories.clone().expect("TODO");
		// todo: filter for self.categories
		// todo: default categories to ALL

		let records = Database::<Transaction>::load("transactions.json")?.records;

		let output = match fmt {
			ResultFormat::Table => {
				let table = tui::table(&records, vec!["date", "amount", "category", "hash"]);
				table.to_string()
			}
			ResultFormat::Csv => convert_to_csv_str(&records)?,
			// todo: fix CSV conversion (unable to parse Maps)
			// idea: just write your own function
		};

		println!("{output}");

		// todo: group by patterns (using summation)
		// todo: group by dates (daily)
		// todo: match for aggregation & grouping
		// todo: fill in empty date groupings

		Ok(())
	}
}

// todo: display as a table
