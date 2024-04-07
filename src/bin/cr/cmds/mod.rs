use thiserror::Error;

mod new;
pub use new::new;

/// Macro to make it easier to get an `Err` which refers to the parent to make
/// error messages better. More info in [new::Err::Parent].
/// The 2nd rule is more convenient if parent `Err` variant takes just a
/// single argument and the 3rd gets formatted better with `rustfmt` when using
/// the 80 character per line limit because it splits/wraps the arguments from
/// the variant identifier.
macro_rules! errpb {
	($scope:path, $error:expr) => {
		Err($scope(Err::Parent(Box::new($error))))
	};

	//
	($scope:path, $error:path) => {
		Err($scope(Err::Parent(Box::new($error))))
	};

	($scope:path, $error:path, $($error_args:expr),+) => {
		Err($scope(Err::Parent(Box::new($error($($error_args),+)))))
	};
}

pub(self) use errpb;

#[derive(Debug, Error)]
pub enum Err {
	#[error(transparent)]
	New(#[from] new::Err),
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error("Directory already exists")]
	DirExists(std::path::PathBuf),
}
