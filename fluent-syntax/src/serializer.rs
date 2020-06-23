use crate::ast::*;
use std::fmt::{self, Display, Error, Write};

pub fn serialize(resource: &Resource<'_>) -> String {
    let options = Options::default();
    let mut ser = Serializer::new(options);

    ser.serialize_resource(resource)
        .expect("Writing to an in-memory buffer never fails");

    ser.into_serialized_text()
}

#[derive(Debug)]
pub struct Serializer {
    writer: TextWriter,
    options: Options,
    state: State,
}

impl Serializer {
    pub fn new(options: Options) -> Self {
        Serializer {
            writer: TextWriter::default(),
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

    pub fn into_serialized_text(self) -> String {
        self.writer.buffer
    }

    fn serialize_entry(&mut self, entry: &Entry<'_>) -> Result<(), Error> {
        match entry {
            Entry::Message(msg) => self.serialize_message(msg),
            Entry::Comment(comment) => {
                self.serialize_comment(comment)?;
                self.writer.newline();
                Ok(())
            }
            Entry::Term(term) => self.serialize_term(term),
        }
    }

    fn serialize_junk(&mut self, junk: &str) -> Result<(), Error> {
        self.writer.write_literal(junk)
    }

    fn serialize_comment(&mut self, comment: &Comment<'_>) -> Result<(), Error> {
        let (prefix, lines) = match comment {
            Comment::Comment { content } => ("#", content),
            Comment::GroupComment { content } => ("##", content),
            Comment::ResourceComment { content } => ("###", content),
        };

        if self.state.has_entries {
            self.writer.newline();
        }

        for line in lines {
            self.writer.write_literal(prefix)?;

            if !line.trim().is_empty() {
                self.writer.write_literal(" ")?;
                self.writer.write_literal(line)?;
            }

            self.writer.newline();
        }

        Ok(())
    }

    fn serialize_message(&mut self, msg: &Message<'_>) -> Result<(), Error> {
        if let Some(comment) = msg.comment.as_ref() {
            self.serialize_comment(comment)?;
        }

        self.writer.write_literal(&msg.id.name)?;
        self.writer.write_literal(" =")?;

        if let Some(value) = msg.value.as_ref() {
            self.serialize_pattern(value)?;
        }

        self.serialize_attributes(&msg.attributes)?;

        self.writer.newline();
        Ok(())
    }

    fn serialize_term(&mut self, term: &Term<'_>) -> Result<(), Error> {
        if let Some(comment) = term.comment.as_ref() {
            self.serialize_comment(comment)?;
        }

        self.writer.write_literal("-")?;
        self.writer.write_literal(&term.id.name)?;
        self.writer.write_literal(" =")?;
        self.serialize_pattern(&term.value)?;

        self.serialize_attributes(&term.attributes)?;

        self.writer.newline();

        Ok(())
    }

    fn serialize_pattern(&mut self, pattern: &Pattern<'_>) -> Result<(), Error> {
        let start_on_newline = pattern.elements.iter().any(|elem| match elem {
            PatternElement::TextElement(text) => text.contains("\n"),
            PatternElement::Placeable(expr) => is_select_expr(expr),
        });

        if start_on_newline {
            self.writer.newline();
            self.writer.indent();
        } else {
            self.writer.write_literal(" ")?;
        }

        for element in &pattern.elements {
            self.serialize_element(element)?;
        }

        if start_on_newline {
            self.writer.dedent();
        }

        Ok(())
    }

    fn serialize_attributes(&mut self, attrs: &[Attribute<'_>]) -> Result<(), Error> {
        if attrs.is_empty() {
            return Ok(());
        }

        self.writer.indent();

        for attr in attrs {
            self.writer.newline();
            self.serialize_attribute(attr)?;
        }

        self.writer.dedent();

        Ok(())
    }

    fn serialize_attribute(&mut self, attr: &Attribute<'_>) -> Result<(), Error> {
        self.writer.write_literal(".")?;
        self.writer.write_literal(&attr.id.name)?;
        self.writer.write_literal(" =")?;

        self.serialize_pattern(&attr.value)?;

        Ok(())
    }

    fn serialize_element(&mut self, elem: &PatternElement<'_>) -> Result<(), Error> {
        match elem {
            PatternElement::TextElement(text) => self.writer.write_literal(text),
            PatternElement::Placeable(expr) => {
                self.writer.write_literal("{ ")?;
                self.serialize_expression(expr)?;
                self.writer.write_literal(" }")?;
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
            InlineExpression::StringLiteral { value } => {
                self.writer.write_literal("\"")?;
                self.writer.write_literal(value)?;
                self.writer.write_literal("\"")?;
                Ok(())
            }
            InlineExpression::NumberLiteral { value } => self.writer.write_literal(value),
            InlineExpression::VariableReference {
                id: Identifier { name: value },
            } => {
                self.writer.write_literal("$")?;
                self.writer.write_literal(value)?;
                Ok(())
            }
            InlineExpression::FunctionReference { id, arguments } => {
                self.writer.write_literal(&id.name)?;

                if let Some(args) = arguments.as_ref() {
                    self.serialize_call_arguments(args)?;
                }
                Ok(())
            }
            InlineExpression::MessageReference { id, attribute } => {
                self.writer.write_literal(&id.name)?;

                if let Some(attr) = attribute.as_ref() {
                    self.writer.write_literal(".")?;
                    self.writer.write_literal(&attr.name)?;
                }

                Ok(())
            }
            InlineExpression::TermReference {
                id,
                attribute,
                arguments,
            } => {
                self.writer.write_literal("-")?;
                self.writer.write_literal(&id.name)?;

                if let Some(attr) = attribute.as_ref() {
                    self.writer.write_literal(".")?;
                    self.writer.write_literal(&attr.name)?;
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

        self.writer.write_literal("(")?;

        for positional in &args.positional {
            if !argument_written {
                self.writer.write_literal(", ")?;
                argument_written = true;
            }

            self.serialize_inline_expression(positional)?;
        }

        for named in &args.named {
            if !argument_written {
                self.writer.write_literal(", ")?;
                argument_written = true;
            }

            self.writer.write_literal(&named.name.name)?;
            self.writer.write_literal(": ")?;
            self.serialize_inline_expression(&named.value)?;
        }

        self.writer.write_literal(")")?;
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

#[derive(Debug, Clone, Default)]
struct TextWriter {
    buffer: String,
    indent_level: usize,
}

impl TextWriter {
    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        self.indent_level = self
            .indent_level
            .checked_sub(1)
            .expect("Dedenting without a corresponding indent");
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.buffer.push_str("    ");
        }
    }

    fn newline(&mut self) {
        self.buffer.push_str("\n");
    }

    fn write_literal<D: Display>(&mut self, item: D) -> fmt::Result {
        if self.buffer.ends_with("\n") {
            // we've just added a newline, make sure it's properly indented
            self.write_indent();
        }

        write!(self.buffer, "{}", item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_something_then_indent() -> fmt::Result {
        let mut writer = TextWriter::default();

        writer.write_literal("foo =")?;
        writer.newline();
        writer.indent();
        writer.write_literal("first line")?;
        writer.newline();
        writer.write_literal("second line")?;
        writer.newline();
        writer.dedent();
        writer.write_literal("not indented")?;
        writer.newline();

        let got = &writer.buffer;
        assert_eq!(
            got,
            "foo =\n    first line\n    second line\nnot indented\n"
        );

        Ok(())
    }

    macro_rules! round_trip_test {
        ($name:ident, $text:expr $(,)?) => {
            round_trip_test!($name, $text, $text);
        };
        ($name:ident, $text:expr, $should_be:expr $(,)?) => {
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
        "foo =\n    Foo\n    Bar\n",
    );
    round_trip_test!(message_reference, "foo = Foo { bar }\n");
    round_trip_test!(term_reference, "foo = Foo { -bar }\n");
    round_trip_test!(external_reference, "foo = Foo { $bar }\n");
    round_trip_test!(number_element, "foo = Foo { 1 }\n");
    round_trip_test!(string_element, "foo = Foo { \"bar\" }\n");
    round_trip_test!(attribute_expression, "foo = Foo { bar.baz }\n");
    round_trip_test!(
        resource_comment,
        "### A multiline\n### resource comment.\n\nfoo = Foo\n",
    );
    round_trip_test!(
        message_comment,
        "# A multiline\n# message comment.\nfoo = Foo\n",
    );
    round_trip_test!(
        group_comment,
        "## Comment Header\n##\n## A multiline\n## group comment.\n\nfoo = Foo\n",
    );
    round_trip_test!(
        standalone_comment,
        "foo = Foo\n\n# A Standalone Comment\n\nbar = Bar\n",
    );
    round_trip_test!(
        multiline_with_placeable,
        "foo =\n    Foo { bar }\n    Baz\n",
    );
    round_trip_test!(attribute, "foo =\n    .attr = Foo Attr\n");
    round_trip_test!(
        multiline_attribute,
        "foo =\n    .attr =\n        Foo Attr\n        Continued\n",
    );
    round_trip_test!(
        two_attributes,
        "foo =\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        value_and_attributes,
        "foo = Foo Value\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        multiline_value_and_attributes,
        "foo =\n    Foo Value\n    Continued\n    .attr-a = Foo Attr A\n    .attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        select_expression,
        "foo =\n    { $sel ->\n        *[a] A\n        [b] B\n    }\n",
    );
    round_trip_test!(
        multiline_variant,
        "foo =\n    { $sel ->\n        *[a]\n            AAA\n           BBBB\n    }\n",
    );
    round_trip_test!(
        multiline_variant_with_first_line_inline,
        "foo =\n    { $sel ->\n        *[a] AAA\n        BBB\n    }\n",
        "foo =\n    { $sel ->\n        *[a]\n            AAA\n            BBB\n    }\n",
    );
    round_trip_test!(
        variant_key_number,
        "foo =\n    { $sel ->\n        *[a] A\n        [b] B\n    }\n",
    );
}
