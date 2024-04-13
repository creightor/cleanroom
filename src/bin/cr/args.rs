//! Argument parsing.

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, version, arg_required_else_help(true), max_term_width(80))]
pub struct CmdMain {
	#[command(subcommand)]
	pub sub: CmdMainSub,

	#[command(flatten)]
	pub args: CmdMainArgs,
}

#[derive(Debug, Subcommand)]
// TODO: Add command for resolving `crenv::Err::BinChanged` conflicts.
pub enum CmdMainSub {
	/// Create a new environment.
	#[command(arg_required_else_help = true)]
	New {
		#[command(flatten)]
		args: SubCmdNewArgs,
	},

	/// Start using an environment.
	#[command(arg_required_else_help = true)]
	Use {
		#[command(flatten)]
		args: SubCmdUseArgs,
	},

	/// Remove the files and directories created by the `new` sub-command.
	#[command(arg_required_else_help = true)]
	Rm {
		#[command(flatten)]
		args: SubCmdRmArgs,
	},
}

#[derive(Debug, Args)]
pub struct SubCmdNewArgs {
	/// Environment name
	#[arg(value_name = "ENV_NAME")]
	pub name: String,
}

#[derive(Debug, Args)]
pub struct SubCmdUseArgs {
	/// Environment name
	#[arg(value_name = "ENV_NAME")]
	pub name: String,
}

#[derive(Debug, Args)]
pub struct SubCmdRmArgs {
	/// Environment name
	#[arg(value_name = "ENV_NAME")]
	pub name: String,
}

#[derive(Debug, Args)]
#[command(about)]
pub struct CmdMainArgs {
	/// Disable inheritance
	#[arg(short = 'I', long, default_value_t = false)]
	no_inherit: bool,
}

impl CmdMain {
	pub fn from_parse() -> Self {
		CmdMain::parse()
	}
}
