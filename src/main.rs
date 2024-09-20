use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod import;
mod parser;
mod pattern;
mod transaction;

/// # idea (To Dos)
///
/// ## Create Structs / Modules
///
/// - Budget
/// - Query
/// - Aggregation (sum, avg, median, count etc)
/// 	- Grouping (day, week, month, year, pattern etc)
///
/// ## Add Crates
///
/// - CLI Table
///

fn main() {
	match execute() {
		Ok(_) => {}
		Err(e) => println!("<!>\n{:?}", e),
	}
}

fn execute() -> Result<()> {
	let res = Cli::parse();
	match res {
		Cli::Import(args) => import::import_transactions(args.path),
	}
}

#[derive(Parser)]
#[clap(version, about)]
enum Cli {
	Import(ImportArgs),
}

#[derive(Parser)]
struct ImportArgs {
	#[arg(required = true)]
	path: PathBuf,
}
