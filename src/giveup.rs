use crate::hint::HintedError;
use colored::Colorize;

/// User-geared program termination.
pub trait Giveup<T, E>
where
	E: std::fmt::Display + Send + Sync,
{
	/// Terminate the program gracefully and display a user-geared
	/// error message.
	/// # Example
	/// ```rust
	/// use std::io;
	/// use giveup::Giveup;
	/// fn main() {
	/// 	let mut input = String::new();
	/// 	io::stdin().read_line(&mut input)
	/// 	// Instead of panicing a user-geared error message is displayed
	/// 		.giveup("Failed to read input");
	/// }
	/// ```
	fn giveup(self, msg: &str) -> T;
	/// Add hints to errors to help users solve the issue which
	/// raised the error.
	///
	/// [`example`](crate::hint::Example::example) can be called on
	/// `Result`s returned by this method to subsidize the hint with
	/// an example of the recommended action.
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
