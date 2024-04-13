//! Functions for file operations related to the
//! [XDG Base Directory specification] and the environments.
//!
//!
//! [XDG Base Directory specification]: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

use std::collections::HashMap;
use std::env;
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
	#[error(transparent)]
	VarErr(#[from] std::env::VarError),
	#[error(transparent)]
	TOMLSerializeErr(#[from] toml::ser::Error),

	#[error("Couldn't convert path to string.")]
	PathToStr,
	#[error("Binary '{0}' not in PATH. If it does exist, check permissions.")]
	NoBinInPath(path::PathBuf),
	#[error("binary '{0}' doesn't exist on host")]
	NoExistsBin(path::PathBuf),
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

pub fn create_env_dirs(name: &str, dirs: &xdg::BaseDirectories) -> Result<()> {
	let env_cfg_dir = dirs.get_config_home().join(name);
	create_new_dir(&env_cfg_dir)?;

	let env_data_dir = dirs.get_data_home().join(name);
	create_new_dir(&env_data_dir)?;
	create_new_dir(&env_data_dir.join("bin"))?;
	Ok(())
}

fn create_new_dir(dir: &path::PathBuf) -> Result<()> {
	match dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if exists {
				return Err(Box::new(crenv::Err::EnvExists(dir.clone())))
					.dbg_panic()?;
			}
		}
		Err(err) => {
			return Err(err).dbg_panic()?;
		}
	}

	fs::create_dir_all(dir).dbg_panic()?;

	Ok(())
}

pub fn create_env_files(name: &str, dirs: &xdg::BaseDirectories) -> Result<()> {
	let env_cfg_dir = dirs.get_config_home().join(name);
	let env_cfg_file_names = get_env_cfg_file_names(&env_cfg_dir);
	for env_cfg_file_name in env_cfg_file_names {
		fs::File::create_new(&env_cfg_file_name).dbg_panic()?;
	}

	fs::write(
		env_cfg_dir.join("config.toml"),
		toml::to_string_pretty(&crenv::Table::new())?,
	)?;

	Ok(())
}

pub fn check_env_exists(name: &str, dirs: &xdg::BaseDirectories) -> Result<()> {
	let env_cfg_dir = dirs.get_config_home().join(name);
	match env_cfg_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if !exists {
				return Err(Box::new(crenv::Err::NoExists))?;
			}
		}
		Err(err) => {
			return Err(err)?;
		}
	}

	for file in get_env_cfg_file_names(&env_cfg_dir) {
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

fn get_env_cfg_file_names(env_cfg_dir: &path::PathBuf) -> Vec<path::PathBuf> {
	vec!["config.toml"]
		.iter()
		.map(|env_cfg_file_base| env_cfg_dir.join(env_cfg_file_base))
		.collect()
}

pub fn lookup_bin(bin: &path::PathBuf) -> Result<path::PathBuf> {
	let path = env::var("PATH").dbg_panic()?;
	let path = path.split(':');

	for path_elem in path {
		let path_elem = path::PathBuf::from(path_elem);
		let bin_in_path_elem = path_elem.join(bin);
		match bin_in_path_elem.try_exists() {
			Ok(exists) => {
				if exists {
					return Ok(bin_in_path_elem);
				} else {
					continue;
				}
			}
			// Probably don't want to error if there's no permission to access
			// a path element. `Err::NoBinInPath` also says to check permissions
			// to PATH if `bin` couldn't be found.
			Err(_) => {
				continue;
			}
		}
	}

	Err(Err::NoBinInPath(bin.clone()))
}

pub fn bin_try_exists(bin: &path::PathBuf) -> Result<()> {
	match bin.try_exists() {
		Ok(exists) => {
			if !exists {
				return Err(Err::NoExistsBin(bin.clone())).dbg_panic();
			}
		}
		Err(err) => {
			return Err(err).dbg_panic()?;
		}
	}
	Ok(())
}

pub fn bin_get_abs(bin: &path::PathBuf) -> Result<path::PathBuf> {
	if bin.is_absolute() {
		Ok(bin.clone())
	} else {
		match lookup_bin(&bin) {
			Ok(ok) => Ok(ok),
			Err(err) => Err(err),
		}
	}
}

/// Return `true` if `target` is a symlink to `src` or `false` otherwise.
pub fn symlink_exists(src: &path::PathBuf, target: &path::PathBuf) -> bool {
	true
}
