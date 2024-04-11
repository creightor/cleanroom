use std::collections::HashMap;
use std::env;
use std::path;

use serde::Deserialize;
use thiserror::Error;

use crate::debug::DebugPanic;
use crate::files;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error(transparent)]
	Files(#[from] files::Err),
	#[error(transparent)]
	TomlDeserialize(#[from] toml::de::Error),
	#[error("didn't find environment variable '{0}' in parent")]
	EnvVarNotPresent(String),
	#[error("environment variable doesn't contain valid unicode: ''")]
	EnvVarNotUnicode(std::ffi::OsString),

	#[error("environment doesn't exist")]
	NoExists,
	#[error("environment file '{0}' doesn't exist")]
	NoExistsEnvFile(path::PathBuf),
	#[error("environment already exists at '{0}'")]
	EnvExists(path::PathBuf),
}

#[derive(Debug, Deserialize)]
#[serde(default = "default_table")]
pub struct Table {
	pub shell: TableShell,
	pub vars: TableVars,
}

fn default_table() -> Table {
	Table {
		shell: default_table_shell(),
		vars: default_table_vars(),
	}
}

#[derive(Debug, Deserialize)]
#[serde(default = "default_table_shell")]
pub struct TableShell {
	pub bin: String,
	pub login: bool,
	pub interactive: bool,
	pub noprofile: bool,
	pub norc: bool,
}

fn default_table_shell() -> TableShell {
	TableShell {
		bin: String::from("/bin/sh"),
		login: false,
		interactive: true,
		noprofile: true,
		norc: false,
	}
}

#[derive(Debug, Deserialize)]
#[serde(default = "default_table_vars")]
pub struct TableVars {
	/// Environment variables that will be inherited from the parent process.
	pub inherit: Vec<String>,

	/// Whether to exit when the variable which is supposed to be inherited is
	/// missing from the parent environment.
	pub exit_on_missing: bool,

	/// Environment variables custom to this config/started shell.
	/// Overrides the ones set in `inherit`.
	pub set: HashMap<String, String>,
}

fn default_table_vars() -> TableVars {
	TableVars {
		inherit: Vec::new(),
		exit_on_missing: true,
		set: HashMap::new(),
	}
}

impl TableVars {
	/// Return the key, value pair for environment variables.
	pub fn to_env(&self) -> Result<HashMap<String, String>> {
		let mut vars = HashMap::<String, String>::new();

		if self.exit_on_missing {
			for var in self.inherit.clone() {
				let val = match env::var(&var) {
					Ok(ok) => ok,
					Err(err) => match err {
						env::VarError::NotPresent => {
							return Err(Err::EnvVarNotPresent(var));
						}
						env::VarError::NotUnicode(data) => {
							return Err(Err::EnvVarNotUnicode(data))
						}
					},
				};

				vars.insert(var, val);
			}
		} else {
			for var in self.inherit.clone() {
				let val = env::var(var.clone());
				if let Err(err) = val {
					continue;
				}
				vars.insert(var, val.unwrap());
			}
		}

		vars.extend(self.set.clone());

		Ok(vars)
	}
}

impl Table {
	pub fn from_env(name: &str, dirs: &xdg::BaseDirectories) -> Result<Self> {
		let env_cfg_home = dirs.get_config_home();
		let env_dir = env_cfg_home.join(name);

		files::check_env_exists(&env_dir).dbg_panic()?;

		let env_cfg: Table = toml::from_str(
			&std::fs::read_to_string(env_dir.join("config.toml"))
				.dbg_panic()?,
		)
		.dbg_panic()?;

		Ok(env_cfg)
	}
}
