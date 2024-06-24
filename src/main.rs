#![allow(dead_code)]
#![allow(unused)]
use std::path::Path;
use std::{env, process::{Command, Stdio}};
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;

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
const CYAN: &str = "\x1B[36m";
const HIGHLIGHT: &str = "\x1B[1;37;48;2;165;42;42m";

struct SearchOutput {
	pkgmgr: String,
	pkgname: String,
}

struct PkgResult {
	res: String,
	pos: Vec<i32>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Pkgmgrs {
	name: Vec<String>,
	install_cmd: HashMap<String, String>,
	search_cmd:  HashMap<String, String>,
	search_local_cmd:  HashMap<String, String>,
	info_cmd:  HashMap<String, String>,
	update_cmd: HashMap<String, String>,
	remove_cmd:  HashMap<String, String>,
	cleanup_cmd:  HashMap<String, String>,
}

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
	let new_p = String::from("/usr/bin/") + p;
	if Path::new(new_p.as_str()).is_file() {
		return true;
	}
	false
}

fn installed_sources() -> Vec<&'static str> {
	let mut sources: Vec<&str> = vec!();
	if pkgmgr_found("pacman") { sources.push("pacman"); }
	if pkgmgr_found("yay")    { sources.push("yay"); }
	if pkgmgr_found("apt")    { sources.push("apt"); }
	if pkgmgr_found("flatpak")    { sources.push("flatpak"); }
	sources
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

fn update_pkg(pm: Pkgmgrs) {
    println!("\n{ITALIC}Updating packages {RESET}");

	let mut output = Command::new("echo") // placeholder for scope purposes.
		.arg("")
		.stdout(Stdio::piped())
		.spawn()
		.expect("");
	
	for i in 0..pm.name.len() {
		if pm.name[i] == "pacman" || pm.name[i] == "apt" { // run with sudo.
			output = Command::new("sh")
				.args(["-c", &format!("sudo {} {}", &pm.name[i], &pm.update_cmd[&pm.name[i]])])
				.stdout(Stdio::piped())
				.spawn()
				.expect("Failed to start command");
		}
		else if pm.name[i] == "flatpak" {
			output = Command::new(&pm.name[i])
				.args([&pm.update_cmd[&pm.name[i]], "--noninteractive"])
				.stdout(Stdio::piped())
				.spawn()
				.expect("");
		}
		if let Some(stdout) = output.stdout.take() {
			let reader = BufReader::new(stdout);
			for line in reader.lines() {
				if let Ok(line) = line {
					println!("{line}");
				}
			}
		}
	}
}


fn cleanup_pkg(pm: Pkgmgrs) {
	println!("\n{ITALIC}Removing unused packages.{RESET}");
	println!("{} {}", pm.name[0], pm.cleanup_cmd[&pm.name[0]]);
	let output = Command::new("sh")
		.args(["-c", &format!("sudo {} {}", &pm.name[0], pm.cleanup_cmd[&pm.name[0]])])
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

fn remove_pkg(pkgmgr: &str, remove_cmd: &str, pkg: &str) {
	if pkg==String::new() {
		println!("{ITALIC}No matching packages installed.{RESET}");
		return;
	}
	println!("\n{ITALIC}Removing packages matching '{}{RESET}'", pkg);
    let mut child = Command::new("sh")
        .args(["-c", &format!("sudo {} {} {}", pkgmgr, remove_cmd, pkg)])
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

fn display_pkg(pm: Pkgmgrs, pkg: &str) -> PkgResult {
	println!("\n{ITALIC}Found packages matching '{}{RESET}':", pkg);
	let mut index = 1;
	let (mut pacman_idx, mut yay_idx, mut flatpak_idx) = (-1, -1, -1);
	let mut result = String::new();
	let mut res_string = String::new();
	for i in 0..pm.name.len() {
		// println!("{RED}Pkgmgr: {}{RESET}", pm.name[i]);
		let mut output = Command::new("echo")
			.stdout(Stdio::piped())
			.spawn()
			.expect("ERROR");
		if pm.name[i] == "flatpak" {
			output = Command::new(pm.name[i].clone())
				.args([&pm.search_cmd[&pm.name[i]], pkg, "--columns=application"])
				.stdout(Stdio::piped())
				.spawn()
				.expect("");
		}
		else {
			output = Command::new(pm.name[i].clone())
				.args([&pm.search_cmd[&pm.name[i]], pkg])
				.stdout(Stdio::piped())
				.spawn()
				.expect("");
		}
		// result = String::from_utf8(output.stdout).unwrap();

		if let Some(stdout) = output.stdout.take() {
			let reader = BufReader::new(stdout);
			for line in reader.lines() {
				let line = &line.unwrap().replace("extra/", "").replace("aur/", "").replace("core/", "");

				if pm.name[i] == "pacman" {
					if line.contains("[installed]") {

						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}{}{RESET}]{RESET}", index, line.replace("[installed]", ""), "pacman");
						// res_string += &(line.to_owned() + &String::from('\n'));
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index;
						index += 1;
					}
					else if !line.contains("    ") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{BLUE}{}{RESET}]: {}", index, line);
						// res_string += &format!("{index} PACMAN\n").to_string();
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index;
						index += 1;
					}
 				}

				else if pm.name[i] == "yay" {
					if line.contains("(Installed)") {
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{VIOLET}{}{RESET}]{RESET}", index, line.replace("(Installed)", ""), "yay");
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						res_string += &line[..fwi];
						res_string += "\n";
						yay_idx = index;
						index += 1;
					}
					else if !line.contains("    ") {
						println!("[{VIOLET}{}{RESET}]: {}", index, line);
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						res_string += &line[..fwi];
						res_string += "\n";
						yay_idx = index;
						index += 1;
					}
				}

				else if pm.name[i]=="flatpak" {
					println!("[{GREEN}{}{RESET}]: {}", index, line);
					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					res_string += &line[..fwi];
					res_string += "\n";
					flatpak_idx = index;
					index += 1;
				}
			}
		}
	}
	
	// println!("{RED}{}{RESET}", res_string);
	println!("{res_string}");
	println!("Pacman: {pacman_idx}, Yay: {yay_idx}, Flatpak: {flatpak_idx}");
	PkgResult {
		res: res_string,
		pos: vec![pacman_idx, yay_idx, flatpak_idx],
	}
}

