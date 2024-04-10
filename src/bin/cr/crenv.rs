use std::path;

use serde::Deserialize;
use thiserror::Error;

use crate::files;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error("Environment doesn't exist")]
	NoExists,
	#[error("Environment already exists at '{0}'")]
	EnvExists(path::PathBuf),
}

#[derive(Debug, Deserialize)]
pub struct Table {
	vars: TableVars,
}

#[derive(Debug, Deserialize)]
pub struct TableVars {
	inherit: Vec<String>,
}

impl Table {
	pub fn from_env(dirs: xdg::BaseDirectories) -> Result<Self> {
		// files::check_env_exists(env_dir);
		Ok(Table {
			vars: TableVars {
				inherit: Vec::new(),
			},
		})
	}
}
