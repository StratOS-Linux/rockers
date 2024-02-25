#[allow(dead_code)]
use clap::Parser;
use std::path::Path;
use std::process::{Command, Stdio};

// const BLACK: &str = "\x1B[30m";
const BLUE: &str = "\x1B[34m";
// const YELLOW: &str = "\x1B[33m";
// const GREEN: &str = "\x1B[32m";
// const RED: &str = "\x1B[31m";
const RESET: &str = "\x1B[0m";

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

// fn run_shell_cmd(command: &str) -> String {
// 	let output = Command::new("bash")
// 		.arg("-c")
// 		.arg(command)
// 		.output()
// 		.unwrap_or_else(|e| {
// 			eprintln!("Unable to run cmd: {}", e);
// 			exit(1);
// 		});
// 	return String::from_utf8_lossy(&output.stdout).trim().to_string();
// }

// fn pkg_install_src(pkg: &str) -> Vec<&'static str> {
// 	let mut sources: Vec<&str> = vec!();
	
// }

fn installed_sources() -> Vec<&'static str> {
	let mut sources: Vec<&str> = vec!();
	if pkgmgr_found("/usr/bin/pacman") { sources.push("pacman"); }
	if pkgmgr_found("/usr/bin/yay")    { sources.push("yay"); }
	if pkgmgr_found("/usr/bin/apt")    { sources.push("apt"); }
	if pkgmgr_found("/usr/bin/flatpak")    { sources.push("apt"); }
	return sources;
}

fn install_pkg(pkgmgr: &str, inst_cmd: &str, pkg: &str) {
	println!("{pkgmgr} {inst_cmd} {pkg}"); // TODO
}

fn info_pkg(pkgmgr: &str, info_cmd: &str, pkg: &str) {
	if pkgmgr=="pacman" {
		let output = Command::new(pkgmgr)
			.args([info_cmd, pkg, " | grep -i Validated | awk '{print $4}"])
			.stdout(Stdio::piped())
			.output()
			.unwrap();
		let result = String::from_utf8(output.stdout).unwrap();
		for line in result.lines() {
			println!("{}{RESET}", line);
		}
	}
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
			println!("{BLUE}{}{RESET}", line);
 		}
		else if !line.contains("    ") {
			println!("{}", line);
		}
	}
}

fn main() {
	let args = CliArgs::parse();
	let mut install_cmd = "";
	let mut search_cmd = "";
	let mut info_cmd = "";
	
	println!("Package managers detected: {:?}", installed_sources());
	// println!("Command: {:?} Pkg: {:?}", args.rockcmd, args.pkgname);

	if pkgmgr_found("/usr/bin/pacman") {
		install_cmd = "-S";
		search_cmd = "-Ss";
		info_cmd = "-Qi"
	}

	match args.rockcmd.as_str() {
		"install" | "i" => {
			search_pkg("pacman", &search_cmd, &args.pkgname);
			install_pkg("pacman", &install_cmd, &args.pkgname);
		},
		"search"  | "s" => search_pkg("pacman", &search_cmd, &args.pkgname),
		"info"    | "I" => info_pkg("pacman", &info_cmd, &args.pkgname),
		&_ => println!("Invalid"),
	};

}
