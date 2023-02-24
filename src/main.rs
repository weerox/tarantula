use std::path::Path;
use clap::{Command, Arg};
use cargo_metadata::Metadata;

fn main() {
	let cli = Command::new("cargo-tarantula")
		.arg(Arg::new("manifest-path").long("manifest-path").help("Path to Cargo.toml"));

	let matches = cli.get_matches();

	let manifest_path = matches.get_one::<String>("manifest-path").map(|s| Path::new(s));

	let metadata = get_metadata(manifest_path);
}

fn get_metadata(manifest_path: Option<&Path>) -> cargo_metadata::Result<Metadata> {
	let mut cmd = cargo_metadata::MetadataCommand::new();

	if let Some(manifest_path) = manifest_path {
		cmd.manifest_path(manifest_path);
	}

	cmd.exec()
}
