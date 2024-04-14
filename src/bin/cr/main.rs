//! # Cleanroom
//!
//! Cleanroom is a CLI program to manage shell environments.

// #![allow(
// 	dead_code,
// 	unused_variables,
// 	unused_imports,
// 	unused_macros,
// 	unused_mut
// )]

// #![warn(
// 	clippy::all,
// 	clippy::correctness,
// 	clippy::suspicious,
// 	clippy::complexity,
// 	clippy::perf,
// 	clippy::style,
// 	clippy::pedantic,
// 	clippy::nursery,
// 	clippy::cargo,
// 	clippy::restriction
// )]
// #![allow(
// 	clippy::must_use_candidate,
// 	clippy::missing_errors_doc,
// 	clippy::std_instead_of_core,
// 	clippy::implicit_return,
// 	clippy::missing_docs_in_private_items,
// 	clippy::self_named_module_files,
// 	clippy::question_mark_used,
// 	clippy::print_stdout,
// 	clippy::single_call_fn,
// 	clippy::blanket_clippy_restriction_lints,
// 	clippy::min_ident_chars,
// 	clippy::shadow_reuse,
// 	clippy::pub_with_shorthand,
// 	clippy::print_stderr,
// 	clippy::wildcard_enum_match_arm
// )]

use std::env;
use std::io;
use std::result;

use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
pub mod args;
#[allow(clippy::pub_use)]
pub mod cmds;
#[allow(clippy::module_name_repetitions)]
mod debug;
pub mod files;
pub mod macros;
pub mod senv;
pub mod table;

type Result<T> = result::Result<T, Err>;

#[derive(Debug, Error)]
enum Err {
	#[error(transparent)]
	Xdg(#[from] xdg::BaseDirectoriesError),
	#[error(transparent)]
	Cmd(#[from] cmds::Err),
	#[error(transparent)]
	IO(#[from] io::Error),
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
		Err(err) => return Err(Err::Xdg(err)),
	};

	match cmd.sub {
		args::CmdMainSub::New { args: args_new } => {
			if let Err(err) = cmds::cmd_new(&cmd.args, &args_new, &dirs) {
				return Err(Err::Cmd(cmds::Err::New(err)));
			}
		}

		args::CmdMainSub::Use { args: args_use } => {
			if let Err(err) = cmds::cmd_use(&cmd.args, &args_use, &dirs) {
				return Err(Err::Cmd(cmds::Err::Use(err)));
			}
		}

		args::CmdMainSub::Rm { args: args_rm } => {
			if let Err(err) = cmds::cmd_rm(&cmd.args, &args_rm, &dirs) {
				return Err(Err::Cmd(cmds::Err::Rm(err)));
			}
		}

		args::CmdMainSub::Ls { args: args_ls } => {
			if let Err(err) = cmds::cmd_ls(&cmd.args, &args_ls, &dirs) {
				return Err(Err::Cmd(cmds::Err::Ls(err)));
			}
		}
	}
	Ok(())
}
