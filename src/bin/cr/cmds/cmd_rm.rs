use std::fs;
use std::io;

use thiserror::Error;

use crate::args;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	IO(#[from] io::Error),
}

pub fn cmd_rm(
	args_main: args::CmdMainArgs,
	args_rm: args::SubCmdRmArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	fs::remove_dir_all(dirs.get_config_home().join(&args_rm.name))?;
	fs::remove_dir_all(dirs.get_data_home().join(&args_rm.name))?;
	Ok(())
}
