//! Structs and methods for storing and operating on files related to the
//! environments.

use std::fs;
use std::io;
use std::path;

use thiserror::Error;

use crate::debug::DebugPanic;
use crate::table;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	IO(#[from] io::Error),
	#[error(transparent)]
	TOMLSerialize(#[from] toml::ser::Error),
}

pub struct Senv {
	pub name: String,
	pub files: Files,
}

pub struct Files {
	pub cfg_dir: path::PathBuf,
	pub cfg_file: path::PathBuf,
	pub data_dir: path::PathBuf,
	pub bin_dir: path::PathBuf,
}

impl Senv {
	pub fn new_xdg(name: &str, dirs: &xdg::BaseDirectories) -> Result<Self> {
		let name = String::from(name);
		let cfg_dir = dirs.get_config_home().join(&name).canonicalize().dp()?;
		let cfg_file = cfg_dir.join("config.toml").canonicalize().dp()?;
		let data_dir = dirs.get_data_home().join(&name).canonicalize().dp()?;
		let bin_dir = data_dir.join("bin").canonicalize().dp()?;
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
}
