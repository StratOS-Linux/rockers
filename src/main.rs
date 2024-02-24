use clap::Parser;

#[derive(Parser)]
struct CliArgs {
	rockcmd: String,
	pkgname: String
}

fn main() {
	let args = CliArgs::parse();
	println!("Command: {:?} Pkg: {:?}", args.rockcmd, args.pkgname);
}
