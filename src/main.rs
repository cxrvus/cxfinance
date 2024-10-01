use anyhow::Result;
use clap::Parser;
use query::ResultFormat;
use std::path::PathBuf;

mod config;
mod database;
mod import;
mod parser;
mod pattern;
mod query;
mod transaction;
mod tui;

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
		Cli::ResetConfig => {
			config::create_default();
			Ok(())
		}
		Cli::Import(args) => import::import_transactions(args.path),
		Cli::Run(run_args) => {
			let fmt = ResultFormat::try_from(run_args.fmt.unwrap_or_default())?;
			query::Query::run_by_name(&run_args.name, fmt)
		}
	}
}

#[derive(Parser)]
#[clap(version, about)]
enum Cli {
	Categorize,
	ResetConfig,
	Import(ImportArgs),
	Run(RunQuery),
}

#[derive(Parser)]
struct ImportArgs {
	path: PathBuf,
}

#[derive(Parser)]
struct RunQuery {
	#[arg(short, long)]
	fmt: Option<String>,
	name: String,
}
