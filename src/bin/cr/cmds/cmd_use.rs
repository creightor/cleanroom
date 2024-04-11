use std::env;
use std::io::Write;
use std::mem;
use std::os::unix::process::CommandExt;
use std::path;
use std::process;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Err as CmdErr;
use crate::args;
use crate::crenv;
use crate::debug::{dbgfmt, todom, DebugPanic};
use crate::files;

type PResult<T> = std::result::Result<T, CmdErr>;
type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	Files(#[from] files::Err),
	#[error(transparent)]
	Crenv(#[from] crenv::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
}

pub fn cmd_use(
	args_main: args::CmdMainArgs,
	args_use: args::SubCmdUseArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	let cfg_home = dirs.get_config_home();
	let env_dir = cfg_home.join(&args_use.name);
	let cfg_env = crenv::Table::from_env(&args_use.name, &dirs)?;

	let mut shell_args: Vec<&str> = Vec::new();
	if cfg_env.shell.noprofile {
		shell_args.push("--noprofile");
	}

	let rc_file = env_dir.join("rc.sh");
	let rc_file = rc_file.to_str().ok_or(files::Err::PathToStr)?;
	if !cfg_env.shell.norc && cfg_env.shell.interactive {
		shell_args.push("--rcfile");
		shell_args.push(rc_file);
		shell_args.push("-i");
	} else {
		shell_args.push("--norc");
	}

	if cfg_env.shell.login {
		shell_args.push("-l");
	}

	dbgfmt!("Using config: {:#?}", cfg_env);
	dbgfmt!("Calling with args: {:?}", shell_args);

	let mut shell = process::Command::new(cfg_env.shell.bin);
	let mut shell = shell.args(shell_args).env_clear();
	let shell_env_vars = cfg_env.vars.to_env()?;
	for (k, v) in shell_env_vars {
		shell = shell.env(k, v);
	}
	let mut shell = shell.spawn()?;
	shell.wait()?;

	Ok(())
}
