// Panic on debug builds or return `Result<T, E>` on release builds.
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

// Panic with `todo!` on debug builds or return `cmds::Err::TODO` with msg on
// release builds.
macro_rules! todom {
	($msg:literal) => {
		if cfg!(debug_assertions) {
			todo!($msg);
		} else {
			return Err(crate::cmds::Err::TODO($msg.to_string()));
		}
	};
}

pub(crate) use todom;

// `dbg!` doesn't use `Display` so on debug builds print using `println!`.
macro_rules! dbgfmt {
	($($exs:expr),*) => {
		if cfg!(debug_assertions) {
			println!($($exs),*);
		}
	};

}

pub(crate) use dbgfmt;
