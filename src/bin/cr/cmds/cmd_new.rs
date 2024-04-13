use super::Err as CmdErr;
use crate::args;
use crate::debug::DebugPanic;
use crate::files;
use thiserror::Error;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	Files(#[from] files::Err),
}

pub fn cmd_new(
	args_main: args::CmdMainArgs,
	args_new: args::SubCmdNewArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	files::create_env_dirs(&args_new.name, &dirs)?;
	files::create_env_files(&args_new.name, &dirs)?;

	Ok(())
}
