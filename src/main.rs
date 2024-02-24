use clap::Parser;
use std::path::Path;
use std::process::{Command, Stdio};

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

fn installed_sources() -> Vec<&'static str> {
	let mut sources: Vec<&str> = vec!();
	if pkgmgr_found("/usr/bin/pacman") { sources.push("pacman"); }
	if pkgmgr_found("/usr/bin/yay")    { sources.push("yay"); }
	if pkgmgr_found("/usr/bin/apt")    { sources.push("apt"); }
	if pkgmgr_found("/usr/bin/flatpak")    { sources.push("apt"); }
	return sources;
}

fn search_pkg(pkgmgr: &str, search_cmd: &str, pkg: &str) {
	let output = Command::new(pkgmgr)
		.args([search_cmd, pkg])
		.stdout(Stdio::piped())
		.output()
		.unwrap();
	let result = String::from_utf8(output.stdout).unwrap();

	for line in result.lines() {
		if pkgmgr=="pacman" && line.contains("[installed]") {
			print!("\x1B[34m{}\x1B[0m\n", line);
		} else {
			println!("{}", line);
		}
	}
}

fn main() {
	let args = CliArgs::parse();
	let mut install_cmd = "";
	
	println!("Package managers detected: {:?}", installed_sources());
	// println!("Command: {:?} Pkg: {:?}", args.rockcmd, args.pkgname);

	if pkgmgr_found("/usr/bin/pacman") {
		install_cmd = "-S";
	}

	match args.rockcmd.as_str() {
		"install"|"i" => install_pkg("pacman", &install_cmd, &args.pkgname),
		&_ => println!("Invalid"),
	};

	search_pkg("pacman", "-Ss", "hyprland");
}
