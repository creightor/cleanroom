//! Structs that are deserialized from the `config.toml` config file.

use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::os;
use std::path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::debug::{dbgfmt, DebugPanic};
use crate::files;
use crate::macros::pathbuf;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error(transparent)]
	Files(#[from] files::Err),
	#[error(transparent)]
	TomlDeserialize(#[from] toml::de::Error),
	#[error("Didn't find environment variable '{0}' in parent")]
	EnvVarNotPresent(String),
	#[error("Environment variable doesn't contain valid unicode: ''")]
	EnvVarNotUnicode(std::ffi::OsString),

	#[error("Environment doesn't exist")]
	NoExists,
	#[error("Environment file '{0}' doesn't exist")]
	NoExistsEnvFile(path::PathBuf),
	#[error("Environment already exists at '{0}'")]
	EnvExists(path::PathBuf),
	#[error("Binary '{0}' doesn't exist on host")]
	NoExistsBinHost(path::PathBuf),
	#[error("Binary '{0}' has invalid path, terminates with '..'")]
	BinTermParent(path::PathBuf),
	#[error("Found non-symlink binary '{0}'")]
	BinNotSymlink(path::PathBuf),
	#[error(
		"While trying to inherit '{0}' found already existing link for '{0}'
		which points to '{1}' but in config file '{0}' is defined to point to
		'{2}'. Exiting because `exit_on_change` is true."
	)]
	BinChanged(path::PathBuf, path::PathBuf, path::PathBuf),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default = "default_root")]
pub struct Root {
	pub shell: Shell,
	pub vars: Vars,
	pub bin: Bin,
}

fn default_root() -> Root {
	Root {
		shell: default_shell(),
		vars: default_vars(),
		bin: default_bin(),
	}
}

impl Root {
	pub fn new() -> Self {
		default_root()
	}

	/// Deserialize from the environment's config.toml.
	pub fn from_env(name: &str, dirs: &xdg::BaseDirectories) -> Result<Self> {
		let env_cfg_home = dirs.get_config_home();
		let env_dir = env_cfg_home.join(name);

		files::check_env_exists(name, dirs).dp()?;

		let env_cfg: Self = toml::from_str(
			&std::fs::read_to_string(env_dir.join("config.toml")).dp()?,
		)
		.dp()?;

		Ok(env_cfg)
	}

	/// Return a `Vec` of arguments to be used for a shell based on the
	/// environment's config.toml.
	pub fn get_shell_args(
		&self,
		name: &str,
		dirs: &xdg::BaseDirectories,
	) -> Result<Vec<String>> {
		let env_dir = dirs.get_config_home().join(name);
		let mut args: Vec<String> = Vec::new();

		if self.shell.noprofile {
			args.push("--noprofile".to_string());
		}

		let rc_file = env_dir.join("rc.sh");
		let rc_file = rc_file
			.to_str()
			.ok_or(files::Err::PathToStr)
			.dp()?
			.to_string();
		if !self.shell.norc && self.shell.interactive {
			args.push("--rcfile".to_string());
			args.push(rc_file);
			args.push("-i".to_string());
		} else {
			args.push("--norc".to_string());
		}

		if self.shell.login {
			args.push("-l".to_string());
		}

		Ok(args)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default = "default_shell")]
// TODO: add field for prompt and use it as a template to generate the prompt.
pub struct Shell {
	pub bin: String,
	pub login: bool,
	pub interactive: bool,
	pub noprofile: bool,
	pub norc: bool,
}

fn default_shell() -> Shell {
	Shell {
		bin: String::from("/bin/sh"),
		login: false,
		interactive: true,
		noprofile: true,
		norc: false,
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default = "default_vars")]
// TODO: Add field `clear: bool` if environment variables should be cleared.
pub struct Vars {
	/// Environment variables that will be inherited from the parent process.
	pub inherit: Vec<String>,

	/// Whether to exit when the variable which is supposed to be inherited is
	/// missing from the parent environment.
	pub exit_on_missing: bool,

	/// Environment variables custom to this config/started shell.
	/// Overrides the ones set in `inherit`.
	pub set: HashMap<String, String>,
}

