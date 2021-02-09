use fluent_syntax::ast;
use fluent_syntax::parser::{parse_runtime, ParserError};
use ouroboros::self_referencing;

#[self_referencing]
#[derive(Debug)]
pub struct InnerFluentResource {
    string: String,
    #[borrows(string)]
    #[covariant]
    ast: ast::Resource<&'this str>,
}

/// A resource containing a list of localization messages.
///
/// [`FluentResource`] wraps an [`Abstract Syntax Tree`](../fluent_syntax/ast/index.html) produced by the
/// [`parser`](../fluent_syntax/parser/index.html) and provides an access to a list
/// of its entries.
///
/// A good mental model for a resource is a single FTL file, but in the future
/// there's nothing preventing a resource from being stored in a data base,
/// pre-parsed format or in some other structured form.
///
/// # Example
///
/// ```
/// use fluent_bundle::FluentResource;
///
/// let source = r#"
///
/// hello-world = Hello World!
///
/// "#;
///
/// let resource = FluentResource::try_new(source.to_string())
///     .expect("Errors encountered while parsing a resource.");
///
/// assert_eq!(resource.entries().count(), 1);
/// ```
///
/// # Ownership
///
/// A resource owns the source string and the AST contains references
/// to the slices of the source.
#[derive(Debug)]
pub struct FluentResource(InnerFluentResource);

impl FluentResource {
    pub fn try_new(source: String) -> Result<Self, (Self, Vec<ParserError>)> {
        let mut errors = None;

        let res = InnerFluentResourceBuilder {
            string: source,
            ast_builder: |string: &str| match parse_runtime(string) {
                Ok(ast) => ast,
                Err((ast, err)) => {
                    errors = Some(err);
                    ast
                }
            },
        }
        .build();

        if let Some(errors) = errors {
            Err((Self(res), errors))
        } else {
            Ok(Self(res))
        }
    }

    pub fn ast(&self) -> &ast::Resource<&str> {
        self.0.borrow_ast()
    }

    pub fn source(&self) -> &str {
        &self.0.borrow_string()
    }

    pub fn entries(&self) -> impl Iterator<Item = &ast::Entry<&str>> {
        self.0.borrow_ast().body.iter()
    }

    pub fn get_entry(&self, idx: usize) -> Option<&ast::Entry<&str>> {
        self.0.borrow_ast().body.get(idx)
    }
}
