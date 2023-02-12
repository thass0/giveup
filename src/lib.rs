//! # Giveup
//!
//! `giveup` is a tiny abstraction for wrapping and nicely displaying
//! `Result`s to end-users.
//!
//! It is meant to be used as a replacement for `expect` or `unwrap_or_else`,
//! if an error occured which terminates the program. 
//!
//! ## Example
//!
//! ```rust should_panic
//! # use std::fs::File;
//! # use giveup::{Giveup, Example};
//! // Here reading the config at the start of the cli app
//! // fails because the user has not yet created a config file.
//! 
//! let config_file = File::open("config-path")
//!     .hint("Create a configuration file")
//!     .example("touch config-filename")
//!     .giveup("Missing configuration file");
//! ```
//!
//! ## Motivation
//! In the above scenario `expect` is misplaced because we do not want
//! the user of the cli to be confronted with a `panic`.
//! 
//! To goal is to display an easily readable error messag and offer
//! as much help as possible, so the user can get back to what
//! they originally intended to do (which never is fixing some issues
//! of the tool one is using).
//! My usual solution would look somewhat like this:
//!  
//! ```rust should_panic
//! # use std::fs::File;
//! let config = File::open("config-path").unwrap_or_else(|err| {
//!     eprintln!("Missing configuration file: {}\n\
//!         Create a new configuration file: `touch config-filename`",
//!         err);
//!     std::process::exit(1);
//! });
//! ```
//!
//! In this case the difference is not world-chaning but using
//! `unwrap_or_else` can get pretty verbose with lot's of
//! boilerplate repeating over and over again.
//! Also `giveup` is more friendly to dynamic error messages
//! using variables.
//!
//! ## Feedback
//!
//! I primarily wrote `giveup` for my personal use so I would love
//! to get your [feedback](https://github.com/d4ckard/giveup/issues).
//!
//!

mod giveup;
mod hint;
pub use crate::giveup::Giveup;
pub use crate::hint::Example;