fn list_pkg(pm: Pkgmgrs, pkg: &str) -> PkgResult {
	let mut index = 1;
	let (mut pacman_idx, mut yay_idx, mut flatpak_idx) = (-1, -1, -1);
	let mut result = String::new();
	let mut res_string = String::new();
	for i in 0..pm.name.len() {
		let mut output = Command::new("echo").stdout(Stdio::piped()).output().unwrap();
		if pm.name[i] == "flatpak" {
			output = Command::new(pm.name[i].clone())
				.args([&pm.search_cmd[&pm.name[i]], pkg, "--columns=application"])
				.stdout(Stdio::piped())
				.output()
				.unwrap();
		}
		else {
			output = Command::new(pm.name[i].clone())
				.args([&pm.search_cmd[&pm.name[i]], pkg])
				.stdout(Stdio::piped())
				.output()
				.unwrap();
		}
		result = String::from_utf8(output.stdout).unwrap();

		for line in result.lines() {
			let line = &line.replace("extra/", "").replace("aur/", "").replace("core/", "");

			if pm.name[i] == "pacman" {
				if line.contains("[installed]") {

					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					res_string += &line[..fwi];
					res_string += "\n";
					pacman_idx = index;
					index += 1;
				}
				else if !line.contains("    ") {
					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					// res_string += &format!("{index} PACMAN\n").to_string();
					res_string += &line[..fwi];
					res_string += "\n";
					pacman_idx = index;
					index += 1;
				}
 			}

			else if pm.name[i] == "yay" {
				if line.contains("(Installed)") {
					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					res_string += &line[..fwi];
					res_string += "\n";
					yay_idx = index;
					index += 1;
				}
				else if !line.contains("    ") {
					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					res_string += &line[..fwi];
					res_string += "\n";
					yay_idx = index;
					index += 1;
				}
			}

			else if pm.name[i]=="flatpak" {
				let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
				res_string += &line[..fwi];
				res_string += "\n";
				flatpak_idx = index;
				index += 1;
			}
		}
	}

	PkgResult {
		res: res_string,
		pos: vec![pacman_idx, yay_idx, flatpak_idx],
	}
}

fn detect_pkg_mgr(pm: Pkgmgrs, pkg: &str, pkgno: i32) -> &str {
	let q = list_pkg(pm, pkg);
	if 1 < pkgno && pkgno <= q.pos[0] {
		"pacman"
	} else if q.pos[0] < pkgno && pkgno <= q.pos[1] {
		"yay"
	} else if q.pos[1] < pkgno && pkgno <= q.pos[2] {
		"flatpak"
	} else {
		""
	}
}

