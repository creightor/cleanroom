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

	#[error("Couldn't convert a `PathBuf` directory to `&str`")]
	DirToStr,
}

pub fn cmd_use(
	args_main: args::CmdMainArgs,
	args_use: args::SubCmdUseArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	let cfg_home = dirs.get_config_home();
	let env_cfg_dir = cfg_home.join(&args_use.name);
	let data_home = dirs.get_data_home();
	let env_data_dir = data_home.join(&args_use.name);

	let env_table = crenv::Table::from_env(&args_use.name, &dirs)?;
	let mut shell_args: Vec<String> =
		env_table.get_shell_args(&args_use.name, &dirs)?;

	dbgfmt!("Using config: {:#?}", env_table);
	dbgfmt!("Calling with args: {:?}", shell_args);
	env_table.bin.inherit_bins(&env_data_dir)?;

	let mut shell = process::Command::new(env_table.shell.bin);
	let mut shell = shell.args(shell_args).env_clear();

	let shell_env_vars = env_table.vars.to_env()?;
	for (k, v) in shell_env_vars {
		shell = shell.env(k, v);
	}

	let shell_path = env_table
		.bin
		.inherit_dirs
		.iter()
		.map(|dir| dir.to_str())
		.collect::<Option<Vec<_>>>()
		.ok_or(Err::DirToStr)?
		.join(":");
	shell.env("PATH", shell_path);

	let mut shell = shell.spawn()?;
	shell.wait()?;

	Ok(())
}
