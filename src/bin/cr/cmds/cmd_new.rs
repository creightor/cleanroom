// use super::errpb;
use super::Err as CmdErr;
use crate::args;
use crate::cfg;
use crate::debug::DebugPanic;
use crate::files;
// use std::path;
// use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {}

pub fn cmd_new(cfg: cfg::Cfg) -> Result<(), CmdErr> {
	files::create_config_dir(&cfg.dirs)?;

	match cfg.args.sub {
		args::CmdMainSub::New { name } => match name {
			Some(name) => new_env(&name, cfg.dirs),
			None => todo!("generate random env name"),
		},

		_ => Err(CmdErr::SubCmdMatch),
	}
}

fn new_env(name: &str, xdg_dirs: xdg::BaseDirectories) -> Result<(), CmdErr> {
	let cfg_home = xdg_dirs.get_config_home();
	let env_dir = cfg_home.join(name);
	files::create_env_dir(env_dir.clone()).dbg_panic()?;
	let env_files = files::create_env_files(env_dir.clone())?;

	println!("Created new environment {name} in {:?}", env_dir);

	let _ = env_files;
	Ok(())
}
