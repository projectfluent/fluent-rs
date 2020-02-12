use super::ast;
use super::lexer::Lexer;
use super::lexer::Token;
use std::ops::Range;

pub struct Parser<'p> {
    source: &'p str,
    lexer: Lexer<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            lexer: Lexer::new(source.as_bytes()),
            source,
        }
    }

    pub fn parse(mut self) -> ast::Resource<'p> {
        let mut body = vec![];

        let mut last_comment = None;

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(r) => {
                    self.add_message(r, &mut body, last_comment.take());
                }
                Token::MinusSign => {
                    let term = self.get_term(last_comment.take());
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Term(term)));
                }
                Token::Eol(_) => {
                    if let Some(comment) = last_comment.take() {
                        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                    }
                }
                c @ Token::CommentSign
                | c @ Token::GroupCommentSign
                | c @ Token::ResourceCommentSign => {
                    self.add_comment(&c, &mut body, &mut last_comment);
                }
                Token::Junk(r) => {
                    if let Some(comment) = last_comment.take() {
                        body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                    }
                    body.push(ast::ResourceEntry::Junk(&self.source[r]));
                }
                _ => panic!(),
            }
        }

        if let Some(comment) = last_comment.take() {
            body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
        }

        ast::Resource {
            body: body.into_boxed_slice(),
        }
    }

    fn add_message(
        &mut self,
        id: Range<usize>,
        body: &mut Vec<ast::ResourceEntry<'p>>,
        comment: Option<ast::Comment<'p>>,
    ) {
        let id = ast::Identifier {
            name: &self.source[id],
        };

        match self.lexer.next() {
            Some(Token::EqSign) => {}
            Some(Token::Junk(r)) => {
                if let Some(comment) = comment {
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                }
                body.push(ast::ResourceEntry::Junk(&self.source[r]));
                return;
            }
            None => {
                if let Some(comment) = comment {
                    body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
                }
                let junk = self.lexer.get_junk();
                body.push(ast::ResourceEntry::Junk(&self.source[junk]));
                return;
            }
            _ => panic!(),
        };

        let pattern = self.maybe_get_pattern();

        let mut attributes = vec![];
        while let Some(Token::Dot) = self.lexer.peek() {
            attributes.push(self.get_attribute());
        }

        if pattern.is_none() && attributes.is_empty() {
            if let Some(comment) = comment {
                body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
            }
            let junk = self.lexer.get_junk();
            body.push(ast::ResourceEntry::Junk(&self.source[junk]));
            return;
        }

        body.push(ast::ResourceEntry::Entry(ast::Entry::Message(
            ast::Message {
                id,
                value: pattern,
                attributes: attributes.into_boxed_slice(),
                comment,
            },
        )));
    }

    fn get_term(&mut self, comment: Option<ast::Comment<'p>>) -> ast::Term<'p> {
        let id = self.get_identifier();

        self.lexer.next(); // EqSign

        let pattern = self.get_pattern();

        let mut attributes = vec![];
        while let Some(Token::Dot) = self.lexer.peek() {
            attributes.push(self.get_attribute());
        }
        ast::Term {
            id,
            value: pattern,
            attributes: attributes.into_boxed_slice(),
            comment,
        }
    }

    fn maybe_get_pattern(&mut self) -> Option<ast::Pattern<'p>> {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(&self.source[r]);
                    pe.push(te);
                }
                Some(Token::Eol(i)) => {
                    if !pe.is_empty() {
                        let te = ast::PatternElement::TextElement(&self.source[i..=i]);
                        pe.push(te);
                    }
                }
                Some(Token::OpenCurlyBraces) => {
                    let expr = self.get_expression();
                    pe.push(ast::PatternElement::Placeable(expr));
                }
                Some(Token::Eot) | None => {
                    break;
                }
                _ => panic!(),
            };
        }
        if pe.is_empty() {
            None
        } else {
            Some(ast::Pattern {
                elements: pe.into_boxed_slice(),
            })
        }
    }

    fn get_pattern(&mut self) -> ast::Pattern<'p> {
        let mut pe = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let te = ast::PatternElement::TextElement(&self.source[r]);
                    pe.push(te);
                }
                Some(Token::Eol(i)) => {
                    if !pe.is_empty() {
                        let te = ast::PatternElement::TextElement(&self.source[i..=i]);
                        pe.push(te);
                    }
                }
                Some(Token::OpenCurlyBraces) => {
                    let expr = self.get_expression();
                    pe.push(ast::PatternElement::Placeable(expr));
                }
                Some(Token::Eot) | None => {
                    break;
                }
                _ => panic!(),
            }
        }
        ast::Pattern {
            elements: pe.into_boxed_slice(),
        }
    }

    fn get_identifier(&mut self) -> ast::Identifier<'p> {
        match self.lexer.next() {
            Some(Token::Identifier(r)) => ast::Identifier {
                name: &self.source[r],
            },
            _ => panic!(),
        }
    }

    fn get_attribute(&mut self) -> ast::Attribute<'p> {
        self.lexer.next(); // Dot
        let id = self.get_identifier();
        self.lexer.next(); // EqSign
        let value = self.get_pattern();
        ast::Attribute { id, value }
    }

    fn add_comment(
        &mut self,
        token: &Token,
        body: &mut Vec<ast::ResourceEntry<'p>>,
        last_comment: &mut Option<ast::Comment<'p>>,
    ) {
        if let Some(comment) = last_comment.take() {
            body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(comment)));
        }
        let mut pe = vec![];
        let comment_type_token = token;
        let comment_type = match comment_type_token {
            Token::CommentSign => ast::CommentType::Regular,
            Token::GroupCommentSign => ast::CommentType::Group,
            Token::ResourceCommentSign => ast::CommentType::Resource,
            _ => panic!(),
        };
        let mut junk = None;
        match self.lexer.next() {
            Some(Token::Text(indent, mut r)) => {
                if indent > 0 {
                    r.start -= indent - 1;
                }
                pe.push(&self.source[r]);
            }
            Some(Token::Junk(r)) => {
                junk = Some(r);
            }
            _ => panic!(),
        }
        if junk.is_none() {
            while let Some(token) = self.lexer.peek() {
                if token != comment_type_token {
                    break;
                }
                self.lexer.next();
                match self.lexer.next() {
                    Some(Token::Text(indent, mut r)) => {
                        if indent > 0 {
                            r.start -= indent - 1;
                        }
                        pe.push(&self.source[r]);
                    }
                    Some(Token::Junk(r)) => {
                        junk = Some(r);
                        break;
                    }
                    _ => panic!(),
                }
            }
        }
        if let Some(r) = junk {
            if !pe.is_empty() {
                body.push(ast::ResourceEntry::Entry(ast::Entry::Comment(
                    ast::Comment {
                        comment_type,
                        content: pe.into_boxed_slice(),
                    },
                )));
            }
            body.push(ast::ResourceEntry::Junk(&self.source[r]));
        } else if !pe.is_empty() {
            *last_comment = Some(ast::Comment {
                comment_type,
                content: pe.into_boxed_slice(),
            });
        }
    }

    fn get_expression(&mut self) -> ast::Expression<'p> {
        let inline_expr = self.get_inline_expression();
        self.lexer.next(); // CloseCurlyBraces
        ast::Expression::InlineExpression(inline_expr)
    }

    fn get_inline_expression(&mut self) -> ast::InlineExpression<'p> {
        match self.lexer.next() {
            Some(Token::DoubleQuote) => match self.lexer.next() {
                Some(Token::Text(_, r)) => {
                    let value = &self.source[r];
                    self.lexer.next(); // DoubleQuote
                    ast::InlineExpression::StringLiteral { value }
                }
                _ => panic!(),
            },
            Some(Token::Number(r)) => {
                let range = self.get_number(r, false);
                let value = &self.source[range];
                ast::InlineExpression::NumberLiteral { value }
            }
            Some(Token::MinusSign) => {
                match self.lexer.next() {
                    Some(Token::Identifier(r)) => {
                        let id = ast::Identifier {
                            name: &self.source[r],
                        };
                        ast::InlineExpression::TermReference {
                            id,
                            attribute: None,
                            arguments: None,
                        }
                    }
                    Some(Token::Number(r)) => {
                        let range = self.get_number(r, true);
                        let value = &self.source[range];
                        ast::InlineExpression::NumberLiteral { value }
                    }
                    _ => panic!(),
                }
                // Some(Token::MinusSign) => {
                //     let ident = self.get_identifier();
                //     break;
                // }
            }
            _ => panic!(),
        }
    }

    fn get_number(&mut self, decimal: Range<usize>, minus: bool) -> Range<usize> {
        let mut result = decimal;
        if minus {
            result.start -= 1;
        }
        if let Some(Token::Dot) = self.lexer.peek() {
            self.lexer.next();
            match self.lexer.next() {
                Some(Token::Number(r)) => {
                    result.end = r.end;
                }
                _ => panic!(),
            }
        }

        result
    }
}
