use crate::ast::*;

/// An implementation of the [Visitor Pattern][wiki] for recursively traversing
/// the AST.
///
/// # Note to Implementors
///
/// Each method has a default implementation which will continue the traversal.
/// If a method is overridden you must do the recursive call manually, various
/// `visit_XXX()` functions are defined in this module which can be used to
/// continue the traversal.
///
/// [wiki]: https://en.wikipedia.org/wiki/Visitor_pattern
pub trait Visitor {
    fn visit_resource(&mut self, resource: &Resource<'_>) {
        visit_resource(self, resource);
    }

    fn visit_resource_entry(&mut self, entry: &ResourceEntry<'_>) {
        visit_resource_entry(self, entry);
    }

    fn visit_entry(&mut self, entry: &Entry<'_>) {
        visit_entry(self, entry);
    }

    fn visit_junk(&mut self, _junk: &str) {}

    fn visit_message(&mut self, msg: &Message<'_>) {
        visit_message(self, msg);
    }

    fn visit_term(&mut self, term: &Term<'_>) {
        visit_term(self, term);
    }

    fn visit_identifier(&mut self, _id: &Identifier<'_>) {}

    fn visit_comment(&mut self, _comment: &Comment<'_>) {}

    fn visit_attributes(&mut self, attributes: &[Attribute<'_>]) {
        visit_attributes(self, attributes);
    }

    fn visit_attribute(&mut self, attribute: &Attribute<'_>) {
        visit_attribute(self, attribute);
    }

    fn visit_pattern(&mut self, pattern: &Pattern<'_>) {
        visit_pattern(self, pattern);
    }

    fn visit_pattern_elements(&mut self, elements: &[PatternElement<'_>]) {
        visit_pattern_elements(self, elements);
    }

    fn visit_pattern_element(&mut self, element: &PatternElement<'_>) {
        visit_pattern_element(self, element);
    }

    fn visit_text_element(&mut self, _text: &str) {}

    fn visit_expression(&mut self, expr: &Expression<'_>) {
        visit_expression(self, expr);
    }

    fn visit_inline_expression(&mut self, expr: &InlineExpression<'_>) {
        visit_inline_expression(self, expr);
    }

    fn visit_select(&mut self, selector: &InlineExpression<'_>, variants: &[Variant<'_>]) {
        visit_select(self, selector, variants);
    }

    fn visit_variants(&mut self, variants: &[Variant<'_>]) {
        visit_variants(self, variants);
    }

    fn visit_variant(&mut self, variant: &Variant<'_>) {
        visit_variant(self, variant);
    }

    fn visit_variant_key(&mut self, _key: &VariantKey<'_>) {}

    fn visit_string_literal(&mut self, _literal: &str) {}

    fn visit_number_literal(&mut self, _literal: &str) {}

    fn visit_function_reference(&mut self, id: &Identifier<'_>, args: &Option<CallArguments>) {
        visit_function_reference(self, id, args);
    }

    fn visit_message_reference(&mut self, id: &Identifier<'_>, attribute: &Option<Identifier<'_>>) {
        visit_message_reference(self, id, attribute);
    }

    fn visit_variable_reference(&mut self, id: &Identifier<'_>) {
        visit_variable_reference(self, id);
    }

    fn visit_term_reference(
        &mut self,
        id: &Identifier<'_>,
        attribute: &Option<Identifier<'_>>,
        args: &Option<CallArguments>,
    ) {
        visit_term_reference(self, id, attribute, args);
    }

    fn visit_call_arguments(&mut self, args: &CallArguments<'_>) {
        visit_call_arguments(self, args);
    }

    fn visit_named_argument(&mut self, arg: &NamedArgument<'_>) {
        visit_named_argument(self, arg);
    }
}

pub fn visit_resource<V>(visitor: &mut V, resource: &Resource<'_>)
where
    V: Visitor + ?Sized,
{
    for entry in &resource.body {
        visitor.visit_resource_entry(entry);
    }
}

pub fn visit_resource_entry<V>(visitor: &mut V, entry: &ResourceEntry<'_>)
where
    V: Visitor + ?Sized,
{
    match entry {
        ResourceEntry::Entry(entry) => visitor.visit_entry(entry),
        ResourceEntry::Junk(junk) => visitor.visit_junk(junk),
    }
}

pub fn visit_entry<V>(visitor: &mut V, entry: &Entry<'_>)
where
    V: Visitor + ?Sized,
{
    match entry {
        Entry::Message(msg) => visitor.visit_message(msg),
        Entry::Term(term) => visitor.visit_term(term),
        Entry::Comment(comment) => visitor.visit_comment(comment),
    }
}

fn visit_message<V>(visitor: &mut V, msg: &Message)
where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(&msg.id);
    if let Some(comment) = msg.comment.as_ref() {
        visitor.visit_comment(comment);
    }
    visitor.visit_attributes(&msg.attributes);

    if let Some(pattern) = msg.value.as_ref() {
        visitor.visit_pattern(pattern);
    }
}

fn visit_term<V>(visitor: &mut V, term: &Term)
where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(&term.id);

    if let Some(comment) = term.comment.as_ref() {
        visitor.visit_comment(comment);
    }
    visitor.visit_attributes(&term.attributes);
    visitor.visit_pattern(&term.value);
}

fn visit_attributes<V>(visitor: &mut V, attributes: &[Attribute<'_>])
where
    V: Visitor + ?Sized,
{
    for attr in attributes {
        visitor.visit_attribute(attr);
    }
}

fn visit_attribute<V>(visitor: &mut V, attribute: &Attribute<'_>)
where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(&attribute.id);
    visitor.visit_pattern(&attribute.value);
}

fn visit_pattern<V>(visitor: &mut V, pattern: &Pattern<'_>)
where
    V: Visitor + ?Sized,
{
    visitor.visit_pattern_elements(&pattern.elements);
}

fn visit_pattern_elements<V>(visitor: &mut V, elements: &[PatternElement<'_>])
where
    V: Visitor + ?Sized,
{
    for element in elements {
        visitor.visit_pattern_element(element);
    }
}

fn visit_pattern_element<V>(visitor: &mut V, element: &PatternElement<'_>)
where
    V: Visitor + ?Sized,
{
    match element {
        PatternElement::TextElement(text) => visitor.visit_text_element(text),
        PatternElement::Placeable(expr) => visitor.visit_expression(expr),
    }
}

fn visit_expression<V>(visitor: &mut V, expr: &Expression<'_>)
where
    V: Visitor + ?Sized,
{
    match expr {
        Expression::InlineExpression(inline) => visitor.visit_inline_expression(inline),
        Expression::SelectExpression { selector, variants } => {
            visitor.visit_select(selector, variants)
        }
    }
}

fn visit_inline_expression<V>(visitor: &mut V, expr: &InlineExpression<'_>)
where
    V: Visitor + ?Sized,
{
    match expr {
        InlineExpression::StringLiteral { value } => visitor.visit_string_literal(value),
        InlineExpression::NumberLiteral { value } => visitor.visit_number_literal(value),
        InlineExpression::FunctionReference { id, arguments } => {
            visitor.visit_function_reference(id, arguments)
        }
        InlineExpression::MessageReference { id, attribute } => {
            visitor.visit_message_reference(id, attribute)
        }
        InlineExpression::TermReference {
            id,
            attribute,
            arguments,
        } => visitor.visit_term_reference(id, attribute, arguments),
        InlineExpression::VariableReference { id } => visitor.visit_variable_reference(id),
        InlineExpression::Placeable { expression } => visitor.visit_expression(expression),
    }
}

fn visit_select<V>(visitor: &mut V, selector: &InlineExpression<'_>, variants: &[Variant<'_>])
where
    V: Visitor + ?Sized,
{
    visitor.visit_inline_expression(selector);
    visitor.visit_variants(variants);
}

fn visit_function_reference<V>(
    visitor: &mut V,
    id: &Identifier<'_>,
    arguments: &Option<CallArguments<'_>>,
) where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(id);
    if let Some(args) = arguments.as_ref() {
        visitor.visit_call_arguments(args);
    }
}

fn visit_variable_reference<V>(visitor: &mut V, id: &Identifier<'_>)
where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(id);
}

fn visit_message_reference<V>(
    visitor: &mut V,
    id: &Identifier<'_>,
    attribute: &Option<Identifier<'_>>,
) where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(id);
    if let Some(attr) = attribute.as_ref() {
        visitor.visit_identifier(attr);
    }
}

fn visit_term_reference<V>(
    visitor: &mut V,
    id: &Identifier,
    attribute: &Option<Identifier>,
    args: &Option<CallArguments>,
) where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(id);
    if let Some(attr) = attribute.as_ref() {
        visitor.visit_identifier(attr);
    }
    if let Some(args) = args.as_ref() {
        visitor.visit_call_arguments(args);
    }
}

fn visit_call_arguments<V>(visitor: &mut V, args: &CallArguments<'_>)
where
    V: Visitor + ?Sized,
{
    for positional in &args.positional {
        visitor.visit_inline_expression(positional);
    }

    for named in &args.named {
        visitor.visit_named_argument(named);
    }
}

fn visit_named_argument<V>(visitor: &mut V, arg: &NamedArgument<'_>)
where
    V: Visitor + ?Sized,
{
    visitor.visit_identifier(&arg.name);
    visitor.visit_inline_expression(&arg.value);
}

fn visit_variants<V>(visitor: &mut V, variants: &[Variant<'_>])
where
    V: Visitor + ?Sized,
{
    for variant in variants {
        visitor.visit_variant(variant);
    }
}

fn visit_variant<V>(visitor: &mut V, variant: &Variant<'_>)
where
    V: Visitor + ?Sized,
{
    visitor.visit_variant_key(&variant.key);
    visitor.visit_pattern(&variant.value);
}
