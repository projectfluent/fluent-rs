use fluent_syntax::ast;

fn adapt_comment(comment: &mut ast::Comment<String>) {
    let mut content = vec![];
    for line in &comment.content {
        content.extend(line.split('\n').map(|s| s.to_string()));
    }
    comment.content = content;
}

fn adapt_pattern(pattern: &mut ast::Pattern<String>) {
    let mut elements = vec![];
    for element in &pattern.elements {
        match element {
            ast::PatternElement::TextElement { value } => {
                let mut start = 0;
                let len = value.as_bytes().len();
                for (i, b) in value.as_bytes().iter().enumerate() {
                    if b == &b'\n' {
                        let chunk = &value.as_bytes()[start..=i];
                        let value = String::from_utf8_lossy(chunk).to_string();
                        elements.push(ast::PatternElement::TextElement { value });
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
                adapt_expression(&mut expression);
                elements.push(ast::PatternElement::Placeable { expression });
            }
        }
    }
    pattern.elements = elements;
}

fn adapt_expression(expression: &mut ast::Expression<String>) {
    match expression {
        ast::Expression::SelectExpression { variants, .. } => {
            for variant in variants {
                adapt_pattern(&mut variant.value);
            }
        }
        ast::Expression::InlineExpression(_) => {}
    }
}

pub fn adapt_ast(ast: &mut ast::Resource<String>) {
    for entry in &mut ast.body {
        match entry {
            ast::Entry::Comment(comment)
            | ast::Entry::GroupComment(comment)
            | ast::Entry::ResourceComment(comment) => {
                adapt_comment(comment);
            }
            ast::Entry::Message(msg) => {
                if let Some(pattern) = &mut msg.value {
                    adapt_pattern(pattern);
                }
                for attr in &mut msg.attributes {
                    adapt_pattern(&mut attr.value);
                }
                if let Some(comment) = &mut msg.comment {
                    adapt_comment(comment);
                }
            }
            ast::Entry::Term(term) => {
                adapt_pattern(&mut term.value);
                if let Some(comment) = &mut term.comment {
                    adapt_comment(comment);
                }
            }
            _ => {}
        }
    }
}
