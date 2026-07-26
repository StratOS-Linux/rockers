use std::path::Path;
use std::process::exit;
use std::{env, process::{Command, Stdio}};
use std::io::{self, Write, BufRead, BufReader};
use std::collections::HashMap;
use std::thread;

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

fn print_dnf() {
    println!("\n{BOLD}{ITALIC}>>>{RED} Fedora repos  {RESET}\n");
}

fn print_emerge() {
    println!("\n{BOLD}{ITALIC}>>>{BLUE} Gentoo repos  {RESET}\n");
}

fn print_xbps() {
    println!("\n{BOLD}{ITALIC}>>>{GREEN} XBPS repos  {RESET}\n");
}

fn pkgmgr_found(p: &str) -> bool {
	if Path::new(p).is_file() { return true; }
	false
}

// adjust_idx {{{
fn adjust_idx(positions: &[i32]) {
	let last = positions.iter().rposition(|&x| x != -1);
	match last {
		None => {
			print!("{ITALIC}No matching packages found.{RESET}");
			exit(-1);
		}
		Some(idx) => {
			print!("\n{ITALIC}Select package [1-{}]: {RESET}", positions[idx]);
			let _ = io::stdout().flush();
		}
	}
}
// }}}

// inst_info_pkg {{{
fn inst_info_pkg(pm: &Pkgmgrs, pkg: &str) {
	let x = display_local_pkg(&pm, pkg);
	let mut input_pkg_str = String::new();
	
	adjust_idx(&x.pos);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let pm_names = ["pacman", "paru", "flatpak", "nala", "dnf5", "emerge", "xbps"];
	let mut info_pkgmgr = "";
	for i in 0..x.pos.len() {
		if (i == 0 && 1 <= input_pkg_num && input_pkg_num <= x.pos[i])
			|| (i > 0 && x.pos[i-1] < input_pkg_num && input_pkg_num <= x.pos[i])
		{
			info_pkgmgr = pm_names[i];
			break;
		}
	}
	if info_pkgmgr.is_empty() {
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
	
	adjust_idx(&x.pos);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let pm_names = ["pacman", "paru", "flatpak", "nala", "dnf5", "emerge", "xbps"];
	let mut info_pkgmgr = "";
	for i in 0..x.pos.len() {
		if (i == 0 && 1 <= input_pkg_num && input_pkg_num <= x.pos[i])
			|| (i > 0 && x.pos[i-1] < input_pkg_num && input_pkg_num <= x.pos[i])
		{
			info_pkgmgr = pm_names[i];
			break;
		}
	}
	if info_pkgmgr.is_empty() {
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
			.args(["--user", &pm.info_cmd[info_pkgmgr], "flathub", info_pkgname])
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
			"dnf5" => "--assumeyes",
			"xbps" => "-y",
			_ => ""
		};
		if pm.name[i] == "pacman" || pm.name[i] == "nala" || pm.name[i] == "dnf5" || pm.name[i] == "xbps" { // run with sudo.
			if pm.name[i] == "pacman" {print_pacman();}
			else if pm.name[i] == "nala" {print_apt();}
			else if pm.name[i] == "dnf5" {print_dnf();}
			else if pm.name[i] == "xbps" {print_xbps();}
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
			output = Command::new(&pm.name[i]).arg("--user").arg(&pm.update_cmd[&pm.name[i]]).arg(noc)
				.stdout(Stdio::piped()).spawn().expect("Failed to start command");
		}
		else if pm.name[i] == "emerge" {
			print_emerge();
			output = Command::new("sudo").arg(&pm.name[i]).arg(&pm.update_cmd[&pm.name[i]])
				.stdout(Stdio::piped()).spawn().expect("Failed to start command");
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
	    match pm.name[i].as_str() {
			"nala" => {
    			print_apt();
    			output = Command::new("sh")
    				.args(["-c", &format!("sudo {} {} --assume-yes", &pm.name[i], &pm.cleanup_cmd[&pm.name[i]])])
    				.stdout(Stdio::piped()).spawn().expect("No such pkg");
			}
			"pacman" => {
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
	        "flatpak" => { // no need to check for paru.
    			print_flatpak();
    			output = Command::new("sh")
    				.args(["-c", &format!("{} --user {} {} {}", &pm.name[i], &pm.cleanup_cmd[&pm.name[i]], "--unused", "--assumeyes")])
    				.stdout(Stdio::piped()).spawn().expect("No such pkg");
			},
			"dnf5" => {
                print_dnf();
                output = Command::new("sudo").arg(&pm.name[i]).arg(&pm.cleanup_cmd[&pm.name[i]]).arg("--assumeyes")
                    .stdout(Stdio::piped()).spawn().expect("No such pkg");
			},
			"emerge" => {
				print_emerge();
				output = Command::new("sudo").arg(&pm.name[i]).arg(&pm.cleanup_cmd[&pm.name[i]]).arg("--ask=n")
					.stdout(Stdio::piped()).spawn().expect("No such pkg");
			},
			"xbps" => {
				print_xbps();
				output = Command::new("sudo").arg(&pm.name[i]).arg(&pm.cleanup_cmd[&pm.name[i]]).arg("-y")
					.stdout(Stdio::piped()).spawn().expect("No such pkg");
			},
			_ => todo!() // IMPOSSIBLE
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
	adjust_idx(&x.pos);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	// don't query repos once again.
	let pm_names = ["pacman", "paru", "flatpak", "nala", "dnf5", "emerge", "xbps"];
	let mut inst_pkgmgr = "";
	for i in 0..x.pos.len() {
		if (i == 0 && 1 <= input_pkg_num && input_pkg_num <= x.pos[i])
			|| (i > 0 && x.pos[i-1] < input_pkg_num && input_pkg_num <= x.pos[i])
		{
			inst_pkgmgr = pm_names[i];
			break;
		}
	}
	if inst_pkgmgr.is_empty() {
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
		"dnf5" => "--assumeyes",
		"xbps" => "-y",
		_ => ""
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
		output = Command::new(&inst_pkgmgr).arg("--user").arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "dnf5" {
		output = Command::new("sudo").arg(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "emerge" {
		print_emerge();
		output = Command::new("sudo").arg(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if inst_pkgmgr == "xbps" {
		print_xbps();
		output = Command::new("sudo").arg(&inst_pkgmgr).arg(&pm.install_cmd[inst_pkgmgr]).arg(inst_pkgname).arg(noc)
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
	adjust_idx(&x.pos);
	
	io::stdin().read_line(&mut input_pkg_str).expect("Enter a valid integer.");
	let input_pkg_num: i32 = input_pkg_str.trim().parse().expect("Cannot convert to integer.");

	let pm_names = ["pacman", "paru", "flatpak", "nala", "dnf5", "emerge", "xbps"];
	let mut rm_pkgmgr = "";
	for i in 0..x.pos.len() {
		if (i == 0 && 1 <= input_pkg_num && input_pkg_num <= x.pos[i])
			|| (i > 0 && x.pos[i-1] < input_pkg_num && input_pkg_num <= x.pos[i])
		{
			rm_pkgmgr = pm_names[i];
			break;
		}
	}
	if rm_pkgmgr.is_empty() {
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
		if rm_pkgmgr == "flatpak" {
			output = Command::new("sh").args(["-c", &format!("{} --user {} {}", &rm_pkgmgr, &pm.remove_cmd[rm_pkgmgr], rm_pkgname)])
				.stdout(Stdio::piped()).spawn().expect("No such pkg");
		} else {
			output = Command::new("sh").args(["-c", &format!("{} {} {}", &rm_pkgmgr, &pm.remove_cmd[rm_pkgmgr], rm_pkgname)])
				.stdout(Stdio::piped()).spawn().expect("No such pkg");
		}
	}
	else if rm_pkgmgr == "dnf5" {
		print_dnf();
		output = Command::new("sudo").arg(&rm_pkgmgr).arg(&pm.remove_cmd[rm_pkgmgr]).arg(rm_pkgname).arg("--assumeyes")
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if rm_pkgmgr == "emerge" {
		print_emerge();
		output = Command::new("sudo").arg(&rm_pkgmgr).arg(&pm.remove_cmd[rm_pkgmgr]).arg(rm_pkgname)
			.stdout(Stdio::piped()).spawn().expect("No such pkg");
	}
	else if rm_pkgmgr == "xbps" {
		print_xbps();
		output = Command::new("sudo").arg(&rm_pkgmgr).arg(&pm.remove_cmd[rm_pkgmgr]).arg(rm_pkgname).arg("-y")
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

struct PmSearchResult {
	pm_name: String,
	packages: Vec<String>,
}

fn spawn_pm_search(pm_name: &str, search_cmd: &str, pkg: &str, search_local: bool) -> PmSearchResult {
	let mut args: Vec<&str> = Vec::new();
	if pm_name == "flatpak" {
		args.push("--user");
	}
	if search_local {
		args.push(search_cmd);
		if pm_name == "flatpak" {
			args.push("--columns=application");
		}
	} else {
		args.push(search_cmd);
		args.push(pkg);
		if pm_name == "flatpak" {
			args.push("--columns=application");
		}
	}

	let output = if pm_name == "flatpak" {
		Command::new(pm_name).args(&args).stdout(Stdio::piped()).spawn()
	} else {
		Command::new(pm_name).args(&args).stdout(Stdio::piped()).spawn()
	};

	let mut packages = Vec::new();
	match output {
		Ok(mut child) => {
			if let Some(stdout) = child.stdout.take() {
				let reader = BufReader::new(stdout);
				for line in reader.lines() {
					if let Ok(line) = line {
						if !line.is_empty() {
							packages.push(line);
						}
					}
				}
			}
			let _ = child.wait();
		}
		Err(_) => {}
	}

	PmSearchResult {
		pm_name: pm_name.to_string(),
		packages,
	}
}

// display_local_pkg {{{
fn display_local_pkg(pm: &Pkgmgrs, pkg: &str) -> PkgResult {
    println!("\n{ITALIC}Finding packages matching '{}{RESET}':", pkg);

    let mut handles = Vec::new();
    for name in &pm.name {
        let name = name.clone();
        let cmd = pm.search_local_cmd[&name].clone();
        let pkg = pkg.to_string();
        handles.push(thread::spawn(move || {
            spawn_pm_search(&name, &cmd, &pkg, true)
        }));
    }

    let mut results: Vec<PmSearchResult> = Vec::new();
    for h in handles {
        results.push(h.join().unwrap());
    }

    let mut index = 1;
    let mut pacman_idx = -1i32;
    let mut paru_idx = -1i32;
    let mut flatpak_idx = -1i32;
    let mut nala_idx = -1i32;
    let mut dnf5_idx = -1i32;
    let mut emerge_idx = -1i32;
    let mut xbps_idx = -1i32;
    let mut res_string = String::new();

    for res in &results {
        match res.pm_name.as_str() {
            "pacman" => {
                if !res.packages.is_empty() { print_pacman(); }
                for line in &res.packages {
                    let line = line.replace("local/", "");
                    if !line.contains("    ") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let pkg_name = line[..fwi].replace("[installed]", "");
                        println!("[{BLUE}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}pacman{RESET}]{RESET}", index, pkg_name);
                        res_string += &line[..fwi];
                        res_string += "\n";
                        pacman_idx = index as i32;
                        index += 1;
                    }
                }
            }
            "paru" => {
                if !res.packages.is_empty() { print_paru(); }
                for line in &res.packages {
                    let line = line.replace("local/", "");
                    if !line.contains("    ") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let pkg_name = &line[..fwi];
                        if !res_string.contains(pkg_name) {
                            let clean = pkg_name.replace("(Installed)", "");
                            println!("[{VIOLET}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{VIOLET}paru{RESET}]{RESET}", index, clean);
                            res_string += pkg_name;
                            res_string += "\n";
                            paru_idx = index as i32;
                            index += 1;
                        }
                    }
                }
            }
            "flatpak" => {
                if !res.packages.is_empty() { print_flatpak(); }
                for line in &res.packages {
                    if line.to_ascii_lowercase().contains(&pkg.to_ascii_lowercase()) {
                        println!("[{GREEN}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{GREEN}flatpak{RESET}]{RESET}", index, line);
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        res_string += &line[..fwi];
                        res_string += "\n";
                        flatpak_idx = index as i32;
                        index += 1;
                    }
                }
            }
            "nala" => {
                if !res.packages.is_empty() { print_apt(); }
                let mut nala_vec: Vec<String> = Vec::new();
                for line in &res.packages {
                    let line = line.replace("local/", "");
                    if line.contains("[Ubuntu") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let tmp = &line[..fwi];
                        nala_vec.push(tmp.to_string());
                        res_string += &line[..fwi];
                        res_string += "\n";
                    } else if line.contains("├── is installed") {
                        if let Some(last) = nala_vec.last_mut() {
                            *last = format!("{} INSTALLED", last);
                        }
                    }
                }
                let base = index;
                for (i, entry) in nala_vec.iter().enumerate() {
                    if entry.contains("INSTALLED") {
                        println!("[{YELLOW}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}nala{RESET}]", base + i, &entry.replace(" INSTALLED", ""));
                        nala_idx = (base + i) as i32;
                    }
                }
                index += nala_vec.len();
            }
            "dnf5" => {
                if !res.packages.is_empty() { print_dnf(); }
                for line in &res.packages {
                    if !line.is_empty() && !line.contains("Installed") && !line.contains("Name") && !line.contains("Last metadata") && !line.contains("Matched") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let pkg_name = &line[..fwi];
                        if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
                            println!("[{RED}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{RED}dnf5{RESET}]{RESET}", index, pkg_name);
                            res_string += pkg_name;
                            res_string += "\n";
                            dnf5_idx = index as i32;
                            index += 1;
                        }
                    }
                }
            }
            "emerge" => {
                if !res.packages.is_empty() { print_emerge(); }
                for line in &res.packages {
                    if !line.is_empty() && !line.contains("Searching") && !line.contains("No matches") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let pkg_name = &line[..fwi];
                        if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
                            println!("[{BLUE}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}emerge{RESET}]{RESET}", index, pkg_name);
                            res_string += pkg_name;
                            res_string += "\n";
                            emerge_idx = index as i32;
                            index += 1;
                        }
                    }
                }
            }
            "xbps" => {
                if !res.packages.is_empty() { print_xbps(); }
                for line in &res.packages {
                    if !line.is_empty() && !line.contains("Name") && !line.contains("---") && !line.contains("No matches") {
                        let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
                        let pkg_name = &line[..fwi];
                        if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
                            println!("[{GREEN}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{GREEN}xbps{RESET}]{RESET}", index, pkg_name);
                            res_string += pkg_name;
                            res_string += "\n";
                            xbps_idx = index as i32;
                            index += 1;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    PkgResult {
        res: res_string,
        pos: vec![pacman_idx, paru_idx, flatpak_idx, nala_idx, dnf5_idx, emerge_idx, xbps_idx],
    }
}
// }}}

// display_pkg {{{
fn display_pkg(pm: &Pkgmgrs, pkg: &str) -> PkgResult {
	println!("\n{ITALIC}Finding packages matching '{}{RESET}':", pkg);

	let mut handles = Vec::new();
	for name in &pm.name {
		let name = name.clone();
		let cmd = pm.search_cmd[&name].clone();
		let pkg = pkg.to_string();
		handles.push(thread::spawn(move || {
			spawn_pm_search(&name, &cmd, &pkg, false)
		}));
	}

	let mut results: Vec<PmSearchResult> = Vec::new();
	for h in handles {
		results.push(h.join().unwrap());
	}

	let mut index = 1;
	let mut pacman_idx = -1i32;
	let mut paru_idx = -1i32;
	let mut flatpak_idx = -1i32;
	let mut nala_idx = -1i32;
	let mut dnf5_idx = -1i32;
	let mut emerge_idx = -1i32;
	let mut xbps_idx = -1i32;
	let mut res_string = String::new();

	for res in &results {
		match res.pm_name.as_str() {
			"pacman" => {
				if !res.packages.is_empty() { print_pacman(); }
				for line in &res.packages {
					if line.contains("[installed]") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let pkg_name = &line[..fwi].replace("[installed]", "");
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}pacman{RESET}]{RESET}", index, pkg_name);
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index as i32;
						index += 1;
					} else if !line.contains("    ") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{BLUE}{}{RESET}]: {}", index, &line[..fwi]);
						res_string += &line[..fwi];
						res_string += "\n";
						pacman_idx = index as i32;
						index += 1;
					}
				}
			}
			"paru" => {
				if !res.packages.is_empty() { print_paru(); }
				for line in &res.packages {
					if line.contains("(Installed") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let pkg_name = &line[..fwi].replace("(Installed", "");
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{VIOLET}paru{RESET}]{RESET}", index, pkg_name);
						res_string += &line[..fwi];
						res_string += "\n";
						paru_idx = index as i32;
						index += 1;
					} else if !line.contains("    ") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						println!("[{VIOLET}{}{RESET}]: {}", index, &line[..fwi]);
						res_string += &line[..fwi];
						res_string += "\n";
						paru_idx = index as i32;
						index += 1;
					}
				}
			}
			"flatpak" => {
				if !res.packages.is_empty() { print_flatpak(); }
				for line in &res.packages {
					if !line.contains("No matches found") {
						println!("[{GREEN}{}{RESET}]: {}", index, line);
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						res_string += &line[..fwi];
						res_string += "\n";
						flatpak_idx = index as i32;
						index += 1;
					}
				}
			}
			"nala" => {
				if !res.packages.is_empty() { print_apt(); }
				let mut nala_vec: Vec<String> = Vec::new();
				for line in &res.packages {
					if line.contains("[Ubuntu") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let tmp = &line[..fwi];
						nala_vec.push(tmp.to_string());
						res_string += &line[..fwi];
						res_string += "\n";
					} else if line.contains("├── is installed") {
						if let Some(last) = nala_vec.last_mut() {
							*last = format!("{} INSTALLED", last);
						}
					}
				}
				let base = index;
				for (i, entry) in nala_vec.iter().enumerate() {
					if entry.contains("INSTALLED") {
						println!("[{HIGHLIGHT}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{YELLOW}nala{RESET}]", base + i, &entry.replace(" INSTALLED", ""));
						nala_idx = (base + i) as i32;
					} else {
						println!("[{YELLOW}{}{RESET}]: {}", base + i, entry);
						nala_idx = (base + i) as i32;
					}
				}
				index += nala_vec.len();
			}
			"dnf5" => {
				if !res.packages.is_empty() { print_dnf(); }
				for line in &res.packages {
					if !line.is_empty() && !line.contains("Name") && !line.contains("Last metadata") && !line.contains("Matched") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let pkg_name = &line[..fwi];
						if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
							println!("[{RED}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{RED}dnf5{RESET}]{RESET}", index, pkg_name);
							res_string += pkg_name;
							res_string += "\n";
							dnf5_idx = index as i32;
							index += 1;
						}
					}
				}
			}
			"emerge" => {
				if !res.packages.is_empty() { print_emerge(); }
				for line in &res.packages {
					if !line.is_empty() && !line.contains("Searching") && !line.contains("No matches") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let pkg_name = &line[..fwi];
						if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
							println!("[{BLUE}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{BLUE}emerge{RESET}]{RESET}", index, pkg_name);
							res_string += pkg_name;
							res_string += "\n";
							emerge_idx = index as i32;
							index += 1;
						}
					}
				}
			}
			"xbps" => {
				if !res.packages.is_empty() { print_xbps(); }
				for line in &res.packages {
					if !line.is_empty() && !line.contains("Name") && !line.contains("---") && !line.contains("No matches") {
						let fwi = line.find(char::is_whitespace).unwrap_or(line.len());
						let pkg_name = &line[..fwi];
						if !pkg_name.is_empty() && !res_string.contains(pkg_name) {
							println!("[{GREEN}{}{RESET}]: {BOLD}{ITALIC}{}{RESET} [{GREEN}xbps{RESET}]{RESET}", index, pkg_name);
							res_string += pkg_name;
							res_string += "\n";
							xbps_idx = index as i32;
							index += 1;
						}
					}
				}
			}
			_ => {}
		}
	}

	PkgResult {
		res: res_string,
		pos: vec![pacman_idx, paru_idx, flatpak_idx, nala_idx, dnf5_idx, emerge_idx, xbps_idx],
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
		println!("{BOLD}{ITALIC} 󰱒 {YELLOW} Apt   {RESET}");
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
		println!("{BOLD}{ITALIC} 󰄱 {YELLOW} Apt   {RESET}");
	}

	if pkgmgr_found("/bedrock/cross/bin/dnf5") {
		println!("{BOLD}{ITALIC} 󰱒 {RED} DNF5  {RESET}");
		pm.name.push("dnf5".to_string());
		pm.install_cmd.insert(pm.name[pm.name.len()-1].clone(), "install".to_string());
		pm.search_cmd.insert(pm.name[pm.name.len()-1].clone(), "search".to_string());
		pm.search_local_cmd.insert(pm.name[pm.name.len()-1].clone(), "list --installed".to_string());
		pm.info_cmd.insert(pm.name[pm.name.len()-1].clone(), "info".to_string());
		pm.inst_info_cmd.insert(pm.name[pm.name.len()-1].clone(), "info --installed".to_string());
		pm.update_cmd.insert(pm.name[pm.name.len()-1].clone(), "upgrade".to_string());
		pm.remove_cmd.insert(pm.name[pm.name.len()-1].clone(), "remove".to_string());
		pm.cleanup_cmd.insert(pm.name[pm.name.len()-1].clone(), "autoremove".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {RED} DNF5  {RESET}");
	}

	if pkgmgr_found("/bedrock/cross/bin/emerge") {
		println!("{BOLD}{ITALIC} 󰱒 {BLUE} Emerge {RESET}");
		pm.name.push("emerge".to_string());
		pm.install_cmd.insert(pm.name[pm.name.len()-1].clone(), "--ask=n".to_string());
		pm.search_cmd.insert(pm.name[pm.name.len()-1].clone(), "--search".to_string());
		pm.search_local_cmd.insert(pm.name[pm.name.len()-1].clone(), "-I".to_string());
		pm.info_cmd.insert(pm.name[pm.name.len()-1].clone(), "--info".to_string());
		pm.inst_info_cmd.insert(pm.name[pm.name.len()-1].clone(), "-fe".to_string());
		pm.update_cmd.insert(pm.name[pm.name.len()-1].clone(), "-uDN @world".to_string());
		pm.remove_cmd.insert(pm.name[pm.name.len()-1].clone(), "--unmerge".to_string());
		pm.cleanup_cmd.insert(pm.name[pm.name.len()-1].clone(), "--depclean".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {BLUE} Emerge {RESET}");
	}

	if pkgmgr_found("/bedrock/cross/bin/xbps-install") {
		println!("{BOLD}{ITALIC} 󰱒 {GREEN} XBPS   {RESET}");
		pm.name.push("xbps".to_string());
		pm.install_cmd.insert(pm.name[pm.name.len()-1].clone(), "-S".to_string());
		pm.search_cmd.insert(pm.name[pm.name.len()-1].clone(), "-Rs".to_string());
		pm.search_local_cmd.insert(pm.name[pm.name.len()-1].clone(), "-l".to_string());
		pm.info_cmd.insert(pm.name[pm.name.len()-1].clone(), "-Si".to_string());
		pm.inst_info_cmd.insert(pm.name[pm.name.len()-1].clone(), "-Qi".to_string());
		pm.update_cmd.insert(pm.name[pm.name.len()-1].clone(), "-Su".to_string());
		pm.remove_cmd.insert(pm.name[pm.name.len()-1].clone(), "-R".to_string());
		pm.cleanup_cmd.insert(pm.name[pm.name.len()-1].clone(), "-O".to_string());
	} else {
		println!("{BOLD}{ITALIC} 󰄱 {GREEN} XBPS   {RESET}");
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
