// use clap::Parser;
use thiserror::Error;

mod cfg;

#[cfg(test)]
mod tests;

fn main() -> Result<(), CRMainErr> {
	cr_main()
}

#[derive(Debug, Error)]
enum CRMainErr {
	#[error("XDG -> {0}")]
	XDG(xdg::BaseDirectoriesError),
}

fn cr_main() -> Result<(), CRMainErr> {
	let cmd_new = clap::Command::new("new").about("Create a new environment");
	let cmd_use = clap::Command::new("use").about("Start using an environment");

	let cmd_main = clap::Command::new(env!("CARGO_CRATE_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.args([clap::Arg::new("inherit")
			.short('I')
			.long("inherit")
			.help("Whether to use inheritance")])
		.subcommand(cmd_new)
		.subcommand(cmd_use)
		.get_matches();

	let dirs = match xdg::BaseDirectories::with_prefix("cleanroom") {
		Ok(ok) => ok,
		Err(err) => return Err(CRMainErr::XDG(err)),
	};

	let _ = cmd_main;
	let _ = dirs;

	Ok(())
}
