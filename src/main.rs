use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod tui;
mod config;
mod database;
mod import;
mod parser;
mod pattern;
mod query;
mod transaction;

/// # idea (To Dos)
///
/// ## Create Structs / Modules
///
/// - Budget
///
/// ## Add Crates
///
/// - CLI Table
/// - Date Time
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
		Cli::Categorize => import::recategorize(),
		Cli::ResetConfig => Ok(config::create_default()),
		Cli::Import(args) => import::import_transactions(args.path),
		Cli::Run(run_args) => query::Query::run_by_name(&run_args.query_name),
	}
}

#[derive(Parser)]
#[clap(version, about)]
enum Cli {
	Categorize,
	ResetConfig,
	Import(ImportArgs),
	Run(RunArgs),
}

#[derive(Parser)]
struct ImportArgs {
	path: PathBuf,
}

#[derive(Parser)]
struct RunArgs {
	#[arg(index = 1)]
	query_name: String,
}
