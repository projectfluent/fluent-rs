use fluent_bundle::FluentArgs;
use std::borrow::Cow;

pub struct L10nKey<'l> {
    pub id: Cow<'l, str>,
    pub args: Option<FluentArgs<'l>>,
}

#[derive(Debug)]
pub struct L10nAttribute<'l> {
    pub name: Cow<'l, str>,
    pub value: Cow<'l, str>,
}

#[derive(Debug)]
pub struct L10nMessage<'l> {
    pub value: Option<Cow<'l, str>>,
    pub attributes: Vec<L10nAttribute<'l>>,
}
