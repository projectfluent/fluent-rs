use crate::ast::*;
use std::io::{Error, Write};

pub fn serialize(resource: &Resource<'_>) -> String {
    let mut buffer = Vec::new();
    let options = Options::default();

    Serializer::new(&mut buffer, options)
        .serialize_resource(resource)
        .expect("Writing to an in-memory buffer never fails");

    String::from_utf8(buffer).expect("The serializer only ever emits valid UTF-8")
}

#[derive(Debug)]
pub struct Serializer<W> {
    writer: W,
    options: Options,
    state: State,
}

impl<W: Write> Serializer<W> {
    pub fn new(writer: W, options: Options) -> Self {
        Serializer {
            writer,
            options,
            state: State::default(),
        }
    }

    pub fn serialize_resource(&mut self, res: &Resource<'_>) -> Result<(), Error> {
        for entry in &res.body {
            match entry {
                ResourceEntry::Entry(entry) => self.serialize_entry(entry)?,
                ResourceEntry::Junk(junk) if self.options.with_junk => self.serialize_junk(junk)?,
                ResourceEntry::Junk(_) => continue,
            }

            self.state.has_entries = true;
        }

        Ok(())
    }

    fn serialize_entry(&mut self, entry: &Entry<'_>) -> Result<(), Error> {
        match entry {
            Entry::Message(msg) => self.serialize_message(msg),
            Entry::Comment(comment) => self.serialize_comment(comment),
            Entry::Term(term) => self.serialize_term(term),
        }
    }

    fn serialize_junk(&mut self, junk: &str) -> Result<(), Error> {
        write!(self.writer, "{}", junk)
    }

    fn serialize_comment(&mut self, comment: &Comment<'_>) -> Result<(), Error> {
        let (prefix, lines) = match comment {
            Comment::Comment { content } => ("#", content),
            Comment::GroupComment { content } => ("##", content),
            Comment::ResourceComment { content } => ("###", content),
        };

        if self.state.has_entries {
            writeln!(self.writer)?;
        }

        for line in lines {
            writeln!(self.writer, "{} {}", prefix, line)?;
        }

        Ok(())
    }

    fn serialize_message(&mut self, msg: &Message<'_>) -> Result<(), Error> {
        if let Some(comment) = msg.comment.as_ref() {
            self.serialize_comment(comment)?;
        }

        write!(self.writer, "{} =", msg.id.name)?;

        if let Some(value) = msg.value.as_ref() {
            self.serialize_pattern(value)?;
        }

        for attr in &msg.attributes {
            self.serialize_attribute(attr)?;
        }

        writeln!(self.writer)?;
        Ok(())
    }

    fn serialize_term(&mut self, term: &Term<'_>) -> Result<(), Error> {
        if let Some(comment) = term.comment.as_ref() {
            self.serialize_comment(comment)?;
        }

        write!(self.writer, "{} =", term.id.name)?;
        self.serialize_pattern(&term.value)?;

        for attr in &term.attributes {
            self.serialize_attribute(attr)?;
        }

        writeln!(self.writer)?;

        Ok(())
    }

    fn serialize_pattern(&mut self, pattern: &Pattern<'_>) -> Result<(), Error> {
        let start_on_newline = pattern.elements.iter().any(|elem| match elem {
            PatternElement::TextElement(text) => text.contains("\n"),
            PatternElement::Placeable(expr) => is_select_expr(expr),
        });

        if start_on_newline {
            unimplemented!("Write a new line then indent everything afterwards");
        }

        write!(self.writer, " ")?;

        for element in &pattern.elements {
            self.serialize_element(element)?;
        }

        Ok(())
    }

    fn serialize_attribute(&mut self, attr: &Attribute<'_>) -> Result<(), Error> {
        writeln!(self.writer)?;
        write!(self.writer, "    .{} =", attr.id.name)?;
        self.serialize_pattern(&attr.value)?;

        Ok(())
    }

    fn serialize_element(&mut self, elem: &PatternElement<'_>) -> Result<(), Error> {
        match elem {
            PatternElement::TextElement(text) => write!(self.writer, "{}", text),
            PatternElement::Placeable(expr) => {
                write!(self.writer, "{{ ")?;
                self.serialize_expression(expr)?;
                write!(self.writer, " }}")?;
                Ok(())
            }
        }
    }

    fn serialize_expression(&mut self, expr: &Expression<'_>) -> Result<(), Error> {
        match expr {
            Expression::InlineExpression(inline) => self.serialize_inline_expression(inline),
            Expression::SelectExpression { selector, variants } => {
                self.serialize_select_expression(selector, variants)
            }
        }
    }

