//! Commands to be called after parsing the arguments.

use thiserror::Error;

use crate::crenv;

mod cmd_new;
pub use cmd_new::cmd_new;
mod cmd_use;
pub use cmd_use::cmd_use;
mod cmd_rm;
pub use cmd_rm::cmd_rm;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	Env(#[from] crenv::Err),
	#[error(transparent)]
	New(#[from] cmd_new::Err),
	#[error(transparent)]
	Use(#[from] cmd_use::Err),
	#[error(transparent)]
	Rm(#[from] cmd_rm::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
}
