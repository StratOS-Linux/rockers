use clap::Parser;
use std::path::Path;

#[derive(Parser)]
struct CliArgs {
	rockcmd: String,
	pkgname: String
}

fn pkgmgr_found(p: &str) -> bool {
	if Path::new(p).is_file() {
		return true;
	}
	return false;
}

fn main() {
	let args = CliArgs::parse();
	println!("Command: {:?} Pkg: {:?}", args.rockcmd, args.pkgname);

	if pkgmgr_found("/usr/bin/pacman") {
		println!("Pacman found.");
	}

	match args.rockcmd.as_str() {
		"i" => println!("-S"),
		"install" => println!("-S"),
		&_ => println!("Invalid"),
	};
}
