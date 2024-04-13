//! Macros and `impl`s for debugging.

// Panic on debug builds or return `Result<T, E>` on release builds.
pub trait DebugPanic<T, E>
where
	E: std::fmt::Display + std::fmt::Debug,
{
	// Short for "debug panic"
	fn dp(self) -> Result<T, E>;
}

impl<T, E> DebugPanic<T, E> for Result<T, E>
where
	E: std::fmt::Display + std::fmt::Debug,
{
	fn dp(self) -> Result<T, E> {
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

// `dbg!` doesn't use `Display` so on debug builds print using `println!`.
macro_rules! dbgfmt {
	($($exs:expr),+) => {
		if cfg!(debug_assertions) {
			println!($($exs),+);
		}
	};
}
pub(crate) use dbgfmt;
