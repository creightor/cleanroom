use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Cfg {
	pub args: crate::args::CmdMain,
	pub dirs: xdg::BaseDirectories,
}

#[derive(Debug, Error)]
pub enum Err {}

impl Cfg {
	pub fn from(
		args: crate::args::CmdMain,
		dirs: xdg::BaseDirectories,
	) -> Result<Self, Err> {
		Ok(Cfg { args, dirs })
	}
}
