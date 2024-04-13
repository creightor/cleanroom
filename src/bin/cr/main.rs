//! # Cleanroom
//!
//! Cleanroom is a CLI program to manage shell environments.

// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_macros)]
// #![allow(unused_mut)]

use std::env;

use thiserror::Error;

pub mod args;
pub mod cmds;
mod debug;
pub mod files;
pub mod macros;
pub mod senv;
pub mod table;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
enum Err {
	#[error(transparent)]
	XDG(#[from] xdg::BaseDirectoriesError),
	#[error(transparent)]
	Cmd(#[from] cmds::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
}

fn main() -> Result<()> {
	match cr_main() {
		Ok(ok) => Ok(ok),
		Err(err) => {
			eprintln!("Error: {err}");
			Err(err)
		}
	}
}

fn cr_main() -> Result<()> {
	let cmd = args::CmdMain::from_parse();
	let dirs = match xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")) {
		Ok(ok) => ok,
		Err(err) => return Err(Err::XDG(err)),
	};

	match cmd.sub {
		args::CmdMainSub::New { args: args_new } => {
			if let Err(err) = cmds::cmd_new(cmd.args, args_new, dirs) {
				return Err(Err::Cmd(cmds::Err::New(err)));
			}
		}

		args::CmdMainSub::Use { args: args_use } => {
			if let Err(err) = cmds::cmd_use(cmd.args, args_use, dirs) {
				return Err(Err::Cmd(cmds::Err::Use(err)));
			}
		}

		args::CmdMainSub::Rm { args: args_rm } => {
			if let Err(err) = cmds::cmd_rm(cmd.args, args_rm, dirs) {
				return Err(Err::Cmd(cmds::Err::Rm(err)));
			}
		}
	}
	Ok(())
}
