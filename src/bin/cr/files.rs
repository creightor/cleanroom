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
	Env(#[from] Box<crenv::Err>),
	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("couldn't convert path to string")]
	PathToStr,
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
				return Err(Box::new(crenv::Err::EnvExists(env_dir)))?;
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

	let env_file_names = get_env_file_names(&env_dir);

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
				return Err(Box::new(crenv::Err::NoExists))?;
			}
		}
		Err(err) => {
			return Err(err)?;
		}
	}

	for file in get_env_file_names(&env_dir) {
		match file.try_exists().dbg_panic() {
			Ok(exists) => {
				if !exists {
					return Err(Box::new(crenv::Err::NoExistsEnvFile(file)))?;
				}
			}
			Err(err) => {
				return Err(err)?;
			}
		}
	}

	Ok(())
}

fn get_env_file_names(env_dir: &path::PathBuf) -> Vec<path::PathBuf> {
	vec!["config.toml"]
		.iter()
		.map(|file_basename| env_dir.join(file_basename))
		.collect()
}