    fn serialize_inline_expression(&mut self, expr: &InlineExpression<'_>) -> Result<(), Error> {
        match expr {
            InlineExpression::StringLiteral { value } => write!(self.writer, "\"{}\"", value),
            InlineExpression::NumberLiteral { value } => write!(self.writer, "{}", value),
            InlineExpression::VariableReference {
                id: Identifier { name: value },
            } => write!(self.writer, "${}", value),
            InlineExpression::FunctionReference { id, arguments } => {
                write!(self.writer, "{}", id.name)?;
                if let Some(args) = arguments.as_ref() {
                    self.serialize_call_arguments(args)?;
                }
                Ok(())
            }
            InlineExpression::MessageReference { id, attribute } => {
                write!(self.writer, "{}", id.name)?;

                if let Some(attr) = attribute.as_ref() {
                    write!(self.writer, ".{}", attr.name)?;
                }

                Ok(())
            }
            InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                write!(self.writer, "-{}", id.name)?;

                if let Some(attr) = attribute.as_ref() {
                    write!(self.writer, ".{}", attr.name)?;
                }
                if let Some(args) = arguments.as_ref() {
                    self.serialize_call_arguments(args)?;
                }

                Ok(())
            }
            InlineExpression::Placeable { expression } => self.serialize_expression(expression),
        }
    }

    fn serialize_select_expression(
        &mut self,
        _selector: &InlineExpression<'_>,
        _variants: &[Variant<'_>],
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn serialize_call_arguments(&mut self, args: &CallArguments<'_>) -> Result<(), Error> {
        let mut argument_written = false;

        write!(self.writer, "(")?;

        for positional in &args.positional {
            if !argument_written {
                write!(self.writer, ", ")?;
                argument_written = true;
            }

            self.serialize_inline_expression(positional)?;
        }

        for named in &args.named {
            if !argument_written {
                write!(self.writer, ", ")?;
                argument_written = true;
            }

            write!(self.writer, "{}: ", named.name.name)?;
            self.serialize_inline_expression(&named.value)?;
        }

        write!(self.writer, ")")?;
        Ok(())
    }
}

fn is_select_expr(expr: &Expression) -> bool {
    match expr {
        Expression::SelectExpression { .. } => true,
        Expression::InlineExpression(InlineExpression::Placeable { expression }) => {
            is_select_expr(&*expression)
        }
        Expression::InlineExpression(_) => false,
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Options {
    pub with_junk: bool,
}

#[derive(Debug, Default, PartialEq)]
struct State {
    has_entries: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! round_trip_test {
        ($name:ident, $text:expr) => {
            round_trip_test!($name, $text, $text);
        };
        ($name:ident, $text:expr, $should_be:expr) => {
            #[test]
            fn $name() {
                let resource = crate::parser::parse($text).unwrap();
                let got = serialize(&resource);

                assert_eq!(got, $should_be);
            }
        };
    }

    round_trip_test!(simple_message_without_eol, "foo = Foo", "foo = Foo\n");
    round_trip_test!(simple_message, "foo = Foo\n");
    round_trip_test!(two_simple_messages, "foo = Foo\nbar = Bar\n");
    round_trip_test!(block_multiline_message, "foo =\n    Foo\n    Bar\n");
    round_trip_test!(
        inline_multiline_message,
        "foo = Foo\n    Bar\n",
        "foo =\n    Foo\n    Bar\n"
    );
    round_trip_test!(message_reference, "foo = Foo { bar }\n");
    round_trip_test!(term_reference, "foo = Foo { -bar }\n");
    round_trip_test!(external_reference, "foo = Foo { $bar }\n");
    round_trip_test!(number_element, "foo = Foo { 1 }\n");
    round_trip_test!(string_element, "foo = Foo { \"bar\" }\n");
    round_trip_test!(attribute_expression, "foo = Foo { bar.baz }\n");
    round_trip_test!(
        resource_comment,
        "### A multiline\n### resource comment.\n\nfoo = Foo\n"
    );
    round_trip_test!(
        message_comment,
        "# A multiline\n# message comment.\nfoo = Foo\n"
    );
    round_trip_test!(
        group_comment,
        "## Comment Header\n##\n## A multiline\n# group comment.\n\nfoo = Foo\n"
    );
    round_trip_test!(
        standalone_comment,
        "foo = Foo\n\n# A Standalone Comment\n\nbar = Bar\n"
    );
    round_trip_test!(
        multiline_with_placeable,
        "foo =\n    Foo { bar }\n    Baz\n"
    );
    round_trip_test!(attribute, "foo =\n    .attr = Foo Attr\n");
    round_trip_test!(
        multiline_attribute,
        "foo =\n    .attr =\n        Foo Attr\n        Continued\n"
    );
    round_trip_test!(
        two_attributes,
        "foo =\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n"
    );
    round_trip_test!(
        value_and_attributes,
        "foo = Foo Value\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n"
    );
    round_trip_test!(
        multiline_value_and_attributes,
        "foo = Foo Value\n    Continued\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n"
    );
    round_trip_test!(
        select_expression,
        "foo =\n    { $sel ->\n        *[a] A\n        [b] B\n    }\n"
    );
    round_trip_test!(
        multiline_variant,
        "foo =\n    { $sel ->\n        *[a]\n            AAA\n           BBBB\n    }\n"
    );
    round_trip_test!(
        multiline_variant_with_first_line_inline,
        "foo =\n    { $sel ->\n        *[a] AAA\n        BBB\n    }\n",
        "foo =\n    { $sel ->\n        *[a]\n            AAA\n            BBB\n    }\n"
    );
}
