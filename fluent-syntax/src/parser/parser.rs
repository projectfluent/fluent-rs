use super::ast;
use super::lexer::Token;
use super::lexer::{Lexer, LexerError};
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Unknown,
}

impl From<LexerError> for ParserError {
    fn from(_input: LexerError) -> Self {
        Self::Unknown
    }
}

type ParserResult<T> = Result<T, ParserError>;

pub struct Parser<'p> {
    source: &'p str,
    lexer: Lexer<'p>,
    body: Vec<ast::ResourceEntry<'p>>,
    last_comment: Option<ast::Comment<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            lexer: Lexer::new(source.as_bytes()),
            source,
            body: vec![],
            last_comment: None,
        }
    }

    pub fn parse(mut self) -> ast::Resource<'p> {
        loop {
            match self.add_entry() {
                Ok(true) => {}
                Ok(false) => break,
                Err(_err) => {
                    let junk = self.lexer.get_junk();
                    if let Some(comment) = self.last_comment.take() {
                        self.body
                            .push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                    }
                    self.body.push(ast::ResourceEntry::Junk(&self.source[junk]));
                }
            }
        }

        if let Some(comment) = self.last_comment.take() {
            self.body
                .push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
        }

        ast::Resource {
            body: self.body.into_boxed_slice(),
        }
    }

    fn add_entry(&mut self) -> Result<bool, ParserError> {
        match self.lexer.try_next()? {
            Some(Token::Identifier(r)) => {
                self.add_message(r)?;
                Ok(true)
            }
            Some(c @ Token::CommentSign)
            | Some(c @ Token::GroupCommentSign)
            | Some(c @ Token::ResourceCommentSign) => {
                if let Some(comment) = self.last_comment.take() {
                    self.body
                        .push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                }
                self.add_comment(&c)?;
                Ok(true)
            }
            Some(Token::Eol(_)) => {
                if let Some(comment) = self.last_comment.take() {
                    self.body
                        .push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                }
                Ok(true)
            }
            Some(Token::MinusSign) => {
                self.add_term()?;
                Ok(true)
            }
            Some(t) => panic!("Token: {:#?}", t),
            None => Ok(false),
        }
    }

    fn add_message(&mut self, id: Range<usize>) -> ParserResult<()> {
        let id = ast::Identifier {
            name: &self.source[id],
        };

        self.lexer.expect(Token::EqSign)?;

        let pattern = self.maybe_get_pattern()?;

        let mut attributes = vec![];
        while self.lexer.take_if(Token::Dot)? {
            attributes.push(self.get_attribute()?);
        }

        if pattern.is_none() && attributes.is_empty() {
            return Err(ParserError::Unknown);
        }

        self.body
            .push(ast::ResourceEntry::Entry(ast::Entry::Message(
                ast::Message {
                    id,
                    value: pattern,
                    attributes: attributes.into_boxed_slice(),
                    comment: self.last_comment.take(),
                },
            )));
        Ok(())
    }

    fn add_term(&mut self) -> ParserResult<()> {
        let id = self.get_identifier()?;

        self.lexer.expect(Token::EqSign)?;

        let pattern = self.get_pattern()?;

        let mut attributes = vec![];
        while self.lexer.take_if(Token::Dot)? {
            attributes.push(self.get_attribute()?);
        }

        self.body
            .push(ast::ResourceEntry::Entry(ast::Entry::Term(ast::Term {
                id,
                value: pattern,
                attributes: attributes.into_boxed_slice(),
                comment: self.last_comment.take(),
            })));
        Ok(())
    }

    fn maybe_get_pattern(&mut self) -> ParserResult<Option<ast::Pattern<'p>>> {
        // Arena?
        let mut pe = Vec::with_capacity(1);
        let mut indents = vec![];
        loop {
            match self.lexer.try_next()? {
                Some(Token::Text(r)) => {
                    pe.push(ast::PatternElement::TextElement(&self.source[r]));
                }
                Some(Token::Indent(i)) => match self.lexer.try_next()? {
                    Some(Token::Text(r)) => {
                        pe.push(ast::PatternElement::TextElement(&self.source[r.clone()]));
                        indents.push((i, Some(r), pe.len() - 1));
                    }
                    Some(Token::OpenCurlyBraces) => {
                        let exp = self.get_expression()?;
                        self.lexer.expect(Token::CloseCurlyBraces)?;
                        pe.push(ast::PatternElement::Placeable(exp));
                        indents.push((i, None, pe.len() - 1));
                    }
                    _ => return Err(ParserError::Unknown),
                },
                Some(Token::Eol(eols)) => {
                    for _ in 0..eols {
                        pe.push(ast::PatternElement::TextElement("\n"));
                    }
                }
                Some(Token::OpenCurlyBraces) => {
                    let exp = self.get_expression()?;
                    self.lexer.expect(Token::CloseCurlyBraces)?;
                    pe.push(ast::PatternElement::Placeable(exp));
                }
                Some(Token::Eot) => {
                    break;
                }
                _ => {
                    return Err(ParserError::Unknown);
                }
            }
        }
        if pe.is_empty() {
            Ok(None)
        } else {
            if !indents.is_empty() {
                let min_indent = indents.iter().map(|(i, _, _)| *i).min();
                if let Some(min_indent) = min_indent {
                    for (i, r, p) in indents {
                        let indent = i - min_indent;
                        match (unsafe { pe.get_unchecked_mut(p) }, r) {
                            (ast::PatternElement::TextElement(ref mut s), Some(ref r)) => {
                                *s = &self.source[r.start - indent..r.end];
                            }
                            (ast::PatternElement::Placeable(_), None) => {}
                            _ => unreachable!(),
                        }
                    }
                }
            }
            if pe.len() > 1 {
                while let Some(ast::PatternElement::TextElement("\n")) = pe.last() {
                    pe.pop();
                }
            }
            // if let Some(ast::PatternElement::TextElement(ref mut s)) = pe.last_mut() {
            //     *s = s.trim_right();
            // }
            Ok(Some(ast::Pattern {
                elements: pe.into_boxed_slice(),
            }))
        }
    }

    fn get_pattern(&mut self) -> ParserResult<ast::Pattern<'p>> {
        if let Some(pattern) = self.maybe_get_pattern()? {
            Ok(pattern)
        } else {
            Err(ParserError::Unknown)
        }
    }

    fn get_identifier(&mut self) -> ParserResult<ast::Identifier<'p>> {
        match self.lexer.try_next()? {
            Some(Token::Identifier(r)) => Ok(ast::Identifier {
                name: &self.source[r],
            }),
            _ => Err(ParserError::Unknown),
        }
    }

    fn get_attribute(&mut self) -> ParserResult<ast::Attribute<'p>> {
        let id = self.get_identifier()?;
        self.lexer.expect(Token::EqSign)?; // EqSign
        match self.maybe_get_pattern()? {
            Some(value) => Ok(ast::Attribute { id, value }),
            None => Err(ParserError::Unknown),
        }
    }

    fn add_comment(&mut self, token: &Token) -> ParserResult<()> {
        let mut pe = Vec::with_capacity(1);

        let result;
        loop {
            match self.add_comment_line(token, &mut pe) {
                Ok(true) => {}
                Ok(false) => {
                    result = Ok(());
                    break;
                }
                Err(err) => {
                    result = Err(err);
                    break;
                }
            }
        }
        let comment_type = match token {
            Token::CommentSign => ast::CommentType::Regular,
            Token::GroupCommentSign => ast::CommentType::Group,
            Token::ResourceCommentSign => ast::CommentType::Resource,
            _ => unreachable!(),
        };
        self.last_comment = Some(ast::Comment {
            comment_type,
            content: pe.into_boxed_slice(),
        });
        result
    }

    fn add_comment_line(&mut self, token: &Token, pe: &mut Vec<&'p str>) -> ParserResult<bool> {
        match self.lexer.try_next()? {
            Some(Token::Text(r)) => {
                pe.push(&self.source[r.start..r.end]);
            }
            Some(Token::Eol(_)) => {
                pe.push("");
            }
            _ => {
                return Err(ParserError::Unknown);
            }
        }
        match self.lexer.try_peek()? {
            Some(c @ Token::CommentSign)
            | Some(c @ Token::GroupCommentSign)
            | Some(c @ Token::ResourceCommentSign)
                if c == token =>
            {
                self.lexer.try_next()?;
            }
            _ => {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn get_expression(&mut self) -> ParserResult<ast::Expression<'p>> {
        self.get_inline_expression()
            .map(ast::Expression::InlineExpression)
    }

    fn get_inline_expression(&mut self) -> ParserResult<ast::InlineExpression<'p>> {
        match self.lexer.try_next()? {
            Some(Token::MinusSign) => match self.lexer.try_next()? {
                Some(Token::Number(r)) => {
                    let num = self.get_number(r, true)?;
                    Ok(ast::InlineExpression::NumberLiteral {
                        value: &self.source[num],
                    })
                }
                Some(Token::Identifier(r)) => {
                    let id = ast::Identifier {
                        name: &self.source[r],
                    };
                    Ok(ast::InlineExpression::TermReference {
                        id,
                        attribute: None,
                        arguments: None,
                    })
                }
                _ => Err(ParserError::Unknown),
            },
            Some(Token::Number(r)) => {
                let num = self.get_number(r, false)?;
                Ok(ast::InlineExpression::NumberLiteral {
                    value: &self.source[num],
                })
            }
            Some(Token::Text(r)) => Ok(ast::InlineExpression::StringLiteral {
                value: &self.source[r],
            }),
            _ => Err(ParserError::Unknown),
        }
    }

    fn get_number(&mut self, decimal: Range<usize>, minus: bool) -> ParserResult<Range<usize>> {
        let mut result = decimal;
        if minus {
            result.start -= 1;
        }
        if self.lexer.take_if(Token::Dot)? {
            match self.lexer.try_next()? {
                Some(Token::Number(r)) => {
                    result.end = r.end;
                }
                _ => {
                    return Err(ParserError::Unknown);
                }
            }
        }

        Ok(result)
    }
}
