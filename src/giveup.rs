use crate::hint::HintedError;
use colored::Colorize;

pub trait Giveup<T, E>
where
	E: std::fmt::Display + Send + Sync,
{
	fn giveup(self, msg: &str) -> T;
	fn hint(self, hint: &str) -> Result<T, HintedError<E>>;
}

impl<T, E> Giveup<T, E> for Result<T, E>
where
	E: std::fmt::Display + Send + Sync,
{
	fn giveup(self, msg: &str) -> T {
		match self {
			Ok(t) => t,
			Err(e) => exit_gracefully(msg, &e),
		}
	}

	fn hint(self, hint: &str) -> Result<T, HintedError<E>> {
		match self {
			Ok(t) => Ok(t),
			Err(e) => Err(HintedError::from_hint(e, hint)),
		}
	}
}

fn exit_gracefully(
	msg: &str,
	error: &(dyn std::fmt::Display + Send + Sync)
) -> ! {
	eprintln!("{}: {}", msg.bold(), error);
	std::process::exit(1);
}