fn default_vars() -> Vars {
	Vars {
		inherit: Vec::new(),
		exit_on_missing: true,
		set: HashMap::new(),
	}
}

impl Vars {
	/// Return the key, value pair for environment variables.
	pub fn to_env(&self) -> Result<HashMap<String, String>> {
		let mut vars = HashMap::<String, String>::new();

		if self.exit_on_missing {
			for var in self.inherit.clone() {
				let val = match env::var(&var) {
					Ok(ok) => ok,
					Err(err) => match err {
						env::VarError::NotPresent => {
							return Err(Err::EnvVarNotPresent(var)).dp();
						}
						env::VarError::NotUnicode(data) => {
							return Err(Err::EnvVarNotUnicode(data)).dp();
						}
					},
				};

				vars.insert(var, val);
			}
		} else {
			for var in self.inherit.clone() {
				let val = env::var(var.clone());
				if let Err(_) = val {
					continue;
				}
				vars.insert(var, val.unwrap());
			}
		}

		vars.extend(self.set.clone());

		Ok(vars)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default = "default_bin")]
pub struct Bin {
	/// Directories to add to PATH.
	pub inherit_dirs: Vec<path::PathBuf>,

	/// List of binaries to inherit from host, if the element starts with '/'
	/// assume it's an absolute path for the binary, otherwise lookup the path
	/// and use whatever is the result.
	pub inherit: Vec<path::PathBuf>,

	/// Whether to exit if the binary is already symlinked for the environment
	/// but points to a different file than what was symlinked to initially.
	/// Only applies when the element in `self.inherit` isn't an absolute path.
	pub exit_on_change: bool,

	/// Whether to exit if the element in `self.inherit` isn't an absolute path
	/// and it wasn't found in PATH.
	pub exit_on_not_found: bool,
}

fn default_bin() -> Bin {
	Bin {
		inherit_dirs: pathbuf!("/usr/local/bin", "/bin", "/usr/bin"),
		inherit: Vec::new(),
		exit_on_change: true,
		exit_on_not_found: true,
	}
}

impl Bin {
	/// Inherit/Symlink binaries listed in `self.inherit` from the host.
	pub fn inherit_bins(&self, env_data_dir: &path::PathBuf) -> Result<()> {
		for host_bin in self.inherit.clone() {
			let host_bin_abs = match files::bin_get_abs(&host_bin) {
				Ok(ok) => ok,
				Err(files::Err::NoExistsBin(bin)) => {
					return Err(files::Err::NoExistsBin(bin)).dp()?;
				}
				Err(err) => {
					return Err(err).dp()?;
				}
			};

			files::bin_try_exists(&host_bin_abs).dp()?;

			let env_bin_base = path::PathBuf::from(
				host_bin
					.file_name()
					.ok_or(Err::BinTermParent(host_bin.clone()))
					.dp()?,
			);
			let env_bin_dir = env_data_dir.join("bin");
			let env_bin_abs = env_bin_dir.join(env_bin_base);

			self.link(&host_bin_abs, &env_bin_abs).dp()?;
		}
		Ok(())
	}

	// Create the symlink `target` pointing to `src` and if `target` already
	// exists, check if it points to `src`, if it doesn't, return an error based
	// on what `self.exit_on_change` is.
	fn link(&self, src: &path::PathBuf, target: &path::PathBuf) -> Result<()> {
		let target_base = target
			.file_name()
			.ok_or(Err::BinTermParent(target.clone()))
			.dp()?;

		if target.try_exists().dp()? {
			if !fs::symlink_metadata(target).dp()?.is_symlink() {
				return Err(Err::BinNotSymlink(target.clone())).dp();
			}

			let orig_link = fs::read_link(target).dp()?;
			match orig_link.cmp(src) {
				cmp::Ordering::Equal => Ok(()),
				_ => {
					if self.exit_on_change {
						Err(Err::BinChanged(
							path::PathBuf::from(target_base),
							orig_link,
							src.clone(),
						))
						.dp()
					} else {
						Ok(())
					}
				}
			}
		} else {
			dbgfmt!("Creating symlink {:?} from {:?}", target, src);
			os::unix::fs::symlink(src, target).dp()?;
			Ok(())
		}
	}
}
