use super::Comment;
#[cfg(feature = "spans")]
use super::Span;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// This is a helper struct used to properly deserialize referential
// JSON comments which are single continuous String, into a vec of
// content slices.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CommentDef<S> {
    Single {
        content: S,
        #[cfg(feature = "spans")]
        span: Span,
    },
    Multi {
        content: Vec<S>,
        #[cfg(feature = "spans")]
        span: Span,
    },
}

impl<S> From<CommentDef<S>> for Comment<S> {
    fn from(input: CommentDef<S>) -> Self {
        match input {
            CommentDef::Single {
                content,
                #[cfg(feature = "spans")]
                span,
            } => Self {
                content: vec![content],
                #[cfg(feature = "spans")]
                span,
            },
            CommentDef::Multi {
                content,
                #[cfg(feature = "spans")]
                span,
            } => Self {
                content,
                #[cfg(feature = "spans")]
                span,
            },
        }
    }
}
