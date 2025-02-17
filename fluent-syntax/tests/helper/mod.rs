use fluent_syntax::ast;

fn adapt_comment(comment: &mut ast::Comment<String>, _crlf: bool) {
    //XXX: We don't handle CRLF comments yet
    let mut content = vec![];
    for line in &comment.content {
        content.extend(line.split('\n').map(|s| s.to_string()));
    }
    comment.content = content;
}

fn adapt_pattern(pattern: &mut ast::Pattern<String>, crlf: bool) {
    let mut elements = vec![];
    for element in &pattern.elements {
        match element {
            ast::PatternElement::TextElement { value } => {
                let mut start = 0;
                let len = value.len();
                for (i, b) in value.as_bytes().iter().enumerate() {
                    if b == &b'\n' {
                        if crlf {
                            if i > start {
                                let chunk = &value.as_bytes()[start..=i - 1];
                                let value = String::from_utf8_lossy(chunk).to_string();
                                elements.push(ast::PatternElement::TextElement { value });
                            }
                            elements.push(ast::PatternElement::TextElement {
                                value: "\n".to_string(),
                            });
                        } else {
                            let chunk = &value.as_bytes()[start..=i];
                            let value = String::from_utf8_lossy(chunk).to_string();
                            elements.push(ast::PatternElement::TextElement { value });
                        }
                        start = i + 1;
                    }
                }
                if start < len {
                    let chunk = &value.as_bytes()[start..len];
                    let value = String::from_utf8_lossy(chunk).to_string();
                    elements.push(ast::PatternElement::TextElement { value });
                }
            }
            ast::PatternElement::Placeable { expression } => {
                let mut expression = expression.clone();
                adapt_expression(&mut expression, crlf);
                elements.push(ast::PatternElement::Placeable { expression });
            }
        }
    }
    pattern.elements = elements;
}

fn adapt_expression(expression: &mut ast::Expression<String>, crlf: bool) {
    match expression {
        ast::Expression::Select { variants, .. } => {
            for variant in variants {
                adapt_pattern(&mut variant.value, crlf);
            }
        }
        ast::Expression::Inline(_) => {}
    }
}

pub fn adapt_ast(ast: &mut ast::Resource<String>, crlf: bool) {
    for entry in &mut ast.body {
        match entry {
            ast::Entry::Comment(comment)
            | ast::Entry::GroupComment(comment)
            | ast::Entry::ResourceComment(comment) => {
                adapt_comment(comment, crlf);
            }
            ast::Entry::Message(msg) => {
                if let Some(pattern) = &mut msg.value {
                    adapt_pattern(pattern, crlf);
                }
                for attr in &mut msg.attributes {
                    adapt_pattern(&mut attr.value, crlf);
                }
                if let Some(comment) = &mut msg.comment {
                    adapt_comment(comment, crlf);
                }
            }
            ast::Entry::Term(term) => {
                adapt_pattern(&mut term.value, crlf);
                if let Some(comment) = &mut term.comment {
                    adapt_comment(comment, crlf);
                }
            }
            _ => {}
        }
    }
}

pub fn strip_comments(ast: &mut ast::Resource<String>) {
    ast.body.retain(|entry| match entry {
        // an arm that returns false makes clippy's match_like_matches_macro a false positive
        ast::Entry::Comment(..)
        | ast::Entry::GroupComment(..)
        | ast::Entry::ResourceComment(..) => false,
        _ => true,
    });

    for entry in &mut ast.body {
        match entry {
            ast::Entry::Message(ref mut msg) => {
                msg.comment = None;
            }
            ast::Entry::Term(ref mut term) => {
                term.comment = None;
            }
            _ => {}
        }
    }
}
