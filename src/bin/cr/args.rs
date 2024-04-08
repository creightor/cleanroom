use clap::{Args, Parser, Subcommand};

#[cfg(test)]
mod test;

#[derive(Debug, Parser)]
#[command(about, version, arg_required_else_help(true), max_term_width(80))]
pub struct CmdMain {
	#[command(subcommand)]
	pub sub: CmdMainSub,

	#[command(flatten)]
	pub args: CmdMainArgs,
}

#[derive(Debug, Subcommand)]
pub enum CmdMainSub {
	/// Create a new environment
	New {
		/// Environment name
		///
		/// TODO: If not entered, generate a random name.
		#[arg(short, long)]
		name: Option<String>,
	},

	/// Start using an environment
	Use {
		/// Environment name
		///
		/// TODO: If not entered and there is only one, use that one.
		#[arg(short, long)]
		name: Option<String>,
	},
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
