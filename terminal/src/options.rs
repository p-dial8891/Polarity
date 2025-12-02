use clap::Parser;
use argfile;

#[derive(Parser)]
struct Flags {
    #[arg(long)]
    host: String,
	#[arg(long)]
	token: String,
	#[arg(long)]
	player: String,
	#[arg(long)]
	tui: String
}

pub fn getHost() -> String {
	let args = argfile::expand_args(
		argfile::parse_fromfile,
		argfile::PREFIX,
	).unwrap();
	let matches = Flags::parse_from(args);
	
	let mut result = String::from("https://");
	result.extend([matches.host]);
	result
}

pub fn getToken() -> String {
	let args = argfile::expand_args(
		argfile::parse_fromfile,
		argfile::PREFIX,
	).unwrap();
	let matches = Flags::parse_from(args);

	matches.token
}

pub fn getPlayerAddress() -> String {
	let args = argfile::expand_args(
		argfile::parse_fromfile,
		argfile::PREFIX,
	).unwrap();
	let matches = Flags::parse_from(args);

	matches.player
}

pub fn getTuiAddress() -> String {
	let args = argfile::expand_args(
		argfile::parse_fromfile,
		argfile::PREFIX,
	).unwrap();
	let matches = Flags::parse_from(args);

	matches.tui
}