/// Add an example message to an object.
pub trait Example<'a> {
	/// Consumes and returns `self` combined with the
	/// given `example` message.
	fn example(self, example: &'a str) -> Self;
}

/// Implementation of `Example` on any `Result`s returned by
/// [`hint`](crate::Giveup::hint)
impl<'a, T, E> Example<'a> for Result<T, HintedError<'a, E>>
where
	E: std::fmt::Display + Send + Sync,
{
	fn example(mut self, example: &'a str) -> Self {
		if let Err(ref mut e) = self {
			e.hint.example = Some(example);
		}
		self
	}
}


pub struct HintedError<'a, E> {
	e: E,
	hint: Hint<'a>,
}

impl<'a, E> HintedError<'a, E>
where
	E: std::fmt::Display + Send + Sync,
{
	pub fn from_hint(e: E, hint: &'a str) -> Self {
		Self {
			e,
			hint: Hint{ hint, example: None },
		}
	}
}

impl<'a, E> std::fmt::Display for HintedError<'a, E>
where
	E: std::fmt::Display + Send + Sync,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}\n{}", self.e, self.hint)
	}
}

struct Hint<'a> {
	hint: &'a str,
	example: Option<&'a str>,
}

impl<'a> std::fmt::Display for Hint<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.example {
			Some(example) => write!(f, "{}: `{}`", self.hint, example),
			None => write!(f, "{}", self.hint),
		}
	}
}
