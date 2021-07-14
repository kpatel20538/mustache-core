pub type Key<'a> = Vec<&'a str>;
pub type KeySlice<'a> = [&'a str];
pub type Value = serde_json::Value;

pub struct ContextTag<'a> {
  pub key: Key<'a>,
  pub tags: Vec<Tag<'a>>,
}

pub struct KeyTag<'a> {
  pub key: Key<'a>,
}

pub struct ValueTag<'a> {
  pub value: &'a str,
}

pub struct DelimiterTag<'a> {
  pub start: &'a str,
  pub stop: &'a str,
}

pub enum Tag<'a> {
  Text(ValueTag<'a>),
  Variable(KeyTag<'a>),
  Unescaped(KeyTag<'a>),
  Inverted(ContextTag<'a>),
  Section(ContextTag<'a>),
  Comment(ValueTag<'a>),
  Partial(ValueTag<'a>),
  Delimiters(DelimiterTag<'a>),
}

pub struct Template<'a> {
  pub tags: Vec<Tag<'a>>,
}

pub type Result<T> = std::result::Result<T, String>;