use std::collections::HashMap;
use std::fs;
use std::path;

use thiserror::Error;

use crate::cmds::Err as CmdErr;
use crate::crenv;
use crate::debug::DebugPanic;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	Env(#[from] crenv::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
}

pub fn create_config_dir(dirs: &xdg::BaseDirectories) -> Result<()> {
	let cfg_dir = dirs.get_config_home();
	match cfg_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if exists {
				return Ok(());
			}
			match std::fs::create_dir(cfg_dir).dbg_panic() {
				Ok(_) => Ok(()),
				Err(err) => return Err(err)?,
			}
		}
		Err(err) => return Err(err)?,
	}
}

pub fn create_env_dir(env_dir: path::PathBuf) -> Result<()> {
	match env_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if exists {
				return Err(crenv::Err::EnvExists(env_dir))?;
			}
		}
		Err(err) => {
			return Err(err)?;
		}
	}

	if let Err(err) = std::fs::create_dir_all(env_dir).dbg_panic() {
		return Err(err)?;
	}

	Ok(())
}

pub fn create_env_files(
	env_dir: path::PathBuf,
) -> Result<HashMap<path::PathBuf, fs::File>> {
	let mut env_files: HashMap<path::PathBuf, fs::File> = HashMap::new();

	let env_file_names = vec![
		env_dir.join("config.toml"),
		env_dir.join("pre-src.sh"),
		env_dir.join("pre-exec.sh"),
		env_dir.join("post-src.sh"),
		env_dir.join("post-exec.sh"),
	];

	for file_name in env_file_names {
		match fs::File::create_new(&file_name).dbg_panic() {
			Ok(file) => {
				env_files.insert(file_name, file);
			}
			Err(err) => {
				return Err(err)?;
			}
		}
	}

	Ok(env_files)
}

pub fn check_env_exists(env_dir: &path::PathBuf) -> Result<()> {
	match env_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if !exists {
				return Err(crenv::Err::NoExists)?;
			} else {
				Ok(())
			}
		}
		Err(err) => {
			return Err(err)?;
		}
	}
}
