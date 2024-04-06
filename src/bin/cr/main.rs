//! # Cleanroom
//!
//! Cleanroom is a CLI program to manage shell environments.

#![deny(missing_docs)]

use thiserror::Error;

mod args;
mod cfg;
mod cmds;

#[cfg(test)]
mod tests;

fn main() -> Result<(), CRMainErr> {
	cr_main()
}

#[derive(Debug, Error)]
enum CRMainErr {
	#[error("XDG -> {0}")]
	XDG(xdg::BaseDirectoriesError),
	#[error("Cfg -> {0}")]
	Cfg(cfg::CfgErr),
}

fn cr_main() -> Result<(), CRMainErr> {
	let args = args::CmdMain::from_parse();
	let dirs = match xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")) {
		Ok(ok) => ok,
		Err(err) => return Err(CRMainErr::XDG(err)),
	};
	let cfg = match cfg::Cfg::from(args, dirs) {
		Ok(ok) => ok,
		Err(err) => return Err(CRMainErr::Cfg(err)),
	};

	match cfg.args.sub {
		args::CmdMainSub::New { name: _ } => {
			cmds::new(cfg);
		}
	}

	Ok(())
}
