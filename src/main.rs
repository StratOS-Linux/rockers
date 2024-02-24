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

fn install_pkg(pkgmgr: &str, inst_cmd: &str, pkg: &str) {
	println!("{pkgmgr} {inst_cmd} {pkg}");
}

// fn installed_sources(pkg: &str) {
// 	let sources: Vec<&str> = vec!();
// }

fn main() {
	let args = CliArgs::parse();
	let mut install_cmd = "";
	
	// println!("Command: {:?} Pkg: {:?}", args.rockcmd, args.pkgname);

	if pkgmgr_found("/usr/bin/pacman") {
		install_cmd = "-S";
	}

	match args.rockcmd.as_str() {
		"install"|"i" => install_pkg("pacman", &install_cmd, &args.pkgname),
		// "install" => println!("-S"),
		&_ => println!("Invalid"),
	};
}
