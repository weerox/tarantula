use std::io::Read;
use std::fs::File;
use std::path::Path;
use clap::{Command, Arg};
use cargo_metadata::{Metadata, Package, Target};
use quote::ToTokens;

mod resolver;

fn main() {
	let cli = Command::new("cargo-tarantula")
		.arg(Arg::new("manifest-path").long("manifest-path").help("Path to Cargo.toml"));

	let matches = cli.get_matches();

	let manifest_path = matches.get_one::<String>("manifest-path").map(|s| Path::new(s));

	let metadata = if let Ok(metadata) = get_metadata(manifest_path) {
		metadata
	} else {
		println!("Failed to retreive metadata");
		return;
	};

	let package = if let Some(package) = metadata.root_package() {
		package
	} else {
		println!("Failed to get root package");
		return;
	};

	let targets = get_binary_targets(&package);

	for target in targets {
		let mut file = if let Ok(file) = File::open(&target.src_path) {
			file
		} else {
			println!("Failed to read src file for target {}", target.name);
			return;
		};

		let mut content = String::new();
		let result = file.read_to_string(&mut content);
		if result.is_err() {
			println!("Failed to read src file for target {}", target.name);
			return;
		}

		let mut file = if let Ok(file) = syn::parse_file(&content) {
			file
		} else {
			println!("Failed to parse file at {}", &target.src_path);
			return;
		};

		let mut base = target.src_path.clone();
		base.pop();

		resolver::module::resolve_modules(&mut file, base, Vec::new());

		println!("{}", file.to_token_stream());
	}
}

fn get_metadata(manifest_path: Option<&Path>) -> cargo_metadata::Result<Metadata> {
	let mut cmd = cargo_metadata::MetadataCommand::new();

	if let Some(manifest_path) = manifest_path {
		cmd.manifest_path(manifest_path);
	}

	cmd.exec()
}

fn get_binary_targets(package: &Package) -> Vec<Target> {
	let mut targets = package.targets.clone();
	targets.retain(|t| t.is_bin());

	targets
}
