use std::path::Path;
use std::env;
use std::process::{Command, Stdio};
use std::io;
use std::io::{BufReader, BufRead};

// const BLACK: &str = "\x1B[30m";
const VIOLET: &str = "\x1B[35m";
const BLUE: &str = "\x1B[34m";
const YELLOW: &str = "\x1B[33m";
const GREEN: &str = "\x1B[32m";
const RED: &str = "\x1B[31m";
const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m\x1B[37m";
const UNDERLINE: &str = "\x1B[1m\x1B[4m";
const ITALIC: &str = "\x1B[3m\x1B[37m";

fn banner() {
	let s = format!(r#"
{BOLD}Usage{RESET}: {RED}rock{RESET} {YELLOW}[function] [flag] <input>{RESET}                                                          

{BOLD}functions{RESET}:
    {UNDERLINE}install{RESET}: Install package(s) - Prompts user to respond with 
             the number(s) associated with the desired package(s).
             
    {UNDERLINE}remove{RESET}:  Uninstall package(s) - Prompts user to respond with
             the number(s) associated with the desired package(s).

    {UNDERLINE}info{RESET}: Provide information about the package.
             
    {UNDERLINE}search{RESET}:  Search for package(s) - Does not have a second prompt.
    
    {UNDERLINE}update{RESET}:  Updates all packages accessible to the wrapper - does
             not accept <input>, instead use install to update 
             individual packages. Has a confirmation prompt.

    {UNDERLINE}cleanup{RESET}: Attempts to repair broken dependencies and remove any
             unused packages. Does not accept <input>, but has 
             a confirmation prompt.

{BOLD}flags{RESET}: 
    {UNDERLINE}--help{RESET}/{UNDERLINE}-h{RESET}: Display this page
    
{BOLD}input{RESET}: 
    Provide a package name or description.

{BOLD}Example execution:{RESET}
    $ {ITALIC}rock install foobar{RESET}
    Found packages matching 'foobar':

    [{GREEN}0{RESET}]: pyfoobar ({GREEN}apt{RESET})
    [{GREEN}1{RESET}]: foobarshell ({GREEN}apt{RESET})
    [{YELLOW}2{RESET}]: foobar ({YELLOW}flatpak{RESET})
    [{BLUE}3{RESET}]: foobar ({BLUE}pacman{RESET})
    [{VIOLET}4{RESET}]: foobar-bin ({VIOLET}yay{RESET})
    [{VIOLET}5{RESET}]: foobar-theme ({VIOLET}yay{RESET})
    [{RED}6{RESET}]: foobar-web ({RED}snap{RESET})

    Select which package to install [0-5]: 3 4 5
    Selecting '{VIOLET}foobar-web{RESET}' from package manager '{VIOLET}snap{RESET}'
    Selecting '{VIOLET}foobar-bin{RESET}' from package manager '{VIOLET}yay{RESET}'
    Selecting '{VIOLET}foobar-theme{RESET}' from package manager '{VIOLET}yay{RESET}'
    Are you sure? (y/N)
    [...]

rock 0.1.3
A package manager wrapper for StratOS
Developed by Magitian <magitian@duck.com> & ZeStig <o0vckutt@duck.com> for StratOS
"#);
	println!("{}", s);
}

fn pkgmgr_found(p: &str) -> bool {
	if Path::new(p).is_file() {
		return true;
	}
	return false;
}

fn installed_sources() -> Vec<&'static str> {
	let mut sources: Vec<&str> = vec!();
	if pkgmgr_found("/usr/bin/pacman") { sources.push("pacman"); }
	if pkgmgr_found("/usr/bin/yay")    { sources.push("yay"); }
	if pkgmgr_found("/usr/bin/apt")    { sources.push("apt"); }
	if pkgmgr_found("/usr/bin/flatpak")    { sources.push("apt"); }
	return sources;
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

fn update_pkg(pkgmgr: &str, update_cmd: &str) {
    println!("\n{ITALIC}Updating packages {RESET}");

    let mut child = Command::new("sh")
        .args(["-c", &format!("sudo {} {}", pkgmgr, update_cmd)])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{line}");
            }
        }
    }

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
    }
}

fn remove_pkg(pkgmgr: &str, remove_cmd: &str, pkg: &str) {
	println!("\n{ITALIC}Removing packages matching '{}{RESET}'", pkg);
    let mut child = Command::new("sh")
        .args(["-c", &format!("sudo {} {}", pkgmgr, remove_cmd)])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{line}");
            }
        }
    }

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
    }
}

fn cleanup_pkg(pkgmgr: &str, cleanup_cmd: &str) {
	println!("\n{ITALIC}Removing unused packages.{RESET}");
	let output = Command::new("sh")
		.args(["-c", &format!("sudo {} {}", pkgmgr, cleanup_cmd)])
		.stdout(Stdio::piped())
		.output()
		.unwrap();
	let result = String::from_utf8(output.stdout).unwrap();
	for line in result.lines() {
		println!("{line}{RESET}");
	}
}

