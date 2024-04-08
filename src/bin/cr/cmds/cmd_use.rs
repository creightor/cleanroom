use super::Err as CmdErr;
use crate::debug::DebugPanic;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
	#[error("Environment {0} doesn't exist")]
	NoEnvExists(String),
}

pub fn cmd_use(cfg: crate::cfg::Cfg) -> Result<(), CmdErr> {
	match cfg.args.sub {
		crate::args::CmdMainSub::Use { name } => match name {
			Some(name) => use_env(&name, cfg.dirs),
			None => todo!("use an environment if there is just one, otherwise return an error")
		},

		_ => Err(CmdErr::SubCmdMatch),
	}
}

fn use_env(name: &str, xdg_dirs: xdg::BaseDirectories) -> Result<(), CmdErr> {
	let cfg_home = xdg_dirs.get_config_home();
	let env_dir = cfg_home.join(name);
	match env_dir.try_exists().dbg_panic() {
		Ok(exists) => {
			if !exists {
				return Err(CmdErr::Use(Err::NoEnvExists(name.to_string())));
			}
		}
		Err(err) => {
			return Err(CmdErr::IO(err));
		}
	}
	println!("Using environment {name}");

	let _ = env_dir;

	Ok(())
}
