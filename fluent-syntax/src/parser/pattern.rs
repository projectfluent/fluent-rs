use super::errors::{ErrorKind, ParserError};
use super::{core::Parser, core::Result, slice::Slice};
use crate::ast;

#[derive(Debug, PartialEq)]
enum TextElementTermination {
    LineFeed,
    CRLF,
    PlaceableStart,
    EOF,
}

// This enum tracks the placement of the text element in the pattern, which is needed for
// dedentation logic.
#[derive(Debug, PartialEq)]
enum TextElementPosition {
    InitialLineStart,
    LineStart,
    Continuation,
}

// This enum tracks whether the text element is blank or not.
// This is important to identify text elements which should not be taken into account
// when calculating common indent.
#[derive(Debug, PartialEq)]
enum TextElementType {
    Blank,
    NonBlank,
}

impl<'s, S> Parser<S>
where
    S: Slice<'s>,
{
    pub(super) fn get_pattern(&mut self) -> Result<Option<ast::Pattern<S>>> {
        let mut elements: Vec<ast::PatternElement<S>> = vec![];
        let mut text_positions = vec![];
        let mut last_non_blank = None;
        let mut common_indent = None;

        self.skip_blank_inline();

        let mut text_element_role = if self.skip_eol() {
            self.skip_blank_block();
            TextElementPosition::LineStart
        } else {
            TextElementPosition::InitialLineStart
        };

        while self.ptr < self.length {
            if self.take_byte_if(b'{') {
                if text_element_role == TextElementPosition::LineStart {
                    common_indent = Some(0);
                }
                let exp = self.get_placeable()?;
                last_non_blank = Some(elements.len());
                elements.push(ast::PatternElement::Placeable { expression: exp });
                text_element_role = TextElementPosition::Continuation;
            } else {
                let slice_start = self.ptr;
                let mut indent = 0;
                if text_element_role == TextElementPosition::LineStart {
                    indent = self.skip_blank_inline();
                    if let Some(b) = get_current_byte!(self) {
                        if indent == 0 {
                            if b != &b'\r' && b != &b'\n' {
                                break;
                            }
                        } else if !Self::is_byte_pattern_continuation(*b) {
                            self.ptr = slice_start;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let start = self.ptr;
                let (end, text_element_type, termination_reason) = self.get_text_slice()?;
                if start != end {
                    if text_element_role == TextElementPosition::LineStart
                        && text_element_type == TextElementType::NonBlank
                    {
                        if let Some(common) = common_indent {
                            if indent < common {
                                common_indent = Some(indent);
                            }
                        } else {
                            common_indent = Some(indent);
                        }
                    }
                    if text_element_role != TextElementPosition::LineStart
                        || text_element_type == TextElementType::NonBlank
                        || termination_reason == TextElementTermination::LineFeed
                    {
                        if text_element_type == TextElementType::NonBlank {
                            last_non_blank = Some(elements.len());
                        }
                        elements.push(ast::PatternElement::TextElement {
                            value: self.source.slice(start..end),
                        });
                        if text_element_role == TextElementPosition::LineStart {
                            text_positions.push((elements.len(), start, end, indent));
                        }
                    }
                }

                text_element_role = match termination_reason {
                    TextElementTermination::LineFeed => TextElementPosition::LineStart,
                    TextElementTermination::CRLF => TextElementPosition::LineStart,
                    TextElementTermination::PlaceableStart => TextElementPosition::Continuation,
                    TextElementTermination::EOF => TextElementPosition::Continuation,
                };
            }
        }

        if let Some(last_non_blank) = last_non_blank {
            elements.truncate(last_non_blank + 1);

            for (idx, start, end, indent) in text_positions {
                let new_start = common_indent.map_or_else(
                    || start + indent,
                    |common_indent| start + std::cmp::min(indent, common_indent),
                );
                if new_start != start {
                    match elements[idx - 1] {
                        ast::PatternElement::TextElement { ref mut value } => {
                            *value = self.source.slice(new_start..end)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            if let Some(ast::PatternElement::TextElement { ref mut value }) = elements.last_mut() {
                (*value).trim()
            }

            return Ok(Some(ast::Pattern { elements }));
        }

        Ok(None)
    }

    fn get_text_slice(&mut self) -> Result<(usize, TextElementType, TextElementTermination)> {
        let mut text_element_type = TextElementType::Blank;

        while let Some(b) = get_current_byte!(self) {
            match b {
                b' ' => self.ptr += 1,
                b'\n' => {
                    self.ptr += 1;
                    return Ok((
                        self.ptr,
                        text_element_type,
                        TextElementTermination::LineFeed,
                    ));
                }
                b'\r' if self.is_byte_at(b'\n', self.ptr + 1) => {
                    self.ptr += 1;
                    return Ok((
                        self.ptr - 1,
                        text_element_type,
                        TextElementTermination::CRLF,
                    ));
                }
                b'{' => {
                    return Ok((
                        self.ptr,
                        text_element_type,
                        TextElementTermination::PlaceableStart,
                    ));
                }
                b'}' => {
                    return error!(ErrorKind::UnbalancedClosingBrace, self.ptr);
                }
                _ => {
                    text_element_type = TextElementType::NonBlank;
                    self.ptr += 1
                }
            }
        }
        Ok((self.ptr, text_element_type, TextElementTermination::EOF))
    }
}
