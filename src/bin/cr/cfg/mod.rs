use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Cfg {}

#[derive(Debug, Error)]
pub enum CfgErr {}

impl Cfg {
	pub fn from() -> Result<Self, CfgErr> {
		Ok(Cfg {})
	}
}
