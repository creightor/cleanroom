use thiserror::Error;

use crate::args;
use crate::debug::DebugPanic;
use crate::senv;
use crate::table;

type Result<T> = std::result::Result<T, Err>;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	ShellEnv(#[from] senv::Err),
	#[error(transparent)]
	Table(#[from] table::Err),
}

pub fn cmd_ls(
	_args_main: args::CmdMainArgs,
	args_ls: args::SubCmdLsArgs,
	dirs: xdg::BaseDirectories,
) -> Result<()> {
	let mut shell_envs = senv::Senv::get_vec(&dirs)?;
	shell_envs.sort();

	let mut rows: Vec<Vec<String>> = Vec::new();

	for shell_env in shell_envs {
		rows.push(row_from_args(&args_ls, &shell_env, &dirs)?);
	}

	for row in rows {
		println!("{}", row.join(","));
	}

	// println!("{}", shell_env.name);

	Ok(())
}

fn row_from_args(
	args: &args::SubCmdLsArgs,
	shell_env: &senv::Senv,
	dirs: &xdg::BaseDirectories,
) -> Result<Vec<String>> {
	let env_table = table::Root::from_env(&shell_env.name, dirs).dp()?;

	let mut row: Vec<String> = Vec::new();
	row.push(shell_env.name.clone());

	if args.shell {
		row.push(env_table.shell.bin);
		let mut mode = String::new();
		if env_table.shell.interactive {
			mode.push_str("i");
		}
		row.push(mode);
	}

	Ok(row)
}
