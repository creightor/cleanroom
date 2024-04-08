//! # Cleanroom
//!
//! Cleanroom is a CLI program to manage shell environments.

#![deny(missing_docs)]

use std::backtrace;

use thiserror::Error;

mod args;
mod cfg;
mod cmds;
mod debug;
mod files;

#[cfg(test)]
mod tests;

fn main() -> Result<(), CRMainErr> {
	match cr_main() {
		Ok(ok) => Ok(ok),
		Err(err) => {
			eprintln!("Error: {err}");
			Err(err)
		}
	}
}

#[derive(Debug, Error)]
enum CRMainErr {
	#[error(transparent)]
	XDG(#[from] xdg::BaseDirectoriesError),
	#[error(transparent)]
	Cfg(#[from] cfg::Err),
	#[error(transparent)]
	Cmd(#[from] cmds::Err),
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
			if let Err(err) = cmds::cmd_new(cfg) {
				return Err(CRMainErr::Cmd(err));
			}
		}

		args::CmdMainSub::Use { name: _ } => {
			if let Err(err) = cmds::cmd_use(cfg) {
				return Err(CRMainErr::Cmd(err));
			}
		}
	}
	Ok(())
}
