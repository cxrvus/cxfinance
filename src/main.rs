use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

mod config;
mod import;
mod pattern;
mod transaction;

fn main() {
	match execute() {
		Ok(_) => {},
		Err(e) => println!("<!>\n{:?}", e)
	}
}

fn execute() -> Result<()> {
	let res = Cli::parse();
	match res {
		Cli::Import(args) => {
			import::import_transactions(args.path)
		}
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
