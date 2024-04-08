use crate::cmds::Err;
use crate::debug::DebugPanic;
use std::collections::HashMap;
use std::fs;
use std::path;

#[cfg(test)]
mod tests;

pub fn create_config_dir(dirs: &xdg::BaseDirectories) -> Result<(), Err> {
	let cfg_dir = dirs.get_config_home();
	match cfg_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if exists {
				return Ok(());
			}
			match std::fs::create_dir(cfg_dir).dbg_panic() {
				Ok(_) => Ok(()),
				Err(err) => Err(Err::IO(err)),
			}
		}
		Err(err) => Err(Err::IO(err)),
	}
}

pub fn create_env_dir(env_dir: path::PathBuf) -> Result<(), Err> {
	match env_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if exists {
				return Err(Err::EnvExists(env_dir));
			}
		}
		Err(err) => {
			return Err(Err::IO(err));
		}
	}

	if let Err(err) = std::fs::create_dir_all(env_dir).dbg_panic() {
		return Err(Err::IO(err));
	}

	Ok(())
}

pub fn create_env_files(
	env_dir: path::PathBuf,
) -> Result<HashMap<path::PathBuf, fs::File>, Err> {
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
				return Err(Err::IO(err));
			}
		}
	}

	Ok(env_files)
}
