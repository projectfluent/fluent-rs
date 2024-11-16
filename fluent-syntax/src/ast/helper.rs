use super::Comment;
#[cfg(feature = "spans")]
use super::Span;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// This is a helper struct used to properly deserialize referential
// JSON comments which are single continuous String, into a vec of
// content slices.
#[derive(Debug, Clone)]
#[cfg_attr(not(feature = "spans"), derive(PartialEq, Eq))]
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

#[cfg(feature = "spans")]
impl<S: Eq> Eq for CommentDef<S> {}

#[cfg(feature = "spans")]
impl<S: PartialEq> PartialEq for CommentDef<S> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Single {
                    content: l_content, ..
                },
                Self::Single {
                    content: r_content, ..
                },
            ) => l_content == r_content,
            (
                Self::Multi {
                    content: l_content, ..
                },
                Self::Multi {
                    content: r_content, ..
                },
            ) => l_content == r_content,
            _ => false,
        }
    }
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
