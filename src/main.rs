use clap::{Command, Arg};

fn main() {
	let cli = Command::new("cargo-tarantula")
		.arg(Arg::new("manifest-path").long("manifest-path").help("Path to Cargo.toml"));

	let matches = cli.get_matches();

	let manifest_path: Option<&String> = matches.get_one("manifest-path");
}
