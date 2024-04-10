use super::Err as CmdErr;
use crate::args;
use crate::debug::todom;
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
	files::create_config_dir(&dirs)?;

	let cfg_home = dirs.get_config_home();
	let env_dir = cfg_home.join(&args_new.name);
	files::create_env_dir(env_dir.clone()).dbg_panic()?;
	let env_files = files::create_env_files(env_dir.clone())?;

	println!("Created new environment {} in {:?}", args_new.name, env_dir);

	let _ = env_files;

	Ok(())
}
