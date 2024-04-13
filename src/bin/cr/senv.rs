//! Structs and methods for storing and operating on files related to the
//! environments.

use std::fs;
use std::io;
use std::path;

use thiserror::Error;

use crate::debug::{dbgfmt, DebugPanic};
use crate::table;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	IO(#[from] io::Error),
	#[error(transparent)]
	TOMLSerialize(#[from] toml::ser::Error),

	#[error("Directory '{1}' doesn't exist for environment '{0}'")]
	MissingDir(String, path::PathBuf),
	#[error("File '{1}' doesn't exist for environment '{0}'")]
	MissingFile(String, path::PathBuf),
}

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Senv {
	pub name: String,
	pub files: Files,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Files {
	pub cfg_dir: path::PathBuf,
	pub cfg_file: path::PathBuf,
	pub data_dir: path::PathBuf,
	pub bin_dir: path::PathBuf,
}

impl Senv {
	pub fn new_xdg(name: &str, dirs: &xdg::BaseDirectories) -> Result<Self> {
		let name = String::from(name);
		let cfg_dir = dirs.get_config_home().join(&name);
		let cfg_file = cfg_dir.join("config.toml");
		let data_dir = dirs.get_data_home().join(&name);
		let bin_dir = data_dir.join("bin");
		Ok(Senv {
			name,
			files: Files {
				cfg_dir,
				cfg_file,
				data_dir,
				bin_dir,
			},
		})
	}

	pub fn create_xdg(self) -> Result<Self> {
		fs::create_dir_all(&self.files.cfg_dir).dp()?;
		fs::create_dir_all(&self.files.data_dir).dp()?;
		fs::create_dir_all(&self.files.bin_dir).dp()?;
		fs::File::create_new(&self.files.cfg_file).dp()?;
		fs::write(
			&self.files.cfg_file,
			toml::to_string_pretty(&table::Root::new()).dp()?,
		)
		.dp()?;
		Ok(self)
	}

	pub fn create_new_xdg(
		name: &str,
		dirs: &xdg::BaseDirectories,
	) -> Result<Self> {
		Ok(Self::new_xdg(name, dirs).dp()?.create_xdg().dp()?)
	}

	pub fn rm(self) -> Result<()> {
		fs::remove_dir_all(&self.files.cfg_dir).dp()?;
		fs::remove_dir_all(&self.files.data_dir).dp()?;

		Ok(())
	}

	pub fn is_valid(&self) -> Result<()> {
		if !self.files.cfg_dir.try_exists().dp()? {
			return Err(Err::MissingDir(
				self.name.clone(),
				self.files.cfg_dir.clone(),
			))
			.dp();
		}

		if !self.files.cfg_file.try_exists().dp()? {
			return Err(Err::MissingFile(
				self.name.clone(),
				self.files.cfg_file.clone(),
			))
			.dp();
		}

		if !self.files.data_dir.try_exists().dp()? {
			return Err(Err::MissingDir(
				self.name.clone(),
				self.files.data_dir.clone(),
			))
			.dp();
		}

		if !self.files.bin_dir.try_exists().dp()? {
			return Err(Err::MissingDir(
				self.name.clone(),
				self.files.bin_dir.clone(),
			))
			.dp();
		}
		Ok(())
	}

	pub fn get_vec(dirs: &xdg::BaseDirectories) -> Result<Vec<Self>> {
		let mut shell_envs: Vec<Self> = Vec::new();

		let files = fs::read_dir(dirs.get_config_home()).dp()?;
		for file in files {
			if let Err(err) = file {
				dbgfmt!("{}:{} {}", file!(), line!(), err);
				continue;
			}
			let file = file.unwrap();

			let meta = file.metadata();
			if let Err(err) = meta {
				dbgfmt!("{}:{} {}", file!(), line!(), err);
				continue;
			}
			let meta = meta.unwrap();

			if !meta.is_dir() {
				continue;
			}

			let file_name = file.file_name();
			let file_name = file_name.to_str();
			if let None = file_name {
				dbgfmt!(
					"{}:{} {}",
					file!(),
					line!(),
					"Couldn't convert file name to `&str`"
				);
				continue;
			}
			let file_name = file_name.unwrap();

			let shell_env = Senv::new_xdg(file_name, dirs);
			if let Err(_) = shell_env {
				continue;
			}
			let shell_env = shell_env.unwrap();

			if let Err(_) = shell_env.is_valid() {
				continue;
			}

			shell_envs.push(shell_env);
		}
		Ok(shell_envs)
	}
}
