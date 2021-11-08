use crate::ast::*;
use std::fmt::{self, Error, Write};

pub fn serialize(resource: &Resource<&str>) -> String {
    serialize_with_options(resource, Options::default())
}

pub fn serialize_with_options(resource: &Resource<&str>, options: Options) -> String {
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

    pub fn serialize_resource(&mut self, res: &Resource<&str>) -> Result<(), Error> {
        for entry in &res.body {
            match entry {
                Entry::Message(msg) => self.serialize_message(msg)?,
                Entry::Term(term) => self.serialize_term(term)?,
                Entry::Comment(comment) => self.serialize_free_comment(comment, "#")?,
                Entry::GroupComment(comment) => self.serialize_free_comment(comment, "##")?,
                Entry::ResourceComment(comment) => self.serialize_free_comment(comment, "###")?,
                Entry::Junk { content } if self.options.with_junk => {
                    self.serialize_junk(content)?
                }
                Entry::Junk { .. } => continue,
            }

            self.state.has_entries = true;
        }

        Ok(())
    }

    pub fn into_serialized_text(self) -> String {
        self.writer.buffer
    }

    fn serialize_junk(&mut self, junk: &str) -> Result<(), Error> {
        self.writer.write_literal(junk)
    }

    fn serialize_free_comment(
        &mut self,
        comment: &Comment<&str>,
        prefix: &str,
    ) -> Result<(), Error> {
        if self.state.has_entries {
            self.writer.newline();
        }
        self.serialize_comment(comment, prefix)?;
        self.writer.newline();

        Ok(())
    }

    fn serialize_comment(&mut self, comment: &Comment<&str>, prefix: &str) -> Result<(), Error> {
        for line in &comment.content {
            self.writer.write_literal(prefix)?;

            if !line.trim().is_empty() {
                self.writer.write_literal(" ")?;
                self.writer.write_literal(line)?;
            }

            self.writer.newline();
        }

        Ok(())
    }

    fn serialize_message(&mut self, msg: &Message<&str>) -> Result<(), Error> {
        if let Some(comment) = msg.comment.as_ref() {
            self.serialize_comment(comment, "#")?;
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

    fn serialize_term(&mut self, term: &Term<&str>) -> Result<(), Error> {
        if let Some(comment) = term.comment.as_ref() {
            self.serialize_comment(comment, "#")?;
        }

        self.writer.write_literal("-")?;
        self.writer.write_literal(&term.id.name)?;
        self.writer.write_literal(" =")?;
        self.serialize_pattern(&term.value)?;

        self.serialize_attributes(&term.attributes)?;

        self.writer.newline();

        Ok(())
    }

    fn serialize_pattern(&mut self, pattern: &Pattern<&str>) -> Result<(), Error> {
        let start_on_newline = pattern.elements.iter().any(|elem| match elem {
            PatternElement::TextElement { value } => value.contains("\n"),
            PatternElement::Placeable { expression } => is_select_expr(expression),
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

    fn serialize_attributes(&mut self, attrs: &[Attribute<&str>]) -> Result<(), Error> {
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

    fn serialize_attribute(&mut self, attr: &Attribute<&str>) -> Result<(), Error> {
        self.writer.write_literal(".")?;
        self.writer.write_literal(&attr.id.name)?;
        self.writer.write_literal(" =")?;

        self.serialize_pattern(&attr.value)?;

        Ok(())
    }

    fn serialize_element(&mut self, elem: &PatternElement<&str>) -> Result<(), Error> {
        match elem {
            PatternElement::TextElement { value } => self.writer.write_literal(value),
            PatternElement::Placeable { expression } => match expression {
                Expression::Inline(InlineExpression::Placeable { expression }) => {
                    // A placeable inside a placeable is a special case because we
                    // don't want the braces to look silly (e.g. "{ { Foo() } }").
                    self.writer.write_literal("{{ ")?;
                    self.serialize_expression(expression)?;
                    self.writer.write_literal(" }}")?;
                    Ok(())
                }
                Expression::Select { .. } => {
                    // select adds its own newline and indent, emit the brace
                    // *without* a space so we don't get 5 spaces instead of 4
                    self.writer.write_literal("{ ")?;
                    self.serialize_expression(expression)?;
                    self.writer.write_literal("}")?;
                    Ok(())
                }
                Expression::Inline(_) => {
                    self.writer.write_literal("{ ")?;
                    self.serialize_expression(expression)?;
                    self.writer.write_literal(" }")?;
                    Ok(())
                }
            },
        }
    }

    fn serialize_expression(&mut self, expr: &Expression<&str>) -> Result<(), Error> {
        match expr {
            Expression::Inline(inline) => self.serialize_inline_expression(inline),
            Expression::Select { selector, variants } => {
                self.serialize_select_expression(selector, variants)
            }
        }
    }

    fn serialize_inline_expression(&mut self, expr: &InlineExpression<&str>) -> Result<(), Error> {
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
                self.serialize_call_arguments(arguments)?;

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
            InlineExpression::Placeable { expression } => {
                self.writer.write_literal("{")?;
                self.serialize_expression(expression)?;
                self.writer.write_literal("}")?;

                Ok(())
            }
        }
    }

    fn serialize_select_expression(
        &mut self,
        selector: &InlineExpression<&str>,
        variants: &[Variant<&str>],
    ) -> Result<(), Error> {
        self.serialize_inline_expression(selector)?;
        self.writer.write_literal(" ->")?;

        self.writer.newline();
        self.writer.indent();

        for variant in variants {
            self.serialize_variant(variant)?;
            self.writer.newline();
        }

        self.writer.dedent();
        Ok(())
    }

    fn serialize_variant(&mut self, variant: &Variant<&str>) -> Result<(), Error> {
        if variant.default {
            self.writer.write_literal("*")?;
        }

        self.writer.write_literal("[")?;
        self.serialize_variant_key(&variant.key)?;
        self.writer.write_literal("]")?;
        self.serialize_pattern(&variant.value)?;

        Ok(())
    }

    fn serialize_variant_key(&mut self, key: &VariantKey<&str>) -> Result<(), Error> {
        match key {
            VariantKey::NumberLiteral { value } | VariantKey::Identifier { name: value } => {
                self.writer.write_literal(value)
            }
        }
    }

    fn serialize_call_arguments(&mut self, args: &CallArguments<&str>) -> Result<(), Error> {
        let mut argument_written = false;

        self.writer.write_literal("(")?;

        for positional in &args.positional {
            if argument_written {
                self.writer.write_literal(", ")?;
            }

            self.serialize_inline_expression(positional)?;
            argument_written = true;
        }

        for named in &args.named {
            if argument_written {
                self.writer.write_literal(", ")?;
            }

            self.writer.write_literal(&named.name.name)?;
            self.writer.write_literal(": ")?;
            self.serialize_inline_expression(&named.value)?;
            argument_written = true;
        }

        self.writer.write_literal(")")?;
        Ok(())
    }
}

fn is_select_expr(expr: &Expression<&str>) -> bool {
    match expr {
        Expression::Select { .. } => true,
        Expression::Inline(InlineExpression::Placeable { expression }) => {
            is_select_expr(&*expression)
        }
        Expression::Inline(_) => false,
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

    fn write_literal(&mut self, mut item: &str) -> fmt::Result {
        if self.buffer.ends_with("\n") {
            // we've just added a newline, make sure it's properly indented
            self.write_indent();

            // we've just added indentation, so we don't care about leading
            // spaces
            item = item.trim_start();
        }

        write!(self.buffer, "{}", item)
    }
}

#[cfg(test)]
mod serialize_resource_tests {
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
                // Note: We add tabs to the input so it's easier to recognise
                // indentation
                let input_without_tabs = $text.replace("\t", "    ");
                let should_be_without_tabs = $should_be.replace("\t", "    ");

                let resource = crate::parser::parse(input_without_tabs.as_str()).unwrap();
                let got = serialize(&resource);

                assert_eq!(got, should_be_without_tabs);
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
        dont_prefix_a_subsequent_entry_comment_with_a_newline,
        "first = Firstsubsequent_ Comment\nfoo = Foo\n",
    );
    round_trip_test!(
        group_comment,
        "## Comment Header\n##\n## A multiline\n## group comment.\n\nfoo = Foo\n",
    );
    round_trip_test!(
        standalone_comment,
        "foo = Foo\n\n# A Standalone Comment\n\nbar = Bar\n",
    );
    round_trip_test!(multiline_with_placeable, "foo =\n\tFoo { bar }\n\tBaz\n",);
    round_trip_test!(attribute, "foo =\n\t.attr = Foo Attr\n");
    round_trip_test!(
        multiline_attribute,
        "foo =\n\t.attr =\n\t\tFoo Attr\n\t\tContinued\n",
    );
    round_trip_test!(
        two_attributes,
        "foo =\n\t.attr-a = Foo Attr A\n\t.attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        value_and_attributes,
        "foo = Foo Value\n\t.attr-a = Foo Attr A\n\t.attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        multiline_value_and_attributes,
        "foo =\n\tFoo Value\n\tContinued\n\t.attr-a = Foo Attr A\n\t.attr-b = Foo Attr B\n",
    );
    round_trip_test!(
        select_expression,
        "foo =\n\t{ $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
    );
    round_trip_test!(
        multiline_variant,
        "foo =\n\t{ $sel ->\n\t\t*[a]\n\t\t\tAAA\n\t\t\tBBBB\n\t}\n",
    );
    round_trip_test!(
        multiline_variant_with_first_line_inline,
        "foo =\n\t{ $sel ->\n\t\t*[a] AAA\n\t\tBBB\n\t}\n",
        "foo =\n\t{ $sel ->\n\t\t*[a]\n\t\t\tAAA\n\t\t\tBBB\n\t}\n",
    );
    round_trip_test!(
        variant_key_number,
        "foo =\n\t{ $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
    );
    round_trip_test!(
        select_expression_in_block_value,
        "foo =\n\tFoo { $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
    );
    round_trip_test!(
        select_expression_in_inline_value,
        "foo = Foo { $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
        "foo =\n\tFoo { $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
    );
    round_trip_test!(
        select_expression_in_multiline_value,
        "foo =\n\tFoo\n\tBar { $sel ->\n\t\t*[a] A\n\t\t[b] B\n\t}\n",
    );
    round_trip_test!(
        nested_select_expression,
        "foo =\n\t{ $a ->\n\t\t*[a]\n\t\t\t{ $b ->\n\t\t\t\t*[b] Foo\n\t\t\t}\n\t}\n",
    );
    round_trip_test!(
        selector_external_argument,
        "foo =\n\t{ $bar ->\n\t\t*[a] A\n\t}\n",
    );
    round_trip_test!(
        selector_number_expression,
        "foo =\n\t{ 1 ->\n\t\t*[a] A\n\t}\n",
    );
    round_trip_test!(
        selector_string_expression,
        "foo =\n\t{ \"bar\" ->\n\t\t*[a] A\n\t}\n",
    );
    round_trip_test!(
        selector_attribute_expression,
        "foo =\n\t{ -bar.baz ->\n\t\t*[a] A\n\t}\n",
    );
    round_trip_test!(call_expression, "foo = { FOO() }\n",);
    round_trip_test!(
        call_expression_with_string_expression,
        "foo = { FOO(\"bar\") }\n",
    );
    round_trip_test!(call_expression_with_number_expression, "foo = { FOO(1) }\n",);
    round_trip_test!(
        call_expression_with_message_reference,
        "foo = { FOO(bar) }\n",
    );
    round_trip_test!(
        call_expression_with_external_argument,
        "foo = { FOO($bar) }\n",
    );
    round_trip_test!(
        call_expression_with_number_named_argument,
        "foo = { FOO(bar: 1) }\n",
    );
    round_trip_test!(
        call_expression_with_string_named_argument,
        "foo = { FOO(bar: \"bar\") }\n",
    );
    round_trip_test!(
        call_expression_with_two_positional_arguments,
        "foo = { FOO(bar, baz) }\n",
    );
    round_trip_test!(
        call_expression_with_positional_and_named_arguments,
        "foo = { FOO(bar, 1, baz: \"baz\") }\n",
    );
    round_trip_test!(macro_call, "foo = { -term() }\n",);
    round_trip_test!(nested_placeables, "foo = {{ FOO() }}\n",);
    round_trip_test!(backslash_in_text_element, "foo = \\{ placeable }\n",);
    round_trip_test!(
        excaped_special_char_in_string_literal,
        "foo = { \"Escaped \\\" quote\" }\n",
    );
    round_trip_test!(unicode_escape_sequence, "foo = { \"\\u0065\" }\n",);

    // Serialize padding around comments

    round_trip_test!(
        standalone_comment_has_not_padding_when_first,
        "# Comment A\n\nfoo = Foo\n\n# Comment B\n\nbar = Bar\n"
    );
    round_trip_test!(
        group_comment_has_not_padding_when_first,
        "## Group A\n\nfoo = Foo\n\n## Group B\n\nbar = Bar\n"
    );
    round_trip_test!(
        resource_comment_has_not_padding_when_first,
        "### Resource Comment A\n\nfoo = Foo\n\n### Resource Comment B\n\nbar = Bar\n"
    );
}

#[cfg(test)]
mod serialize_expression_tests {
    use super::*;

    macro_rules! expression_test {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                let input_without_tabs = $input.replace("\t", "    ");
                let src = format!("foo = {{ {} }}", input_without_tabs);
                let resource = crate::parser::parse(src.as_str()).unwrap();

                // extract the first expression from the value of the first
                // message
                assert_eq!(resource.body.len(), 1);
                let first_item = &resource.body[0];
                let message = match first_item {
                    Entry::Message(msg) => msg,
                    other => panic!("Expected a message but found {:#?}", other),
                };
                let value = message.value.as_ref().expect("The message has a value");
                assert_eq!(value.elements.len(), 1);
                let expr = match &value.elements[0] {
                    PatternElement::Placeable { expression } => expression,
                    other => panic!("Expected a single expression but found {:#?}", other),
                };

                // we've finally extracted the first expression, now we can
                // actually serialize it and finish the test
                let mut serializer = Serializer::new(Options::default());
                serializer.serialize_expression(expr).unwrap();
                let got = serializer.into_serialized_text();

                assert_eq!(got, input_without_tabs);
            }
        };
    }

    expression_test!(string_expression, "\"str\"");
    expression_test!(number_expression, "3");
    expression_test!(message_reference, "msg");
    expression_test!(external_arguemnt, "$ext");
    expression_test!(attribute_expression, "msg.attr");
    expression_test!(call_expression, "BUILTIN(3.14, kwarg: \"value\")");
    expression_test!(select_expression, "$num ->\n\t*[one] One\n");
}

#[cfg(test)]
mod serialize_variant_key_tests {
    use super::*;

    macro_rules! variant_key_test {
        ($name:ident, $input:expr => $( $keys:expr ),+ $(,)?) => {
            #[test]
            #[allow(unused_assignments)]
            fn $name() {
                let input_without_tabs = $input.replace("\t", "    ");
                let src = format!("foo = {{ {}\n }}", input_without_tabs);
                let resource = crate::parser::parse(src.as_str()).unwrap();

                // extract variant from the first expression from the value of
                // the first message
                assert_eq!(resource.body.len(), 1);
                let first_item = &resource.body[0];
                let message = match first_item {
                    Entry::Message(msg) => msg,
                    other => panic!("Expected a message but found {:#?}", other),
                };
                let value = message.value.as_ref().expect("The message has a value");
                assert_eq!(value.elements.len(), 1);
                let variants = match &value.elements[0] {
                    PatternElement::Placeable { expression: Expression::Select { variants, .. } } => variants,
                    other => panic!("Expected a single select expression but found {:#?}", other),
                };

                let mut ix = 0;

                $(
                    let variant_key = &variants[ix].key;

                    // we've finally extracted the variant key, now we can
                    // actually serialize it and finish the test
                    let mut serializer = Serializer::new(Options::default());
                    serializer.serialize_variant_key(variant_key).unwrap();
                    let got = serializer.into_serialized_text();

                    assert_eq!(got, $keys);

                    ix += 1;
                )*
            }
        };
    }

    variant_key_test!(identifiers, "$num ->\n\t[one] One\n\t*[other] Other" => "one", "other");
    variant_key_test!(
        number_literals,
        "$num ->\n\t[-123456789] Minus a lot\n\t[0] Zero\n\t*[3.14] Pi\n\t[007] James"
        => "-123456789", "0", "3.14", "007",
    );
}
