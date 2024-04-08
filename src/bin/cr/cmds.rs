use thiserror::Error;

mod cmd_new;
pub use cmd_new::cmd_new;
mod cmd_use;
pub use cmd_use::cmd_use;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	New(#[from] cmd_new::Err),
	#[error(transparent)]
	Use(#[from] cmd_use::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error("Environment already exists at '{0}'")]
	EnvExists(std::path::PathBuf),

	#[error("Wrong function for subcommand")]
	SubCmdMatch,
}
