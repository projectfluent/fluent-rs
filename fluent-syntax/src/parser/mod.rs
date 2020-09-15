use crate::ast;
use std::ops::Range;
use std::result;

#[macro_use]
mod errors;
pub use errors::{ErrorKind, ParserError};

pub trait Slice<'s>: AsRef<str> {
    fn slice(&self, range: Range<usize>) -> Self;
    fn trim(&mut self);
}

impl<'s> Slice<'s> for String {
    fn slice(&self, range: Range<usize>) -> String {
        self[range].to_string()
    }

    fn trim(&mut self) {
        *self = self.trim_end().to_string();
    }
}

impl<'s> Slice<'s> for &'s str {
    fn slice(&self, range: Range<usize>) -> &'s str {
        &self[range]
    }

    fn trim(&mut self) {
        *self = self.trim_end();
    }
}

pub type Result<T> = result::Result<T, ParserError>;

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

// This enum allows us to mark pointers in the source which will later become text elements
// but without slicing them out of the source string. This makes the indentation adjustments
// cheaper since they'll happen on the pointers, rather than extracted slices.
#[derive(Debug)]
enum PatternElementPlaceholders<S> {
    Placeable(ast::Expression<S>),
    // (start, end, indent, position)
    TextElement(usize, usize, usize, TextElementPosition),
}

// This enum tracks whether the text element is blank or not.
// This is important to identify text elements which should not be taken into account
// when calculating common indent.
#[derive(Debug, PartialEq)]
enum TextElementType {
    Blank,
    NonBlank,
}

pub struct Parser<S> {
    source: S,
    ptr: usize,
    length: usize,
}

