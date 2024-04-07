use super::errpb;
use super::Err as CmdErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
	#[error("subcommand didn't match `New`")]
	SubCmdMatch,

	/// This is probably stupid but this so that when returning the "generic"
	/// error from `CmdErr` and it eventually is returned from `main()`, wrapped
	/// into all the other errors, it can be possible to differentiate that
	/// the error is coming from the `new` module.
	///
	/// For example:
	///
	/// `Cmd(New(Parent(DirExists("/Users/usr/.config/cleanroom/"))))`
	///
	/// instead of
	///
	/// `Cmd(DirExists("/Users/usr/.config/cleanroom/"))`
	#[error(transparent)]
	Parent(#[from] Box<CmdErr>),
}

pub fn new(cfg: crate::cfg::Cfg) -> Result<(), CmdErr> {
	match cfg.args.sub {
		crate::args::CmdMainSub::New { name } => match name {
			Some(name) => new_env(&name, cfg.dirs),
			None => todo!("generate random env name"),
		},

		#[allow(unreachable_patterns)]
		_ => Err(CmdErr::New(Err::SubCmdMatch)),
	}
}

fn new_env(name: &str, dirs: xdg::BaseDirectories) -> Result<(), CmdErr> {
	let env_dir = dirs.get_config_home().join(name);
	match env_dir.try_exists() {
		Ok(exists) => {
			if exists {
				return errpb!(CmdErr::New, CmdErr::DirExists(env_dir));
			}
		}
		Err(err) => {
			return errpb!(CmdErr::New, CmdErr::IO(err));
		}
	}

	if let Err(err) = std::fs::create_dir(&env_dir) {
		return errpb!(CmdErr::New, CmdErr::IO(err));
	}
	println!("created {:?}", env_dir);
	Ok(())
}
