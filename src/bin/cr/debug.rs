#[cfg(test)]
mod tests;

pub trait DebugPanic<T, E>
where
	E: std::fmt::Display + std::fmt::Debug,
{
	fn dbg_panic(self) -> Result<T, E>;
}

impl<T, E> DebugPanic<T, E> for Result<T, E>
where
	E: std::fmt::Display + std::fmt::Debug,
{
	fn dbg_panic(self) -> Result<T, E> {
		if cfg!(debug_assertions) {
			if let Err(err) = self {
				panic!("{err}");
			} else {
				self
			}
		} else {
			self
		}
	}
}