fn install_pkg(pkgmgr: &str, inst_cmd: &str, pkg: &str) {
    let mut child = Command::new("sh")
        .args(["-c", &format!("sudo {} {} {}", pkgmgr, inst_cmd, pkg)])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}{RESET}", line);
            }
        }
    }

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        eprintln!("Command failed with exit code: {}", status);
    }
}

fn display_pkg(pkgmgr: &str, search_cmd: &str, pkg: &str) {
	println!("\n{ITALIC}Found packages matching '{}{RESET}':", pkg);
	let mut index = 1;
	let output = Command::new(pkgmgr)
		.args([search_cmd, pkg])
		.stdout(Stdio::piped())
		.output()
		.unwrap();
	let result = String::from_utf8(output.stdout).unwrap();

	for line in result.lines() {
		let line = &line.replace("extra/", "");
		if pkgmgr=="pacman" {
			if line.contains("[installed]") {
				println!("[{RED}{}{RESET}]: {GREEN}{}{RESET} [{BLUE}{}{RESET}]", index, line.replace("[installed]", ""), "pacman");
				index += 1;
			}
			else if !line.contains("    ") {
				println!("[{BLUE}{}{RESET}]: {}", index, line);
				index += 1;
			}
 		}
	}
}

fn search_pkg(pkgmgr: &str, search_cmd: &str, pkg: &str) -> String {
	println!("\n{ITALIC}Found packages matching '{}{RESET}':", pkg);
	let mut input_pkg_no: String = String::new();
	let mut index = 1;
	let output = Command::new(pkgmgr)
		.args([search_cmd, pkg])
		.stdout(Stdio::piped())
		.output()
		.unwrap();
	let result = String::from_utf8(output.stdout).unwrap();

	for line in result.lines() {
		let line = &line.replace("extra/", "");
		if pkgmgr=="pacman" {
			if line.contains("[installed]") {
				println!("[{RED}{}{RESET}]: {GREEN}{}{RESET} [{BLUE}{}{RESET}]", index, line.replace("[installed]", ""), "pacman");
				index += 1;
			}
			else if !line.contains("    ") {
				println!("[{BLUE}{}{RESET}]: {}", index, line);
				index += 1;
			}
 		}
	}

	println!("{ITALIC}Select package [1-{}]: {RESET}", index-1);
	io::stdin().read_line(&mut input_pkg_no).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_no.trim().parse().expect("Cannot convert to integer.");

	index=1;
	for line in result.lines() {
		let line = &line.replace("extra/", "");
		if pkgmgr=="pacman" {
			if line.contains("[installed]") || !line.contains("   ") {
				index += 1;
				
				if index==input_pkg_num+1 {
					if let Some(pkg_name) = line.split_whitespace().next() {
						// println!("{} => {:?}", index-1, pkg_name);
						return pkg_name.to_string();
					}
				}
			}
 		}
	}
	String::new() // default return item if no match found
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut rockcmd = "";
	let mut pkgname = ""; // to handle cases where a pkg name is not required

	if args.len() == 1  {
		banner();
	}

	else if args.len() == 2 {
		rockcmd = &args[1];
	} 
	else if args.len() == 3 {
		rockcmd = &args[1];
		pkgname = &args[2];
	}
	let mut install_cmd = "";
	let mut search_cmd = "";
	let mut search_local_cmd = "";
	let mut info_cmd = "";
	let mut update_cmd = "";
	let mut remove_cmd = "";
	let mut cleanup_cmd = "";
	let mut pkgmgr = "";

	println!("Package managers detected: {:?}", installed_sources());

	if pkgmgr_found("/usr/bin/pacman") {
		pkgmgr = "pacman";
		install_cmd = "-S --noconfirm";
		search_cmd = "-Ss";
		search_local_cmd = "-Qs";
		info_cmd = "-Si";
		update_cmd = "-Syu --noconfirm";
		remove_cmd = "-Rns --noconfirm";
		cleanup_cmd = "-Rns --noconfirm $(pacman -Qtdq)";
	}

	match rockcmd {
		"install" | "i" => {
			let selected_pkg = search_pkg(pkgmgr, &search_cmd, &pkgname);
			install_pkg(pkgmgr, &install_cmd, &selected_pkg);
		},
		
		"search"   | "s" => {
			display_pkg(pkgmgr, &search_cmd, &pkgname);
		},
		
		"info"     | "I" => info_pkg(pkgmgr, &info_cmd, &pkgname),
		"update"   | "u" => update_pkg(pkgmgr, &update_cmd),
		"remove"   | "r" => {
			let selected_pkg = search_pkg(pkgmgr, &search_local_cmd, &pkgname);
			remove_pkg(pkgmgr, &remove_cmd, &selected_pkg);
		},
		
		"clean"    | "c" => cleanup_pkg(pkgmgr, &cleanup_cmd),
		"-h"  | "--help" => banner(),
		&_ => print!("{BOLD}Invalid Usage.{RESET} Consult {ITALIC}rock --help{RESET} for more information."),
	};
}
