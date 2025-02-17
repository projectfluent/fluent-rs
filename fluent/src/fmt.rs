//! ```
//! use fluent::{
//!     fluent_args,
//!     fmt::{self, FluentDisplay as _},
//!     FluentBundle, FluentResource,
//! };
//!
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! impl fmt::FluentDisplay for Person {
//!     fn fmt(&self, f: &mut dyn fmt::FluentFormatter) -> fmt::Result<()> {
//!         f.write_message(
//!             "person",
//!             fluent_args!("name" => &self.name, "age" => self.age),
//!         )
//!     }
//! }
//!
//! let ftl_string = String::from("person = Hello to { $name } who is { $age }.");
//! let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");
//!
//! let langid_en = "en-US".parse().expect("Parsing failed");
//! let mut bundle = FluentBundle::new(vec![langid_en]);
//!
//! bundle
//!     .add_resource(res)
//!     .expect("Failed to add FTL resources to the bundle.");
//!
//! let mut output = String::new();
//! let mut f = fmt::Formatter::new(&bundle, &mut output);
//!
//! let person = Person {
//!     name: "Vivian".into(),
//!     age: 6,
//! };
//!
//! person.fmt(&mut f).expect("Formatting failed");
//!
//! assert_eq!(
//!     output,
//!     "Hello to \u{2068}Vivian\u{2069} who is \u{2068}6\u{2069}."
//! );
//! ```

use std::{borrow::Borrow, error::Error, fmt};

use crate::{FluentArgs, FluentBundle, FluentError, FluentResource};

/// The type returned by formatter methods.
pub type Result<T> = std::result::Result<T, FormatError>;

/// Formats a localized value to a [`FluentFormatter`].
pub trait FluentDisplay {
    /// Formats the value to a formatter.
    fn fmt(&self, f: &mut dyn FluentFormatter) -> Result<()>;
}

/// A destination for a localized value.
pub trait FluentFormatter {
    /// Writes the localized message with the given id and arguments.
    // TODO[DISCUSS]: Should we take an `Option` for `args`, like other functions?
    fn write_message(&mut self, id: &str, args: FluentArgs) -> Result<()>;
}

#[derive(Debug)]
enum InternalFormatError {
    MessageDoesNotExist { id: String },

    MessageHasNoValue { id: String },

    Formatting { source: fmt::Error },

    Fluent { errors: Vec<FluentError> },
}

/// The error type which is returned from formatting a message.
#[derive(Debug)]
pub struct FormatError(InternalFormatError);

impl FormatError {
    /// When the error is caused by one or more underlying
    /// [`FluentError`]s, this consumes the error and returns the
    /// [`FluentError`]s.
    ///
    /// Note that some or all of the value may have been formatted to
    /// the output if [`FluentError`]s are reported.
    pub fn fluent_errors(self) -> Result<Vec<FluentError>> {
        use InternalFormatError::*;

        match self.0 {
            Fluent { errors } => Ok(errors),
            MessageDoesNotExist { .. } | MessageHasNoValue { .. } | Formatting { .. } => Err(self),
        }
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InternalFormatError::*;

        match &self.0 {
            MessageDoesNotExist { id } => write!(f, "Fluent message `{id}` does not exist"),
            MessageHasNoValue { id } => write!(f, "Fluent message `{id}` has no value"),
            Formatting { .. } => write!(f, "Could not format the Fluent message"),
            Fluent { .. } => write!(f, "Internal Fluent errors occurred"),
        }
    }
}

impl Error for FormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use InternalFormatError::*;

        match &self.0 {
            Formatting { source } => Some(source),
            MessageDoesNotExist { .. } | MessageHasNoValue { .. } | Fluent { .. } => None,
        }
    }
}

impl From<InternalFormatError> for FormatError {
    fn from(value: InternalFormatError) -> Self {
        Self(value)
    }
}

/// Wraps a [`FluentBundle`] with a value implementing [`fmt::Write`]
/// as a target for [`FluentDisplay`].
pub struct Formatter<'a, R, W> {
    bundle: &'a FluentBundle<R>,
    output: W,
}

impl<'a, R, W> Formatter<'a, R, W>
where
    R: Borrow<FluentResource>,
    W: fmt::Write,
{
    /// Constructs the [`Formatter`].
    pub fn new(bundle: &'a FluentBundle<R>, output: W) -> Self {
        Self { bundle, output }
    }

    /// Returns the wrapped output.
    pub fn into_inner(self) -> W {
        self.output
    }
}

impl<R, W> FluentFormatter for Formatter<'_, R, W>
where
    R: Borrow<FluentResource>,
    W: fmt::Write,
{
    fn write_message(&mut self, id: &str, args: FluentArgs) -> Result<()> {
        let msg = self
            .bundle
            .get_message(id)
            .ok_or_else(|| InternalFormatError::MessageDoesNotExist { id: id.to_string() })?;

        let pattern = msg
            .value()
            .ok_or_else(|| InternalFormatError::MessageHasNoValue { id: id.to_string() })?;

        let args = Some(&args);

        let mut errors = vec![];

        self.bundle
            .write_pattern(&mut self.output, pattern, args, &mut errors)
            .map_err(|source| InternalFormatError::Formatting { source })?;

        if !errors.is_empty() {
            Err(InternalFormatError::Fluent { errors })?;
        }

        Ok(())
    }
}