fn search_pkg(pm: Pkgmgrs, pkg: &str) -> SearchOutput {
	let mut search_out = SearchOutput {
		pkgmgr: String::from(""),
		pkgname: String::from("")
	};
	
	let mut input_pkg_no: String = String::new();
	let mut index = 1;

	let pkg_output = display_pkg(pm.clone(), pkg);
	
	println!("{ITALIC}Select package [1-{}]: {RESET}", index-1);
	io::stdin().read_line(&mut input_pkg_no).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_no.trim().parse().expect("Cannot convert to integer.");

	if index<=1 {
		return search_out;
	}

	index=1;
	for line in pkg_output.res.lines() {
		let line = &line.replace("extra/", "");
		if pm.name[0] == "pacman" {
			if line.contains("[installed]") || !line.contains("   ") {
				index += 1;
				
				if index==input_pkg_num+1 {
					if let Some(pkg_name) = line.split_whitespace().next() {
						search_out.pkgname = String::from(pkg_name);
					}
				}
			}
 		}
	}
	search_out
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
	// let mut install_cmd = "";
	// let mut search_cmd = "";
	// let mut search_local_cmd = "";
	// let mut info_cmd = "";
	// let mut update_cmd = "";
	// let mut remove_cmd = "";
	// let mut cleanup_cmd = "";
	// let mut pkgmgr = "";

	println!("Package managers detected: {:?}", installed_sources());

	let mut pm = Pkgmgrs {
		name: Vec::new(),
		install_cmd: HashMap::new(),
		search_cmd: HashMap::new(),
		search_local_cmd: HashMap::new(),
		info_cmd: HashMap::new(),
		update_cmd: HashMap::new(),
		remove_cmd: HashMap::new(),
		cleanup_cmd: HashMap::new(),
	};
	
	if pkgmgr_found("pacman") {
		pm.name.push("pacman".to_string());
		pm.install_cmd.insert(pm.name[0].clone(), "-S".to_string());
		pm.search_cmd.insert(pm.name[0].clone(), "-Ss".to_string());
		pm.search_local_cmd.insert(pm.name[0].clone(), "-Qs".to_string());
		pm.info_cmd.insert(pm.name[0].clone(), "-Si".to_string());
		pm.update_cmd.insert(pm.name[0].clone(), "-Syu".to_string());
		pm.remove_cmd.insert(pm.name[0].clone(), "-Rns".to_string());
		pm.cleanup_cmd.insert(pm.name[0].clone(), "-Rcns $(pacman -Qtdq) --noconfirm".to_string());
	}
	
	if pkgmgr_found("yay") {
		pm.name.push("yay".to_string());
		pm.install_cmd.insert(pm.name[1].clone(), "-Sa".to_string());
		pm.search_cmd.insert(pm.name[1].clone(), "-Ssqa".to_string());
		pm.search_local_cmd.insert(pm.name[1].clone(), "-Qsa".to_string());
		pm.info_cmd.insert(pm.name[1].clone(), "-Sai".to_string());
		pm.update_cmd.insert(pm.name[1].clone(), "-Syu".to_string());
		pm.remove_cmd.insert(pm.name[1].clone(), "-Rns".to_string());
		pm.cleanup_cmd.insert(pm.name[1].clone(), "-Rcns $(yay -Qtdq)".to_string());
	}
	
	if pkgmgr_found("flatpak") {
		pm.name.push("flatpak".to_string());
		pm.install_cmd.insert(pm.name[2].clone(), "install".to_string());
		pm.search_cmd.insert(pm.name[2].clone(), "search".to_string());
		pm.search_local_cmd.insert(pm.name[2].clone(), "search".to_string());
		pm.info_cmd.insert(pm.name[2].clone(), "remote-info".to_string());
		pm.update_cmd.insert(pm.name[2].clone(), "update".to_string());
		pm.remove_cmd.insert(pm.name[2].clone(), "uninstall".to_string());
		pm.cleanup_cmd.insert(pm.name[2].clone(), "uninstall --unused".to_string());
	}

	println!("{RED}Pkg mgr: {}{RESET}", detect_pkg_mgr(pm.clone(), &pkgname, 16)); // 16 is to check if Flatpak's correct.
	println!("{:?}", pm);

	match rockcmd {
		"update"   | "u" => update_pkg(pm),
		"search"   | "s" => {
			let _ = display_pkg(pm, &pkgname);
		},
	 	"clean"    | "c" => cleanup_pkg(pm),
		"-h"  | "--help" => banner(),
		_ => {
			print!("{BOLD}Invalid Usage.{RESET} Consult {ITALIC}rock --help{RESET} for more information.")
		},
	}
	// match rockcmd {
	// 	"install" | "i" => {
	// 		let selected_pkg = search_pkg(pkgmgr, &search_cmd, &pkgname);
	// 		install_pkg(&selected_pkg.pkgmgr, &install_cmd, &selected_pkg.pkgname);
	// 	},
		
	// 	"search"   | "s" => display_pkg(pkgmgr, &search_cmd, &pkgname),
	// 	"info"     | "I" => info_pkg(pkgmgr, &info_cmd, &pkgname),
	// 	"update"   | "u" => update_pkg(pkgmgr, &update_cmd),
	// 	"remove"   | "r" => {
	// 		let selected_pkg = search_pkg(pkgmgr, &search_local_cmd, &pkgname);
	// 		remove_pkg(&selected_pkg.pkgmgr, &remove_cmd, &selected_pkg.pkgname);
	// 	},
		
	// 	"clean"    | "c" => cleanup_pkg(pkgmgr, &cleanup_cmd),
	// 	"-h"  | "--help" => banner(),
	// 	&_ => print!("{BOLD}Invalid Usage.{RESET} Consult {ITALIC}rock --help{RESET} for more information."),
	// };
}
