use fluent_syntax::ast;
use fluent_syntax::parser::ParserError;
use self_cell::self_cell;
use std::convert::TryInto;

type Resource<'s> = ast::Resource<&'s str>;
self_cell!(
    pub struct FluentResource {
        #[try_from]
        owner: String,
        #[covariant]
        dependent: Resource,
    }

    impl {Debug}
);

impl FluentResource {
    /// A fallible constructor of a new [`FluentResource`].
    ///
    /// It takes an encoded `Fluent Translation List` string, parses
    /// it and stores both, the input string and the AST view of it,
    /// for runtime use.
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string());
    ///
    /// assert!(resource.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// The method will return the resource irrelevant of parse errors
    /// encountered during parsing of the source, but in case of errors,
    /// the `Err` variant will contain both the structure and a vector
    /// of errors.
    pub fn try_new(source: String) -> Result<Self, (Self, Vec<ParserError>)> {
        Ok(Self::try_from(source).unwrap())
    }

    /// Returns a reference to the source string that was used
    /// to construct the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    ///
    /// let source = "hello-world = Hello, { $user }!";
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert_eq!(
    ///     resource.source(),
    ///     "hello-world = Hello, { $user }!"
    /// );
    /// ```
    pub fn source(&self) -> &str {
        self.borrow_owner().as_str()
    }

    /// Returns an iterator over [`entries`](fluent_syntax::ast::Entry) of the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    /// use fluent_syntax::ast;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert_eq!(
    ///     resource.entries().count(),
    ///     1
    /// );
    /// assert!(matches!(resource.entries().next(), Some(ast::Entry::Message(_))));
    /// ```
    pub fn entries(&self) -> impl Iterator<Item = &ast::Entry<&str>> {
        self.borrow_dependent().body.iter()
    }

    /// Returns an [`Entry`](fluent_syntax::ast::Entry) at the
    /// given index out of the [`FluentResource`].
    ///
    /// # Example
    ///
    /// ```
    /// use fluent_bundle::FluentResource;
    /// use fluent_syntax::ast;
    ///
    /// let source = r#"
    ///
    /// hello-world = Hello, { $user }!
    ///
    /// "#;
    ///
    /// let resource = FluentResource::try_new(source.to_string())
    ///     .expect("Failed to parse FTL.");
    ///
    /// assert!(matches!(resource.get_entry(0), Some(ast::Entry::Message(_))));
    /// ```
    pub fn get_entry(&self, idx: usize) -> Option<&ast::Entry<&str>> {
        self.borrow_dependent().body.get(idx)
    }
}
