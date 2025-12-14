use std::path::Path;
use std::process::exit;
use std::{env, process::{Command, Stdio}};
use std::io::{self, Write, BufRead, BufReader};
use std::collections::HashMap;

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
// const CYAN: &str = "\x1B[36m";
const HIGHLIGHT: &str = "\x1B[1;37;48;2;165;42;42m";

struct PkgResult {
	res: String,
	pos: Vec<i32>,
}

#[derive(Debug)]
#[derive(Clone)]

// struct Pkgmgrs {{{
struct Pkgmgrs {
	name: Vec<String>,
	install_cmd: HashMap<String, String>,
	search_cmd:  HashMap<String, String>,
	search_local_cmd:  HashMap<String, String>,
	info_cmd:  HashMap<String, String>,
	inst_info_cmd:  HashMap<String, String>,
	update_cmd: HashMap<String, String>,
	remove_cmd:  HashMap<String, String>,
	cleanup_cmd:  HashMap<String, String>,
}
// }}}

// banner {{{
fn banner() {
	let s = format!(r#"
{BOLD}Usage{RESET}: {RED}rock{RESET} {YELLOW}[function] [flag] <input>{RESET}                                                          

{BOLD}Functions{RESET}:
    {UNDERLINE}install (i){RESET}: Install a package - Pick the number associated with the desired package.
             
    {UNDERLINE}remove (r){RESET}:  Uninstall package(s) - Pick the number associated with the desired package. Removes related unnecessary dependencies.

    {UNDERLINE}info (if){RESET}: Retrieve remote information about the package. Fetches information from the relevant repo.
             
    {UNDERLINE}install-info (iif){RESET}: Display local information about the package. Fetches information from the installed package.

    {UNDERLINE}search (s){RESET}:  Search for {ITALIC}package{RESET} across configured package managers.
    
    {UNDERLINE}update (u){RESET}:  Update all packages across package managers. Doesn't take secondary arguments.

    {UNDERLINE}cleanup (c){RESET}: Remove any unused packages. Does not accept secondary arguments.

{BOLD}Flags{RESET}: 
    {UNDERLINE}--help{RESET}/{UNDERLINE}-h{RESET}: Display this help page.
    
{BOLD}Input{RESET}: 
    Provide a package name or description.

{BOLD}Example execution:{RESET}
    $ {ITALIC}rock install kitty{RESET}
    Finding packages matching 'kitty':

    [{HIGHLIGHT}1{RESET}]: {ITALIC}kitty{RESET} [{BLUE}pacman{RESET}]
    [{HIGHLIGHT}2{RESET}]: {ITALIC}kitty-shell-integration{RESET} [{BLUE}pacman{RESET}]
    [{BLUE}3{RESET}]: hyperkitty 
    [{VIOLET}4{RESET}]: kitty-git
    [{VIOLET}5{RESET}]: kitty-terminfo-git
    [{GREEN}6{RESET}]: com.daidouji.oneko
    [{YELLOW}7{RESET}]: kitty
    [{YELLOW}8{RESET}]: hyperkitty

    Select package [1-8]: 3
    {ITALIC}Installing package{RESET} {HIGHLIGHT}hyperkitty{RESET}.
    [...]

Rockers 0.2
A package manager wrapper for {BOLD}StratOS{RESET} (https://stratos-linux.org/)
Developed by Magitian <magitian@duck.com> & ZeStig <o0vckutt@duck.com> 
"#);
	println!("{}", s);
}

// }}}

fn print_pacman() {
	println!("\n{BOLD}{ITALIC}>>>{BLUE} Arch Linux repos 󰣇 {RESET}\n");
}

fn print_paru() {
	println!("\n{BOLD}{ITALIC}>>>{VIOLET} Arch User Repository 󰣇 {RESET}\n");
}

fn print_flatpak() {
	println!("\n{BOLD}{ITALIC}>>>{GREEN} Flathub  {RESET}\n");
}

fn print_apt() {
	println!("\n{BOLD}{ITALIC}>>>{YELLOW} Ubuntu repos   {RESET}\n");
}

fn pkgmgr_found(p: &str) -> bool {
	if Path::new(p).is_file() { return true; }
	false
}

// adjust_idx {{{
fn adjust_idx(a: i32, b: i32, c: i32, d: i32) {
	if a==-1 && b==-1 && c==-1 && d==-1 { // no matches at all
		print!("{ITALIC}No matching packages found.{RESET}");
		exit(-1);
	} else if b==-1 && c==-1 && d==-1 { // only pacman
		print!("\n{ITALIC}Select package [1-{}]: {RESET}", a);
		let _ = io::stdout().flush();
	} else if c==-1 && d==-1{ // only pacman, paru
		print!("\n{ITALIC}Select package [1-{}]: {RESET}", b);
		let _ = io::stdout().flush();
    } else if d==-1 { // only pacman, paru, flatpak
        print!("{ITALIC}\nSelect package [1-{}]: {RESET}", c);
        let _ = io::stdout().flush();
	} else { // all 4 PMs
		print!("{ITALIC}\nSelect package [1-{}]: {RESET}", d);
		let _ = io::stdout().flush();
	}
}
// }}}

// inst_info_pkg {{{
fn inst_info_pkg(pm: &Pkgmgrs, pkg: &str) {
	let x = display_local_pkg(&pm, pkg);
	let mut input_pkg_str = String::new();
	
	adjust_idx(x.pos[0], x.pos[1], x.pos[2], x.pos[3]);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let mut info_pkgmgr = "";
	if 1 <= input_pkg_num && input_pkg_num <= x.pos[0] { info_pkgmgr = "pacman"; }
	else if x.pos[0] < input_pkg_num && input_pkg_num <= x.pos[1] { info_pkgmgr = "paru"; }
	else if x.pos[1] < input_pkg_num && input_pkg_num <= x.pos[2] { info_pkgmgr = "flatpak"; }
	else if x.pos[2] < input_pkg_num && input_pkg_num <= x.pos[3] { info_pkgmgr = "nala"; }
	else if input_pkg_num > x.pos[3] {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	let tmp: Vec<&str> = x.res.lines().collect();
	let info_pkgname;
	if input_pkg_num > 0 {
		info_pkgname = tmp[(input_pkg_num as usize) - 1];
		println!();
		println!("{ITALIC}Info for package {RESET}{HIGHLIGHT}{}{RESET}.", info_pkgname);
	} else {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	
	let mut output = Command::new(&info_pkgmgr)
		.args([&pm.inst_info_cmd[info_pkgmgr], info_pkgname])
		.stdout(Stdio::piped())
		.spawn()
		.expect("No such pkg");
	if let Some(stdout) = output.stdout.take() {
		let reader = BufReader::new(stdout);
		for line in reader.lines() {
			if let Ok(line) = line {
				println!("{line}");
			}
		}
	}
}
// }}}

// info_pkg {{{
fn info_pkg(pm: &Pkgmgrs, pkg: &str) {
	let x = display_pkg(&pm, pkg);
	let mut input_pkg_str = String::new();
	
	adjust_idx(x.pos[0], x.pos[1], x.pos[2], x.pos[3]);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let mut info_pkgmgr = "";
	if 1 <= input_pkg_num && input_pkg_num <= x.pos[0] { info_pkgmgr = "pacman"; }
	else if x.pos[0] < input_pkg_num && input_pkg_num <= x.pos[1] { info_pkgmgr = "paru"; }
	else if x.pos[1] < input_pkg_num && input_pkg_num <= x.pos[2] { info_pkgmgr = "flatpak"; }
    else if x.pos[2] < input_pkg_num && input_pkg_num <= x.pos[3] { info_pkgmgr = "nala"; }
	else if input_pkg_num > x.pos[3] {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	let tmp: Vec<&str> = x.res.lines().collect();
	let info_pkgname;
	if input_pkg_num > 0 {
		info_pkgname = tmp[(input_pkg_num as usize) - 1];
		println!();
		println!("{ITALIC}Fetching info for package {RESET}{HIGHLIGHT}{}{RESET}.", info_pkgname);
	} else {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	let mut output;
	
	if info_pkgmgr == "flatpak" {
		output = Command::new(&info_pkgmgr)
			.args([&pm.info_cmd[info_pkgmgr], "flathub", info_pkgname])
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else {
		output = Command::new(&info_pkgmgr).args([&pm.info_cmd[info_pkgmgr], info_pkgname])
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
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
// }}}

// update_pkg {{{
fn update_pkg(pm: &Pkgmgrs) {
    println!("\n{ITALIC}Updating packages {RESET}");

	let mut output = Command::new("echo").arg("").stdout(Stdio::piped()).spawn().expect("");
	
	for i in 0..pm.name.len() {
		let noc = match pm.name[i].as_str() {
			"pacman" | "paru" => "--color=always",
			"nala" => "--assume-yes",
			_ => "--assumeyes"
		};
		if pm.name[i] == "pacman" || pm.name[i] == "nala" { // run with sudo.
			if pm.name[i] == "pacman" {print_pacman();} // TODO
			else {print_apt();}
			output = Command::new("sudo").arg(&pm.name[i]).arg(&pm.update_cmd[&pm.name[i]]).arg(noc)
				.stdout(Stdio::piped()).spawn().expect("Failed to start command");
		}
		else if pm.name[i] == "paru" {
			print_paru();
			output = Command::new(&pm.name[i]).arg(&pm.update_cmd[&pm.name[i]]).arg(noc)
				.stdout(Stdio::piped()).spawn().expect("Failed to start command");
		}
		else if pm.name[i] == "flatpak" {
			print_flatpak();
			output = Command::new(&pm.name[i])
				.arg(&pm.update_cmd[&pm.name[i]]).arg(noc)
				.stdout(Stdio::piped()).spawn().expect("");
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
// }}}

// cleanup_pkg {{{
fn cleanup_pkg(pm: &Pkgmgrs) {
	println!("{ITALIC}Finding unused packages:{RESET}");
	let mut output = Command::new("echo").arg("").stdout(Stdio::piped()).spawn().expect("");
	for i in 0..pm.name.len() {
		if pm.name[i] == "nala" {
			print_apt();
			output = Command::new("sh")
				.args(["-c", &format!("sudo {} {} --assume-yes", &pm.name[i], &pm.cleanup_cmd[&pm.name[i]])])
				.stdout(Stdio::piped()).spawn().expect("No such pkg");
		}
		else if pm.name[i] == "pacman" {
			print_pacman();
			let mut unused_pkgs: Vec<String> = Vec::new();
			let mut unused_pkgs_str = String::new();
			let mut output1 = Command::new(&pm.name[i]).arg("-Qtdq").stdout(Stdio::piped()).spawn().expect("");

			if let Some(stdout) = output1.stdout.take() {
				let reader = BufReader::new(stdout);
				for line in reader.lines() {
					let tmp = line.unwrap();
					unused_pkgs.push(tmp);
				}
				for i in 0..unused_pkgs.len() {
					unused_pkgs_str += &unused_pkgs[i];
					unused_pkgs_str += " ";
				}
			}

			_ = Command::new("sh").args(["-c", &format!("sudo rm -f /var/lib/pacman/db.lck")]).spawn();
			if unused_pkgs_str != "" {
				output = Command::new("sh").args(["-c", &format!("sudo {} {} {} --noconfirm", &pm.name[i], &pm.cleanup_cmd[&pm.name[i]], unused_pkgs_str)])
					.stdout(Stdio::piped()).spawn().expect("No such pkg");
			}
			if let Some(stdout) = output.stdout.take() {
				let reader = BufReader::new(stdout);
				for line in reader.lines() {
					if let Ok(line) = line {
						if unused_pkgs.len() != 0 { println!("{line}"); }
					}
				}
			}
		}
		else if pm.name[i] == "flatpak" { // no need to check for paru.
			print_flatpak();
			output = Command::new("sh")
				.args(["-c", &format!("{} {} {} {}", &pm.name[i], &pm.cleanup_cmd[&pm.name[i]], "--unused", "--assumeyes")])
				.stdout(Stdio::piped()).spawn().expect("No such pkg");
		}
	}
	
	if let Some(stdout) = output.stdout.take() {
		let reader = BufReader::new(stdout);
		for line in reader.lines() {
			if let Ok(line) = line {
				if !(line.contains("Nothing unused to uninstall") || line.contains("no targets specified") || line.contains("Nothing for Nala to remove")) {
					println!("{line}");
				}
			}
		}
	}
}
// }}}

// install_pkg {{{
fn install_pkg(pm: &Pkgmgrs, pkg: &str) {
	let x = display_pkg(&pm, pkg);
	let mut input_pkg_str = String::new();
	adjust_idx(x.pos[0], x.pos[1], x.pos[2], x.pos[3]);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	// don't query repos once again.
	let mut inst_pkgmgr = "";
	if 1 <= input_pkg_num && input_pkg_num <= x.pos[0] { inst_pkgmgr = "pacman"; }
	else if x.pos[0] < input_pkg_num && input_pkg_num <= x.pos[1] { inst_pkgmgr = "paru"; }
	else if x.pos[1] < input_pkg_num && input_pkg_num <= x.pos[2] { inst_pkgmgr = "flatpak"; }
	else if x.pos[2] < input_pkg_num && input_pkg_num <= x.pos[3] { inst_pkgmgr = "nala"; }
	else if input_pkg_num > x.pos[3] {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	let tmp: Vec<&str> = x.res.lines().collect();
	let inst_pkgname: &str;
	if input_pkg_num > 0 {
		inst_pkgname = tmp[(input_pkg_num as usize) - 1];
		println!();
		println!("{ITALIC}Installing package {RESET}{HIGHLIGHT}{}{RESET}.", inst_pkgname);
	} else {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}

	let noc = match inst_pkgmgr {
		"pacman" | "paru" => "--noconfirm",
		"apt" | "nala" => "-y",
		_ => "--assumeyes"
	};
	
	let mut output = Command::new("echo").arg("").stdout(Stdio::piped()).spawn().expect("");
	if inst_pkgmgr == "pacman" {
		output = Command::new("sudo").arg(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc).arg("--color=always")
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "paru" {
		output = Command::new(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc).arg("--color=always")
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "nala" {
		output = Command::new("sudo").arg(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "flatpak" {
		output = Command::new(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
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
// }}}

// remove_pkg {{{
fn remove_pkg(pm: &Pkgmgrs, pkg: &str) {
	let x = display_local_pkg(&pm, pkg);
	let mut input_pkg_str = String::new();
	adjust_idx(x.pos[0], x.pos[1], x.pos[2], x.pos[3]);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let mut rm_pkgmgr = "";
	if 1 <= input_pkg_num && input_pkg_num <= x.pos[0] { rm_pkgmgr = "pacman"; }
	else if x.pos[0] < input_pkg_num && input_pkg_num <= x.pos[1] { rm_pkgmgr = "paru"; }
	else if x.pos[1] < input_pkg_num && input_pkg_num <= x.pos[2] { rm_pkgmgr = "flatpak"; }
	else if x.pos[2] < input_pkg_num && input_pkg_num <= x.pos[3] { rm_pkgmgr = "nala"; }
	else if input_pkg_num > x.pos[3] {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}
	let tmp: Vec<&str> = x.res.lines().collect();
	let rm_pkgname: &str;
	if input_pkg_num > 0 {
		rm_pkgname = tmp[(input_pkg_num as usize) - 1];
		println!();
		println!("{ITALIC}Removing package {RESET}{HIGHLIGHT}{}{RESET}.", rm_pkgname);
	} else {
		println!("{RED}ERROR: {RESET}{UNDERLINE}Enter a valid number.{RESET}");
		exit(-1);
	}

	let mut output = Command::new("echo").arg("").stdout(Stdio::piped()).spawn().expect("");
	if rm_pkgmgr == "pacman" {
		print_pacman();
		output = Command::new("sh").args(["-c", &format!("sudo {} {} {}", &rm_pkgmgr, &pm.remove_cmd[rm_pkgmgr], rm_pkgname)]) // ask for user confirmation for removal.
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if rm_pkgmgr == "nala" {
		output = Command::new("sh").args(["-c", &format!("sudo {} {} {} {}", &rm_pkgmgr, &pm.remove_cmd[rm_pkgmgr], rm_pkgname, "--assume-yes")])
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if rm_pkgmgr == "paru" || rm_pkgmgr == "flatpak" {
		if rm_pkgmgr=="paru" {print_paru();}
		else {print_flatpak()}
		output = Command::new("sh").args(["-c", &format!("{} {} {}", &rm_pkgmgr, &pm.remove_cmd[rm_pkgmgr], rm_pkgname)])
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
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
// }}}

// display_local_pkg {{{
fn display_local_pkg(pm: &Pkgmgrs, pkg: &str) -> PkgResult {
    println!("\n{ITALIC}Finding packages matching '{}{RESET}':", pkg);
    let mut index = 1;
    let (mut pacman_idx, mut paru_idx, mut flatpak_idx, mut nala_idx) = (-1, -1, -1, -1);
    let mut res_string = String::new();
    for i in 0..pm.name.len() {
        let mut output;
        if pm.name[i] == "flatpak" {
            print_flatpak();
            output = Command::new(pm.name[i].clone()).args([&pm.search_local_cmd[&pm.name[i]], "--columns=application"])
                .stdout(Stdio::piped()).spawn().expect("");
        }
        else {
            if pm.name[i]=="pacman" {
                print_pacman();
            } else if pm.name[i]=="nala" {
                print_apt();
            }
            output = Command::new(pm.name[i].clone()).args([&pm.search_local_cmd[&pm.name[i]], pkg]).stdout(Stdio::piped()).spawn()
                .expect("");
        }

        if let Some(stdout) = output.stdout.take() {
            let mut nala_vec: Vec<String> = Vec::new();
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let line = &line.unwrap().replace("local/", "");
                if pm.name[i] == "pacman" && !line.contains("    ") {
                    let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                    println!("[{BLUE}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}{}{RESET}]{RESET}", index, &line[..fwi].replace("[installed]", ""), "pacman");
                    res_string += &line[..fwi];
                    res_string += "\n";
                    pacman_idx = index as i32;
                    index += 1;
                }

                else if pm.name[i] == "paru" && !line.contains("    ") {
                    let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                    if !res_string.contains(&line[..fwi]) {
                        println!("[{VIOLET}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{VIOLET}{}{RESET}]{RESET}", index, &line[..fwi].replace("(Installed)", ""), "paru");
                        res_string += &line[..fwi];
                        res_string += "\n";
                        paru_idx = index as i32;
                        index += 1;
                    }
                }

                else if pm.name[i]=="flatpak" && line.to_ascii_lowercase().contains(&pkg) {
                    println!("[{GREEN}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{GREEN}{}{RESET}]{RESET}", index, line, &pm.name[i]);
                    let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                    res_string += &line[..fwi];
                    res_string += "\n";
                    flatpak_idx = index as i32;
                    index += 1;
                }

                else if pm.name[i] == "nala" {
                    if line.contains("[Ubuntu") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let tmp = &line[..fwi];
                        nala_vec.push(tmp.to_string());
                        // println!("[{YELLOW}{}{RESET}]: {}", index, tmp);
                        res_string += &line[..fwi];
                        res_string += "\n";
                        // index += 1;
                    }
                    else if line.contains("├── is installed") {
                        // println!("[{YELLOW}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}{}{RESET}]{RESET}", index, &nala_vec[nala_vec.len() - 1], "nala");
                        let mut x = nala_vec.pop().unwrap();
                        x += " INSTALLED";
                        nala_vec.push(x);
                    }
                }
            }

            for i in 0..nala_vec.len() {
                if nala_vec[i].contains("INSTALLED") {
                    println!("[{YELLOW}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}{}{RESET}]", index+i, &nala_vec[i].replace(" INSTALLED", ""), "nala");
                    nala_idx = (i+index) as i32;
                } 
            }
        }
    }

    PkgResult {
        res: res_string,
        pos: vec![pacman_idx, paru_idx, flatpak_idx, nala_idx],
    }
}
// }}}

// display_pkg {{{
fn display_pkg(pm: &Pkgmgrs, pkg: &str) -> PkgResult {
	println!("\n{ITALIC}Finding packages matching '{}{RESET}':", pkg);
	let mut index = 1;
	let (mut pacman_idx, mut paru_idx, mut flatpak_idx, mut nala_idx) = (-1, -1, -1, -1);
	let mut res_string = String::new();
	for i in 0..pm.name.len() {
		let mut output;
		if pm.name[i] == "flatpak" {
			print_flatpak();
			output = Command::new(pm.name[i].clone())
				.args([&pm.search_cmd[&pm.name[i]], pkg, "--columns=application"]).stdout(Stdio::piped()).spawn()
				.expect("");
		}
		else {
			match pm.name[i].as_str() {
				"pacman" => print_pacman(),
				"paru" => print_paru(),
				"apt" | "nala" => print_apt(),
				_ => {}
			}
			output = Command::new(pm.name[i].clone()).args([&pm.search_cmd[&pm.name[i]], pkg]).stdout(Stdio::piped()).spawn()
				.expect("");
		}
		if let Some(stdout) = output.stdout.take() {
			let reader = BufReader::new(stdout);
            let mut nala_vec: Vec<String> = Vec::new();
			for line in reader.lines() {
				let line = &line.unwrap().replace("extra/", "").replace("aur/", "").replace("core/", "").replace("stratos/", "");
				if pm.name[i] == "pacman" {
					if line.contains("[installed]") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}{}{RESET}]{RESET}", index, &line[..fwi].replace("[installed]", ""), "pacman");
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index as i32;
						index += 1;
					}
					else if !line.contains("    ") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{BLUE}{}{RESET}]: {}", index, &line[..fwi]);
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index as i32;
						index += 1;
					}
 				}

				else if pm.name[i] == "paru" {
					if line.contains("(Installed") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{VIOLET}{}{RESET}]{RESET}", index, &line[..fwi].replace("(Installed", ""), "paru");
						res_string += &line[..fwi];
						res_string += "\n";
						paru_idx = index as i32;
						index += 1;
					}
					else if !line.contains("    ") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{VIOLET}{}{RESET}]: {}", index, &line[..fwi]);
						res_string += &line[..fwi];
						res_string += "\n";
						paru_idx = index as i32;
						index += 1;
					}
				}

				else if pm.name[i]=="flatpak" && !line.contains("No matches found") {
					println!("[{GREEN}{}{RESET}]: {}", index, line);
					let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
					res_string += &line[..fwi];
					res_string += "\n";
					flatpak_idx = index as i32;
					index += 1;
				}

                else if pm.name[i] == "nala" {
                    if line.contains("[Ubuntu") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let tmp = &line[..fwi];
                        nala_vec.push(tmp.to_string());
                        // println!("[{YELLOW}{}{RESET}]: {}", index, tmp);
                        res_string += &line[..fwi];
                        res_string += "\n";
                        // index += 1;
                    }
                    else if line.contains("├── is installed") {
                        // println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}{}{RESET}]{RESET}", index, &nala_vec[nala_vec.len() - 1], "nala");
                        let mut x = nala_vec.pop().unwrap();
                        x += " INSTALLED";
                        nala_vec.push(x);
                    }

                }
			}

            for i in 0..nala_vec.len() {
                if nala_vec[i].contains("INSTALLED") {
                    println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}{}{RESET}]", index+i, &nala_vec[i].replace(" INSTALLED", ""), "nala");
                    nala_idx = (i+index) as i32;
                } else {
                    println!("[{YELLOW}{}{RESET}]: {}", index + i, &nala_vec[i]);
                    nala_idx = (i+index) as i32;
                }
            }
		}
	}
	
	PkgResult {
		res: res_string,
		pos: vec![pacman_idx, paru_idx, flatpak_idx, nala_idx],
	}
}
// }}}

// main {{{
fn main() {
	let args: Vec<String> = env::args().collect();
	let mut rockcmd: &str = "";
	let mut pkgname: String = String::from(""); // to handle cases where a pkg name is not required

	match args.len() {
		1 => banner(),
		2 => { rockcmd = &args[1]; }
		_ => {
			rockcmd = &args[1];
			pkgname = args[2..].join(" ");
		}
	}

	println!("{ITALIC}Package managers detected:{RESET}");
	let mut pm = Pkgmgrs {
		name: Vec::new(), install_cmd: HashMap::new(), search_cmd: HashMap::new(), search_local_cmd: HashMap::new(),
		info_cmd: HashMap::new(), inst_info_cmd: HashMap::new(), update_cmd: HashMap::new(), remove_cmd: HashMap::new(),
		cleanup_cmd: HashMap::new()
	};
	
	if pkgmgr_found("/usr/bin/pacman") {
		println!("{BOLD}{ITALIC} 󰱒 {BLUE} Pacman 󰣇 {RESET}");
		pm.name.push("pacman".to_string());
		pm.install_cmd.insert(pm.name[0].clone(), "-S".to_string());
		pm.search_cmd.insert(pm.name[0].clone(), "-Ss".to_string());
		pm.search_local_cmd.insert(pm.name[0].clone(), "-Qs".to_string());
		pm.info_cmd.insert(pm.name[0].clone(), "-Si".to_string());
		pm.inst_info_cmd.insert(pm.name[0].clone(), "-Qi".to_string());
		pm.update_cmd.insert(pm.name[0].clone(), "-Syu".to_string());
		pm.remove_cmd.insert(pm.name[0].clone(), "-Rns".to_string());
		pm.cleanup_cmd.insert(pm.name[0].clone(), "-Rcns".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {BLUE} Pacman 󰣇 {RESET}");
	}
	
	if pkgmgr_found("/usr/bin/paru") {
		println!("{BOLD}{ITALIC} 󰱒 {VIOLET} Paru 󰣇 {RESET}");
		pm.name.push("paru".to_string());
		pm.install_cmd.insert(pm.name[1].clone(), "-Sa".to_string());
		pm.search_cmd.insert(pm.name[1].clone(), "-Ssa".to_string());
		pm.search_local_cmd.insert(pm.name[1].clone(), "-Qsa".to_string());
		pm.info_cmd.insert(pm.name[1].clone(), "-Sai".to_string());
		pm.inst_info_cmd.insert(pm.name[1].clone(), "-Qi".to_string());
		pm.update_cmd.insert(pm.name[1].clone(), "-Syu".to_string());
		pm.remove_cmd.insert(pm.name[1].clone(), "-Rns".to_string());
		pm.cleanup_cmd.insert(pm.name[1].clone(), "-Rcns".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {VIOLET} Paru 󰣇 {RESET}");
	}
	
	if pkgmgr_found("/usr/bin/flatpak") {
		println!("{BOLD}{ITALIC} 󰱒 {GREEN} Flatpak  {RESET}");
		pm.name.push("flatpak".to_string());
		pm.install_cmd.insert(pm.name[2].clone(), "install".to_string());
		pm.search_cmd.insert(pm.name[2].clone(), "search".to_string());
		pm.search_local_cmd.insert(pm.name[2].clone(), "list".to_string());
		pm.info_cmd.insert(pm.name[2].clone(), "remote-info".to_string());
		pm.inst_info_cmd.insert(pm.name[2].clone(), "info".to_string());
		pm.update_cmd.insert(pm.name[2].clone(), "update".to_string());
		pm.remove_cmd.insert(pm.name[2].clone(), "uninstall".to_string());
		pm.cleanup_cmd.insert(pm.name[2].clone(), "uninstall".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {GREEN} Flatpak  {RESET}");
	}

	if pkgmgr_found("/bedrock/cross/bin/nala") {
		println!("{BOLD}{ITALIC} 󰱒 {YELLOW} Apt   {RESET}");
		pm.name.push("nala".to_string());
		pm.install_cmd.insert(pm.name[3].clone(), "install".to_string());
		pm.search_cmd.insert(pm.name[3].clone(), "search".to_string());
		pm.search_local_cmd.insert(pm.name[3].clone(), "list".to_string());
		pm.info_cmd.insert(pm.name[3].clone(), "show".to_string());
		pm.inst_info_cmd.insert(pm.name[3].clone(), "show".to_string());
		pm.update_cmd.insert(pm.name[3].clone(), "upgrade".to_string());
		pm.remove_cmd.insert(pm.name[3].clone(), "uninstall".to_string());
		pm.cleanup_cmd.insert(pm.name[3].clone(), "autopurge".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {YELLOW} Apt   {RESET}");
	}
	match rockcmd {
		"install"          | "i"      => install_pkg(&pm, &pkgname),
		"search"           | "s"      => { let _ = display_pkg(&pm, &pkgname); },
		"install-info"     | "iif"    => inst_info_pkg(&pm, &pkgname),
		"info"             | "if"     => info_pkg(&pm, &pkgname),
		"update"           | "u"      => update_pkg(&pm),
		"remove"           | "r"      => remove_pkg(&pm, &pkgname),
	 	"clean"            | "c"      => cleanup_pkg(&pm),
		"-h"               | "--help" => banner(),
		_                             => print!("{BOLD}Invalid Usage.{RESET} Consult {ITALIC}rock --help{RESET} for more information."),
	}
}
// }}}
