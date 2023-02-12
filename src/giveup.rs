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
	E: std::error::Error + Send + Sync,
{
	fn giveup(self, msg: &str) -> T {
		match self {
			Ok(t) => t,
			Err(e) => {
				let err_msg = format_err_msg(&e);
				exit_gracefully(msg, &err_msg)
			}
		}
	}

	fn hint(self, hint: &str) -> Result<T, HintedError<E>> {
		match self {
			Ok(t) => Ok(t),
			Err(e) => Err(HintedError::from_hint(e, hint)),
		}
	}
}


fn exit_gracefully<S>(msg: S, err_msg: S) -> !
where
	S: AsRef<str>
{
	let msg: &str = msg.as_ref();
	let err_msg: &str = err_msg.as_ref();
	// err_msg contains a trailing newline so and 
	// additional newline is omitted here.
	eprint!("{}: {}", msg.bold(), err_msg);
	std::process::exit(1);
}

/// Format an error to display its contents to a CLI user.
fn format_err_msg(
	error: &(dyn std::error::Error + Send + Sync),
) -> String {
	// Error message starts with the display implementation.
	let mut err_msg = format!("{error}\n");  

	// Add the error messages of the original's sources to the message.
	let mut current = error.source();
	while let Some(cause) = current {
		let cause_msg = format!("Caused by: {cause}\n");
		err_msg.push_str(&cause_msg);
		// Get option to next source.
		current = cause.source();
	}

	err_msg
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::error::Error;
	use std::fmt::{self, Display};

	/*
	The `giveup` method itself will always print to stderr and exit.
	Because of this testing if the error messages are correct is
	accomplish by testing the `format_err_msg` function.
	*/

	const FLAT_SRC_MSG: &str = "I am the message of a flag error without source";
	const SINGLE_SRC_MSG: &str = "I am the message of an error with a single source";
	const MULTI_SRC_MSG: &str = "I am the message of an error with more than once sources";
	const HINT_MSG: &str = "I am a hint.";
	const EXAMPLE_MSG: &str = "I am an example.";

	// Flat test error.
	#[derive(Debug)]
	struct FlatErr {}
	impl Error for FlatErr {}
	impl Display for FlatErr {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{FLAT_SRC_MSG}")
		}
	}

	// Test error with a single source.
	#[derive(Debug)]
	struct SingleSourceErr {}
	impl Error for SingleSourceErr {
		fn source(&self) -> Option<&(dyn Error + 'static)> {
			Some(&FlatErr {})
		}
	}
	impl Display for SingleSourceErr {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{SINGLE_SRC_MSG}")
		}
	}

	// Test error with multiple sources.
	#[derive(Debug)]
	struct MultiSourceErr {}
	impl Error for MultiSourceErr {
		fn source(&self) -> Option<&(dyn Error + 'static)>  {
			Some(&SingleSourceErr {})
		}
	}
	impl Display for MultiSourceErr {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{MULTI_SRC_MSG}")
		}
	}
	
	#[test]
	fn error_messages_are_correct_for_flat_errors() {
		// Assert that an error without a `source` is correctly formatted.
		let flat_err = FlatErr {};
		let result = format_err_msg(&flat_err);
		assert_eq!(result, format!("{FLAT_SRC_MSG}\n"));
	}

	#[test]
	fn error_messages_are_correct_for_errors_with_a_source() {
		// Assert that an error which has a `source` is correctly formatted.
		let single_src_err = SingleSourceErr {};
		let result = format_err_msg(&single_src_err);
		assert_eq!(result, format!("{SINGLE_SRC_MSG}\nCaused by: {FLAT_SRC_MSG}\n"));
	}

	#[test]
	fn error_messages_are_correct_for_multi_source_errors() {
		// Assert that an error which has multiple sources is correctly formatted.
		let multi_src_err = MultiSourceErr {};
		let result = format_err_msg(&multi_src_err);
		assert_eq!(result, format!("{MULTI_SRC_MSG}\nCaused by: {SINGLE_SRC_MSG}\nCaused by: {FLAT_SRC_MSG}\n"));
	}

	#[test]
	fn hints_are_added_correctly() {
		// Assert that errors are correctly combined and formatted with hints.
		let raw_result: Result<(), FlatErr> = Err(FlatErr {});
		let with_hint = raw_result.hint(HINT_MSG);
		let err_msg = format_err_msg(&with_hint.unwrap_err());
		assert_eq!(err_msg, format!("{FLAT_SRC_MSG}\n{HINT_MSG}\n"));
	}

	#[test]
	fn examples_are_added_correctly() {
		// Assert that errors are correctly combinded and formatted with hints AND examples.
		use crate::Example;
		let raw_result: Result<(), FlatErr> = Err(FlatErr {});
		let with_hint = raw_result.hint(HINT_MSG);
		let with_example = with_hint.example(EXAMPLE_MSG);
		let err_msg = format_err_msg(&with_example.unwrap_err());
		assert_eq!(err_msg, format!("{FLAT_SRC_MSG}\n{HINT_MSG}: `{EXAMPLE_MSG}`\n"));
	}
}