impl<'s, S> Parser<S>
where
    S: Slice<'s> + Clone + PartialEq,
{
    pub fn new(source: S) -> Self {
        let length = source.as_ref().len();
        Self {
            source,
            ptr: 0,
            length,
        }
    }

    pub fn parse(
        &mut self,
    ) -> std::result::Result<ast::Resource<S>, (ast::Resource<S>, Vec<ParserError>)> {
        let mut errors = vec![];

        let mut body = vec![];

        self.skip_blank_block();
        let mut last_comment = None;
        let mut last_blank_count = 0;

        while self.ptr < self.length {
            let entry_start = self.ptr;
            let mut entry = self.get_entry(entry_start);

            if let Some(comment) = last_comment.take() {
                match entry {
                    Ok(ast::Entry::Message(ref mut msg)) if last_blank_count < 2 => {
                        msg.comment = Some(comment);
                    }
                    Ok(ast::Entry::Term(ref mut term)) if last_blank_count < 2 => {
                        term.comment = Some(comment);
                    }
                    _ => {
                        body.push(ast::Entry::Comment(comment));
                    }
                }
            }

            match entry {
                Ok(ast::Entry::Comment(comment)) => {
                    last_comment = Some(comment);
                }
                Ok(entry) => {
                    body.push(entry);
                }
                Err(mut err) => {
                    self.skip_to_next_entry_start();
                    err.slice = Some((entry_start, self.ptr));
                    errors.push(err);
                    let content = self.source.slice(entry_start..self.ptr);
                    body.push(ast::Entry::Junk { content });
                }
            }
            last_blank_count = self.skip_blank_block();
        }

        if let Some(last_comment) = last_comment.take() {
            body.push(ast::Entry::Comment(last_comment));
        }
        if errors.is_empty() {
            Ok(ast::Resource { body })
        } else {
            Err((ast::Resource { body }, errors))
        }
    }

    fn get_entry(&mut self, entry_start: usize) -> Result<ast::Entry<S>> {
        let entry = match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b'#') => {
                let (comment, level) = self.get_comment()?;
                match level {
                    1 => ast::Entry::Comment(comment),
                    2 => ast::Entry::GroupComment(comment),
                    3 => ast::Entry::ResourceComment(comment),
                    _ => unreachable!(),
                }
            }
            Some(b'-') => ast::Entry::Term(self.get_term(entry_start)?),
            _ => ast::Entry::Message(self.get_message(entry_start)?),
        };
        Ok(entry)
    }

    fn get_message(&mut self, entry_start: usize) -> Result<ast::Message<S>> {
        let id = self.get_identifier()?;
        self.skip_blank_inline();
        self.expect_byte(b'=')?;
        let pattern = self.get_pattern()?;

        self.skip_blank_block();

        let attributes = self.get_attributes();

        if pattern.is_none() && attributes.is_empty() {
            let entry_id = id.name.as_ref().to_owned();
            return error!(
                ErrorKind::ExpectedMessageField { entry_id },
                entry_start, self.ptr
            );
        }

        Ok(ast::Message {
            id,
            value: pattern,
            attributes,
            comment: None,
        })
    }

    fn get_term(&mut self, entry_start: usize) -> Result<ast::Term<S>> {
        self.expect_byte(b'-')?;
        let id = self.get_identifier()?;
        self.skip_blank_inline();
        self.expect_byte(b'=')?;
        self.skip_blank_inline();

        let value = self.get_pattern()?;

        self.skip_blank_block();

        let attributes = self.get_attributes();

        if let Some(value) = value {
            Ok(ast::Term {
                id,
                value,
                attributes,
                comment: None,
            })
        } else {
            error!(
                ErrorKind::ExpectedTermField {
                    entry_id: id.name.as_ref().to_owned()
                },
                entry_start, self.ptr
            )
        }
    }

    fn get_attributes(&mut self) -> Vec<ast::Attribute<S>> {
        let mut attributes = vec![];

        loop {
            let line_start = self.ptr;
            self.skip_blank_inline();
            if !self.is_current_byte(b'.') {
                self.ptr = line_start;
                break;
            }

            match self.get_attribute() {
                Ok(attr) => attributes.push(attr),
                Err(_) => {
                    self.ptr = line_start;
                    break;
                }
            }
        }
        attributes
    }

    fn get_attribute(&mut self) -> Result<ast::Attribute<S>> {
        self.expect_byte(b'.')?;
        let id = self.get_identifier()?;
        self.skip_blank_inline();
        self.expect_byte(b'=')?;
        let pattern = self.get_pattern()?;

        match pattern {
            Some(pattern) => Ok(ast::Attribute { id, value: pattern }),
            None => error!(ErrorKind::MissingValue, self.ptr),
        }
    }

    fn get_identifier(&mut self) -> Result<ast::Identifier<S>> {
        let mut ptr = self.ptr;

        match self.source.as_ref().as_bytes().get(ptr) {
            Some(b) if b.is_ascii_alphabetic() => {
                ptr += 1;
            }
            _ => {
                return error!(
                    ErrorKind::ExpectedCharRange {
                        range: "a-zA-Z".to_string()
                    },
                    ptr
                );
            }
        }

        while let Some(b) = self.source.as_ref().as_bytes().get(ptr) {
            if b.is_ascii_alphabetic() || b.is_ascii_digit() || [b'_', b'-'].contains(b) {
                ptr += 1;
            } else {
                break;
            }
        }

        let name = self.source.slice(self.ptr..ptr);
        self.ptr = ptr;

        Ok(ast::Identifier { name })
    }

    fn get_attribute_accessor(&mut self) -> Result<Option<ast::Identifier<S>>> {
        if !self.take_byte_if(b'.') {
            Ok(None)
        } else {
            let ident = self.get_identifier()?;
            Ok(Some(ident))
        }
    }

    fn get_variant_key(&mut self) -> Result<ast::VariantKey<S>> {
        if !self.take_byte_if(b'[') {
            return error!(ErrorKind::ExpectedToken('['), self.ptr);
        }
        self.skip_blank();

        let key = if self.is_number_start() {
            ast::VariantKey::NumberLiteral {
                value: self.get_number_literal()?,
            }
        } else {
            ast::VariantKey::Identifier {
                name: self.get_identifier()?.name,
            }
        };

        self.skip_blank();

        self.expect_byte(b']')?;

        Ok(key)
    }

    fn get_variants(&mut self) -> Result<Vec<ast::Variant<S>>> {
        let mut variants = vec![];
        let mut has_default = false;

        while self.is_current_byte(b'*') || self.is_current_byte(b'[') {
            let default = self.take_byte_if(b'*');

            if default {
                if has_default {
                    return error!(ErrorKind::MultipleDefaultVariants, self.ptr);
                } else {
                    has_default = true;
                }
            }

            let key = self.get_variant_key()?;

            let value = self.get_pattern()?;

            if let Some(value) = value {
                variants.push(ast::Variant {
                    key,
                    value,
                    default,
                });
                self.skip_blank();
            } else {
                return error!(ErrorKind::MissingValue, self.ptr);
            }
        }

        if !has_default {
            error!(ErrorKind::MissingDefaultVariant, self.ptr)
        } else {
            Ok(variants)
        }
    }

    fn get_pattern(&mut self) -> Result<Option<ast::Pattern<S>>> {
        let mut elements = vec![];
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
            if self.is_current_byte(b'{') {
                if text_element_role == TextElementPosition::LineStart {
                    common_indent = Some(0);
                }
                let exp = self.get_placeable()?;
                last_non_blank = Some(elements.len());
                elements.push(PatternElementPlaceholders::Placeable(exp));
                text_element_role = TextElementPosition::Continuation;
            } else {
                let slice_start = self.ptr;
                let mut indent = 0;
                if text_element_role == TextElementPosition::LineStart {
                    indent = self.skip_blank_inline();
                    if self.ptr >= self.length {
                        break;
                    }
                    let b = self.source.as_ref().as_bytes().get(self.ptr);
                    if indent == 0 {
                        if b != Some(&b'\n') {
                            break;
                        }
                    } else if !self.is_byte_pattern_continuation(*b.unwrap()) {
                        self.ptr = slice_start;
                        break;
                    }
                }
                let (start, end, text_element_type, termination_reason) = self.get_text_slice()?;
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
                        elements.push(PatternElementPlaceholders::TextElement(
                            slice_start,
                            end,
                            indent,
                            text_element_role,
                        ));
                    }
                }

                text_element_role = match termination_reason {
                    TextElementTermination::LineFeed => TextElementPosition::LineStart,
                    TextElementTermination::CRLF => TextElementPosition::Continuation,
                    TextElementTermination::PlaceableStart => TextElementPosition::Continuation,
                    TextElementTermination::EOF => TextElementPosition::Continuation,
                };
            }
        }

        if let Some(last_non_blank) = last_non_blank {
            let elements = elements
                .into_iter()
                .take(last_non_blank + 1)
                .enumerate()
                .map(|(i, elem)| match elem {
                    PatternElementPlaceholders::Placeable(expression) => {
                        ast::PatternElement::Placeable { expression }
                    }
                    PatternElementPlaceholders::TextElement(start, end, indent, role) => {
                        let start = if role == TextElementPosition::LineStart {
                            if let Some(common_indent) = common_indent {
                                start + std::cmp::min(indent, common_indent)
                            } else {
                                start + indent
                            }
                        } else {
                            start
                        };
                        let mut value = self.source.slice(start..end);
                        if last_non_blank == i {
                            value.trim();
                            ast::PatternElement::TextElement { value }
                        } else {
                            ast::PatternElement::TextElement { value }
                        }
                    }
                })
                .collect();
            return Ok(Some(ast::Pattern { elements }));
        }

        Ok(None)
    }

    fn get_text_slice(
        &mut self,
    ) -> Result<(usize, usize, TextElementType, TextElementTermination)> {
        let start_pos = self.ptr;
        let mut text_element_type = TextElementType::Blank;

        while let Some(b) = self.source.as_ref().as_bytes().get(self.ptr) {
            match b {
                b' ' => self.ptr += 1,
                b'\n' => {
                    self.ptr += 1;
                    return Ok((
                        start_pos,
                        self.ptr,
                        text_element_type,
                        TextElementTermination::LineFeed,
                    ));
                }
                b'\r' if self.is_byte_at(b'\n', self.ptr + 1) => {
                    self.ptr += 1;
                    return Ok((
                        start_pos,
                        self.ptr - 1,
                        text_element_type,
                        TextElementTermination::CRLF,
                    ));
                }
                b'{' => {
                    return Ok((
                        start_pos,
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
        Ok((
            start_pos,
            self.ptr,
            text_element_type,
            TextElementTermination::EOF,
        ))
    }

    fn get_comment(&mut self) -> Result<(ast::Comment<S>, usize)> {
        let mut level = None;
        let mut content = vec![];

        while self.ptr < self.length {
            let line_level = self.get_comment_level();
            if line_level == 0 {
                self.ptr -= 1;
                break;
            }
            if level.is_some() && Some(line_level) != level {
                self.ptr -= line_level;
                break;
            }

            level = Some(line_level);

            if self.ptr == self.length {
                break;
            } else if self.is_current_byte(b'\n') {
                content.push(self.get_comment_line()?);
            } else {
                if let Err(e) = self.expect_byte(b' ') {
                    if content.is_empty() {
                        return Err(e);
                    } else {
                        self.ptr -= line_level;
                        break;
                    }
                }
                content.push(self.get_comment_line()?);
            }
            self.skip_eol();
        }

        Ok((ast::Comment { content }, level.unwrap()))
    }

    fn get_comment_level(&mut self) -> usize {
        let mut chars = 0;

        while self.take_byte_if(b'#') {
            chars += 1;
        }

        chars
    }

    fn get_comment_line(&mut self) -> Result<S> {
        let start_pos = self.ptr;

        while self.ptr < self.length && !self.is_eol() {
            self.ptr += 1;
        }

        Ok(self.source.slice(start_pos..self.ptr))
    }

    fn get_placeable(&mut self) -> Result<ast::Expression<S>> {
        self.expect_byte(b'{')?;
        self.skip_blank();
        let exp = self.get_expression()?;
        self.skip_blank_inline();
        self.expect_byte(b'}')?;

        let invalid_expression_found = match &exp {
            ast::Expression::InlineExpression(ast::InlineExpression::TermReference {
                ref attribute,
                ..
            }) => attribute.is_some(),
            _ => false,
        };
        if invalid_expression_found {
            return error!(ErrorKind::TermAttributeAsPlaceable, self.ptr);
        }

        Ok(exp)
    }

    fn get_expression(&mut self) -> Result<ast::Expression<S>> {
        let exp = self.get_inline_expression()?;

        self.skip_blank();

        if !self.is_current_byte(b'-') || !self.is_byte_at(b'>', self.ptr + 1) {
            if let ast::InlineExpression::TermReference { ref attribute, .. } = exp {
                if attribute.is_some() {
                    return error!(ErrorKind::TermAttributeAsPlaceable, self.ptr);
                }
            }
            return Ok(ast::Expression::InlineExpression(exp));
        }

        match exp {
            ast::InlineExpression::MessageReference { ref attribute, .. } => {
                if attribute.is_none() {
                    return error!(ErrorKind::MessageReferenceAsSelector, self.ptr);
                } else {
                    return error!(ErrorKind::MessageAttributeAsSelector, self.ptr);
                }
            }
            ast::InlineExpression::TermReference { ref attribute, .. } => {
                if attribute.is_none() {
                    return error!(ErrorKind::TermReferenceAsSelector, self.ptr);
                }
            }
            ast::InlineExpression::StringLiteral { .. }
            | ast::InlineExpression::NumberLiteral { .. }
            | ast::InlineExpression::VariableReference { .. }
            | ast::InlineExpression::FunctionReference { .. } => {}
            _ => {
                return error!(ErrorKind::ExpectedSimpleExpressionAsSelector, self.ptr);
            }
        };

        self.ptr += 2; // ->

        self.skip_blank_inline();
        if !self.skip_eol() {
            return error!(
                ErrorKind::ExpectedCharRange {
                    range: "\n | \r\n".to_string()
                },
                self.ptr
            );
        }
        self.skip_blank();

        let variants = self.get_variants()?;

        Ok(ast::Expression::SelectExpression {
            selector: exp,
            variants,
        })
    }

    fn get_inline_expression(&mut self) -> Result<ast::InlineExpression<S>> {
        match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b'"') => {
                self.ptr += 1; // "
                let start = self.ptr;
                while let Some(b) = self.source.as_ref().as_bytes().get(self.ptr) {
                    match b {
                        b'\\' => match self.source.as_ref().as_bytes().get(self.ptr + 1) {
                            Some(b'\\') => self.ptr += 2,
                            Some(b'{') => self.ptr += 2,
                            Some(b'"') => self.ptr += 2,
                            Some(b'u') => {
                                self.ptr += 2;
                                self.skip_unicode_escape_sequence(4)?;
                            }
                            Some(b'U') => {
                                self.ptr += 2;
                                self.skip_unicode_escape_sequence(6)?;
                            }
                            _ => return error!(ErrorKind::Generic, self.ptr),
                        },
                        b'"' => {
                            break;
                        }
                        b'\n' => {
                            return error!(ErrorKind::Generic, self.ptr);
                        }
                        _ => self.ptr += 1,
                    }
                }

                self.expect_byte(b'"')?;
                let slice = self.source.slice(start..self.ptr - 1);
                Ok(ast::InlineExpression::StringLiteral { value: slice })
            }
            Some(b) if b.is_ascii_digit() => {
                let num = self.get_number_literal()?;
                Ok(ast::InlineExpression::NumberLiteral { value: num })
            }
            Some(b'-') => {
                self.ptr += 1; // -
                if self.is_identifier_start() {
                    let id = self.get_identifier()?;
                    let attribute = self.get_attribute_accessor()?;
                    let arguments = self.get_call_arguments()?;
                    Ok(ast::InlineExpression::TermReference {
                        id,
                        attribute,
                        arguments,
                    })
                } else {
                    self.ptr -= 1;
                    let num = self.get_number_literal()?;
                    Ok(ast::InlineExpression::NumberLiteral { value: num })
                }
            }
            Some(b'$') => {
                self.ptr += 1; // -
                let id = self.get_identifier()?;
                Ok(ast::InlineExpression::VariableReference { id })
            }
            Some(b) if b.is_ascii_alphabetic() => {
                let id = self.get_identifier()?;
                let arguments = self.get_call_arguments()?;
                if arguments.is_some() {
                    if !id.name.as_ref().bytes().all(|c| {
                        c.is_ascii_uppercase() || c.is_ascii_digit() || c == b'_' || c == b'-'
                    }) {
                        return error!(ErrorKind::ForbiddenCallee, self.ptr);
                    }

                    Ok(ast::InlineExpression::FunctionReference { id, arguments })
                } else {
                    let attribute = self.get_attribute_accessor()?;
                    Ok(ast::InlineExpression::MessageReference { id, attribute })
                }
            }
            Some(b'{') => {
                let exp = self.get_placeable()?;
                Ok(ast::InlineExpression::Placeable {
                    expression: Box::new(exp),
                })
            }
            _ => error!(ErrorKind::ExpectedInlineExpression, self.ptr),
        }
    }

    fn get_call_arguments(&mut self) -> Result<Option<ast::CallArguments<S>>> {
        self.skip_blank();
        if !self.take_byte_if(b'(') {
            return Ok(None);
        }

        let mut positional = vec![];
        let mut named = vec![];
        let mut argument_names = vec![];

        self.skip_blank();

        while self.ptr < self.length {
            if self.is_current_byte(b')') {
                break;
            }

            let expr = self.get_inline_expression()?;

            match expr {
                ast::InlineExpression::MessageReference {
                    ref id,
                    attribute: None,
                } => {
                    self.skip_blank();
                    if self.is_current_byte(b':') {
                        if argument_names.contains(&id.name) {
                            return error!(
                                ErrorKind::DuplicatedNamedArgument(id.name.as_ref().to_owned()),
                                self.ptr
                            );
                        }
                        self.ptr += 1;
                        self.skip_blank();
                        let val = self.get_inline_expression()?;

                        argument_names.push(id.name.clone());
                        named.push(ast::NamedArgument {
                            name: ast::Identifier {
                                name: id.name.clone(),
                            },
                            value: val,
                        });
                    } else {
                        if !argument_names.is_empty() {
                            return error!(ErrorKind::PositionalArgumentFollowsNamed, self.ptr);
                        }
                        positional.push(expr);
                    }
                }
                _ => {
                    if !argument_names.is_empty() {
                        return error!(ErrorKind::PositionalArgumentFollowsNamed, self.ptr);
                    }
                    positional.push(expr);
                }
            }

            self.skip_blank();
            self.take_byte_if(b',');
            self.skip_blank();
        }

        self.expect_byte(b')')?;

        Ok(Some(ast::CallArguments { positional, named }))
    }

    fn get_number_literal(&mut self) -> Result<S> {
        let start = self.ptr;
        self.take_byte_if(b'-');
        self.skip_digits()?;
        if self.take_byte_if(b'.') {
            self.skip_digits()?;
        }

        Ok(self.source.slice(start..self.ptr))
    }

    // *************************

    pub fn is_current_byte(&self, b: u8) -> bool {
        self.source.as_ref().as_bytes().get(self.ptr) == Some(&b)
    }

    fn is_byte_at(&self, b: u8, pos: usize) -> bool {
        self.source.as_ref().as_bytes().get(pos) == Some(&b)
    }

    fn skip_to_next_entry_start(&mut self) {
        while let Some(b) = self.source.as_ref().as_bytes().get(self.ptr) {
            let new_line =
                self.ptr == 0 || self.source.as_ref().as_bytes().get(self.ptr - 1) == Some(&b'\n');

            if new_line && (b.is_ascii_alphabetic() || [b'-', b'#'].contains(b)) {
                break;
            }

            self.ptr += 1;
        }
    }

    fn skip_eol(&mut self) -> bool {
        match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b'\n') => {
                self.ptr += 1;
                true
            }
            Some(b'\r') if self.is_byte_at(b'\n', self.ptr + 1) => {
                self.ptr += 2;
                true
            }
            _ => false,
        }
    }

    fn skip_unicode_escape_sequence(&mut self, length: usize) -> Result<()> {
        let start = self.ptr;
        for _ in 0..length {
            match self.source.as_ref().as_bytes().get(self.ptr) {
                Some(b) if b.is_ascii_hexdigit() => self.ptr += 1,
                _ => break,
            }
        }
        if self.ptr - start != length {
            let end = if self.ptr >= self.length {
                self.ptr
            } else {
                self.ptr + 1
            };
            let seq = self.source.slice(start..end).as_ref().to_owned();
            return error!(ErrorKind::InvalidUnicodeEscapeSequence(seq), self.ptr);
        }
        Ok(())
    }

    fn is_identifier_start(&self) -> bool {
        match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b) if b.is_ascii_alphabetic() => true,
            _ => false,
        }
    }

    fn take_byte_if(&mut self, b: u8) -> bool {
        if self.is_current_byte(b) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }

    fn skip_blank_block(&mut self) -> usize {
        let mut count = 0;
        loop {
            let start = self.ptr;
            self.skip_blank_inline();
            if !self.skip_eol() {
                self.ptr = start;
                break;
            }
            count += 1;
        }
        count
    }

    fn skip_blank(&mut self) {
        loop {
            match self.source.as_ref().as_bytes().get(self.ptr) {
                Some(b' ') => self.ptr += 1,
                Some(b'\n') => self.ptr += 1,
                Some(b'\r')
                    if self.source.as_ref().as_bytes().get(self.ptr + 1) == Some(&b'\n') =>
                {
                    self.ptr += 2
                }
                _ => break,
            }
        }
    }

    fn skip_blank_inline(&mut self) -> usize {
        let start = self.ptr;
        while let Some(b' ') = self.source.as_ref().as_bytes().get(self.ptr) {
            self.ptr += 1;
        }
        self.ptr - start
    }

    fn is_byte_pattern_continuation(&self, b: u8) -> bool {
        ![b'}', b'.', b'[', b'*'].contains(&b)
    }

    fn expect_byte(&mut self, b: u8) -> Result<()> {
        if !self.is_current_byte(b) {
            return error!(ErrorKind::ExpectedToken(b as char), self.ptr);
        }
        self.ptr += 1;
        Ok(())
    }

    fn is_number_start(&self) -> bool {
        match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b) if (b == &b'-') || b.is_ascii_digit() => true,
            _ => false,
        }
    }

    fn is_eol(&self) -> bool {
        match self.source.as_ref().as_bytes().get(self.ptr) {
            Some(b'\n') => true,
            Some(b'\r') if self.is_byte_at(b'\n', self.ptr + 1) => true,
            _ => false,
        }
    }

    fn skip_digits(&mut self) -> Result<()> {
        let start = self.ptr;
        loop {
            match self.source.as_ref().as_bytes().get(self.ptr) {
                Some(b) if b.is_ascii_digit() => self.ptr += 1,
                _ => break,
            }
        }
        if start == self.ptr {
            error!(
                ErrorKind::ExpectedCharRange {
                    range: "0-9".to_string()
                },
                self.ptr
            )
        } else {
            Ok(())
        }
    }
}
