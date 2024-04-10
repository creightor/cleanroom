use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
	#[error("`malloc()` returned NULL")]
	Malloc,
	#[error("`sigaddset()` returned non-zero")]
	SigAddSet,
	#[error("`sigwait()` returned non-zero")]
	SigWait,
}
