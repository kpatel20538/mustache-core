use super::core;
use crate::types::{ContextTag, DelimiterTag, Key, KeyTag, Result, Tag, Template, ValueTag};

enum Action<'a> {
  PushTag { tag: Tag<'a> },
  PushInvertedContext { key: Key<'a> },
  PushSectionContext { key: Key<'a> },
  PopContext { key: Key<'a> },
  ChangeDelimiter { start: &'a str, stop: &'a str },
}

enum TagKind {
  Text,
  Variable,
  UnescapedWrapped,
  Unescaped,
  Delimiters,
  Inverted,
  Section,
  Comment,
  Partial,
  SectionEnd,
}

fn start_of_tag<'a>(text: &'a str, start: &'a str) -> Result<(TagKind, &'a str)> {
  if let Ok((_, text0)) = core::string(text, start) {
    let (kind, text1) = core::take(text0, 1)?;
    let token_kind = match kind {
      "!" => TagKind::Comment,
      ">" => TagKind::Partial,
      "{" => TagKind::UnescapedWrapped,
      "&" => TagKind::Unescaped,
      "=" => TagKind::Delimiters,
      "^" => TagKind::Inverted,
      "#" => TagKind::Section,
      "/" => TagKind::SectionEnd,
      _ => {
        return Ok((TagKind::Variable, text0));
      }
    };
    Ok((token_kind, text1))
  } else {
    Ok((TagKind::Text, text))
  }
}

fn end_of_tag<'a>(text: &'a str, stop: &'a str) -> Result<(&'a str, &'a str)> {
  core::string(text, stop)
}

fn value_tag<'a>(text: &'a str, stop: &'a str) -> Result<(&'a str, &'a str)> {
  let (value, text1) = core::take_until(text, |text0| text0.starts_with(stop))?;
  Ok((value.trim(), text1))
}

fn key_tag(text: &str) -> Result<(Key<'_>, &str)> {
  let (_, text0) = core::many_chars(text, char::is_whitespace)?;
  let (key, text1) = super::key::key(text0)?;
  let (_, text2) = core::many_chars(text1, char::is_whitespace)?;
  Ok((key, text2))
}

fn is_delimiter_char(c: char) -> bool {
  !char::is_whitespace(c) && c != '='
}

fn delimiter_tag(text: &str) -> Result<((&str, &str), &str)> {
  let (_, text0) = core::many_chars(text, char::is_whitespace)?;
  let (start, text1) = core::some_chars(text0, is_delimiter_char)?;
  let (_, text2) = core::many_chars(text1, char::is_whitespace)?;
  let (stop, text3) = core::some_chars(text2, is_delimiter_char)?;
  let (_, text4) = core::many_chars(text3, char::is_whitespace)?;
  Ok(((start, stop), text4))
}

fn text_tag<'a>(text: &'a str, stop: &'a str) -> Result<(&'a str, &'a str)> {
  core::take_until(text, |t| t.starts_with(stop))
}

fn mustache_tag<'a>(text: &'a str, start: &'a str, stop: &'a str) -> Result<(Action<'a>, &'a str)> {
  let (kind, text0) = start_of_tag(text, start)?;
  let (action, text1) = match kind {
    TagKind::Text => {
      let (value, text1) = text_tag(text0, start)?;
      let tag = Tag::Text(ValueTag { value });
      return Ok((Action::PushTag { tag }, text1));
    }
    TagKind::Delimiters => {
      let ((start, stop), text0) = delimiter_tag(text0)?;
      let (_, text1) = core::string(text0, "=")?;
      (Action::ChangeDelimiter { start, stop }, text1)
    }
    TagKind::Inverted => {
      let (key, text1) = key_tag(text0)?;
      (Action::PushInvertedContext { key }, text1)
    }
    TagKind::Section => {
      let (key, text1) = key_tag(text0)?;
      (Action::PushSectionContext { key }, text1)
    }
    TagKind::SectionEnd => {
      let (key, text1) = key_tag(text0)?;
      (Action::PopContext { key }, text1)
    }
    TagKind::UnescapedWrapped => {
      let (key, text1) = key_tag(text0)?;
      let (_, text2) = core::string(text1, "}")?;
      let tag = Tag::Unescaped(KeyTag { key });
      (Action::PushTag { tag }, text2)
    }
    TagKind::Unescaped => {
      let (key, text1) = key_tag(text0)?;
      let tag = Tag::Unescaped(KeyTag { key });
      (Action::PushTag { tag }, text1)
    }
    TagKind::Variable => {
      let (key, text1) = key_tag(text0)?;
      let tag = Tag::Variable(KeyTag { key });
      (Action::PushTag { tag }, text1)
    }
    TagKind::Comment => {
      let (value, text2) = value_tag(text0, stop)?;
      let tag = Tag::Comment(ValueTag { value });
      (Action::PushTag { tag }, text2)
    }
    TagKind::Partial => {
      let (value, text1) = value_tag(text0, stop)?;
      let tag = Tag::Partial(ValueTag { value });
      (Action::PushTag { tag }, text1)
    }
  };
  let (_, text2) = end_of_tag(text1, stop)?;
  Ok((action, text2))
}

pub fn template<'a>(text: &'a str) -> Result<(Template<'a>, &'a str)> {
  enum ContextKind {
    Inverted,
    Section,
  }

  struct Context<'a> {
    kind: ContextKind,
    key: Vec<&'a str>,
    tags: Vec<Tag<'a>>,
  }

  let mut stack: Vec<Context<'a>> = vec![];
  let mut context: Context<'a> = Context {
    kind: ContextKind::Section,
    key: vec![],
    tags: vec![],
  };
  let mut start = "{{";
  let mut stop = "}}";
  let mut needle = text;

  while !needle.is_empty() {
    let (action, text0) = mustache_tag(needle, start, stop)?;
    needle = text0;
    match action {
      Action::PushTag { tag } => {
        context.tags.push(tag);
      }
      Action::PushInvertedContext { key } => {
        stack.push(context);
        context = Context {
          kind: ContextKind::Inverted,
          key,
          tags: vec![],
        };
      }
      Action::PushSectionContext { key } => {
        stack.push(context);
        context = Context {
          kind: ContextKind::Section,
          key,
          tags: vec![],
        };
      }
      Action::PopContext { key } => {
        if context.key != key {
          return Err(format!(
            "Key mismatch for section: lead={} trail={}",
            context.key.join("."),
            key.join(".")
          ));
        }

        let mut parent_context = stack
          .pop()
          .ok_or_else(|| format!("No parent context found: {}", key.join(".")))?;

        match context.kind {
          ContextKind::Section => parent_context.tags.push(Tag::Section(ContextTag {
            key: context.key,
            tags: context.tags,
          })),
          ContextKind::Inverted => parent_context.tags.push(Tag::Inverted(ContextTag {
            key: context.key,
            tags: context.tags,
          })),
        }
        context = parent_context;
      }
      Action::ChangeDelimiter {
        start: left,
        stop: right,
      } => {
        start = left;
        stop = right;
        context.tags.push(Tag::Delimiters(DelimiterTag {
          start: left,
          stop: right,
        }));
      }
    }
  }
  Ok((Template { tags: context.tags }, needle))
}
