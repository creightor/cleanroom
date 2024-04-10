use std::mem;
use std::path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Err as CmdErr;
use crate::args;
use crate::c;
use crate::debug::{dbgfmt, todom, DebugPanic};
use crate::files;

type PResult<T> = std::result::Result<T, CmdErr>;
type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error("Couldn't fork")]
	Fork,
	#[error("Got unexpected signal from `wait_for_sig()`: {0}")]
	UnexpectedSig(libc::c_int),
	#[error(transparent)]
	Files(#[from] files::Err),
	#[error(transparent)]
	C(#[from] c::Err),
}

pub fn cmd_use(
	args_main: args::CmdMainArgs,
	args_use: args::SubCmdUseArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	let cfg_home = dirs.get_config_home();

	let env_dir = cfg_home.join(&args_use.name);
	files::check_env_exists(&env_dir).dbg_panic()?;

	unsafe {
		let parent_pid = libc::getpid();
		// On success - child PID for parent, 0 for child
		// On failure - -1 for parent, no child created
		let ret_fork = libc::fork();

		if ret_fork == -1 {
			return Err(Err::Fork);
		}

		// Child
		if ret_fork == 0 {
			if let Err(err) =
				setup_child_env(&args_main, &args_use, &dirs, &parent_pid)
			{
				libc::kill(parent_pid, libc::SIGINT);
				return Err(err);
			}

			libc::kill(parent_pid, libc::SIGUSR1);
		}

		// Parent
		if ret_fork != 0 {
			let ret_sig = wait_for_sig(libc::SIGUSR1)?;
			match ret_sig {
				libc::SIGUSR1 => (),
				libc::SIGINT => {
					return Ok(());
				}
				_ => {
					return Err(Err::UnexpectedSig(ret_sig));
				}
			}
		}
	}
	println!("done");

	Ok(())
}

// Setup the environment and signal `SIGUSR1` to the parent when done.
fn setup_child_env(
	args_main: &args::CmdMainArgs,
	args_use: &args::SubCmdUseArgs,
	dirs: &xdg::BaseDirectories,
	parent_pid: &libc::c_int,
) -> Result<()> {
	let env_dir = dirs.get_config_home().join(&args_use.name);

	std::thread::sleep(std::time::Duration::from_secs(2));
	Ok(())
}

// Wait for the `sig` signal without ignoring `SIGINT`.
fn wait_for_sig(sig: libc::c_int) -> Result<libc::c_int> {
	let ret_sig: *mut libc::c_int;
	unsafe {
		let sigset: *mut libc::sigset_t;
		sigset = libc::malloc(mem::size_of::<libc::sigset_t>())
			as *mut libc::sigset_t;
		if sigset.is_null() {
			return Err(c::Err::Malloc).dbg_panic()?;
		}

		let mut ret;
		libc::sigemptyset(sigset);
		if sigset.is_null() {
			return Err(c::Err::Malloc).dbg_panic()?;
		}

		ret = libc::sigaddset(sigset, sig);
		if ret != 0 {
			return Err(c::Err::SigAddSet).dbg_panic()?;
		}
		ret = libc::sigaddset(sigset, libc::SIGINT);
		if ret != 0 {
			return Err(c::Err::SigAddSet).dbg_panic()?;
		}

		ret_sig =
			libc::malloc(mem::size_of::<libc::c_int>()) as *mut libc::c_int;
		if ret_sig.is_null() {
			return Err(c::Err::Malloc).dbg_panic()?;
		}

		ret = libc::sigwait(sigset, ret_sig);
		if ret != 0 {
			return Err(c::Err::SigWait).dbg_panic()?;
		}
	}

	unsafe { Ok(*ret_sig) }
}
