use std::io::{Result, Write};
use v_htmlescape::escape;

use super::resolver::resolve;
use super::types::{ContextTag, Value, KeyTag, Tag, Template, ValueTag};

struct Emitter<'a, W, P>
where
  W: Write,
  P: Fn(&str) -> Option<String>,
{
  writer: W,
  context: Vec<&'a Value>,
  partials: P,
}


fn bool_to_str(flag: bool) -> &'static str {
  if flag {
    "true"
  } else {
    "false"
  }
}

impl<'a, W, P> Emitter<'a, W, P>
where
  W: Write,
  P: Fn(&str) -> Option<String>,
{
  fn new(writer: W, partials: P) -> Emitter<'a, W, P> {
    Emitter {
      writer,
      context: vec![],
      partials,
    }
  }

  fn emit_nothing(&self) -> Result<()> {
    Ok(())
  }

  fn emit_string(&mut self, text: &str) -> Result<()> {
    self.writer.write_all(text.as_bytes())?;
    Ok(())
  }

  fn emit_context(&mut self, tags: &[Tag], value: &'a Value) -> Result<()>
  {
    self.context.push(value);
    let result = self.emit_tags(tags);
    self.context.pop();
    result
  }

  fn emit_tags(&mut self, tags: &[Tag]) -> Result<()> {
    for tag in tags.iter() {
      self.emit_tag(tag)?;
    }
    Ok(())
  }

  fn emit_tag(&mut self, tag: &Tag) -> Result<()> {
    match tag {
      Tag::Text(value) => self.emit_string(&value.value),
      Tag::Variable(key) => self.emit_variable(key),
      Tag::Unescaped(key) => self.emit_unescaped(key),
      Tag::Section(section) => self.emit_section(section),
      Tag::Inverted(section) => self.emit_inverted(section),
      Tag::Partial(value) => self.emit_partial(value),
      _ => self.emit_nothing(),
    }
  }

  fn emit_variable(&mut self, tag: &KeyTag) -> Result<()> {
    match resolve(&self.context, &tag.key) {
      Ok(Value::String(string)) => self.emit_string(&escape(string).to_string()),
      Ok(Value::Number(number)) => self.emit_string(&number.to_string()),
      Ok(Value::Bool(boolean)) => self.emit_string(bool_to_str(*boolean)),
      _ => self.emit_nothing(),
    }
  }

  fn emit_unescaped(&mut self, tag: &KeyTag) -> Result<()> {
    match resolve(&self.context, &tag.key) {
      Ok(Value::String(string)) => self.emit_string(string),
      Ok(Value::Number(number)) => self.emit_string(&number.to_string()),
      Ok(Value::Bool(boolean)) => self.emit_string(bool_to_str(*boolean)),
      _ => self.emit_nothing(),
    }
  }

  fn emit_section(&mut self, tag: &ContextTag) -> Result<()> {
    match resolve(&self.context, &tag.key) {
      Ok(Value::Null) => self.emit_nothing(),
      Ok(Value::Bool(false)) => self.emit_nothing(),
      Ok(Value::Array(vec)) if vec.is_empty() => self.emit_nothing(),
      Ok(Value::Array(vec)) if !vec.is_empty() => {
        for item in vec {
          self.emit_context(&tag.tags, item)?;
        }
        Ok(())
      }
      Ok(value) => self.emit_context(&tag.tags, &value),
      _ => self.emit_nothing(),
    }
  }

  fn emit_inverted(&mut self, tag: &ContextTag) -> Result<()> {
    match resolve(&self.context, &tag.key) {
      Ok(Value::Null) => self.emit_tags(&tag.tags),
      Ok(Value::Bool(false)) => self.emit_tags(&tag.tags),
      Ok(Value::Array(vec)) if vec.is_empty() => self.emit_tags(&tag.tags),
      Ok(_) => self.emit_nothing(),
      _ => self.emit_tags(&tag.tags),
    }
  }

  fn emit_partial(&mut self, tag: &ValueTag) -> Result<()> {
    let key = tag.value.to_string();
    if let Some(text) = (self.partials)(&key) {
      if let Ok((template, _)) = crate::parser::template(&text) {
        return self.emit_tags(&template.tags);
      }
    }

    self.emit_nothing()
  }
}

pub fn emit<W, P>(writer: W, template: &Template, value: &Value, partials: P) -> Result<()>
where
  W: Write,
  P: Fn(&str) -> Option<String>,
{
  Emitter::new(writer, partials).emit_context(&template.tags, value)
}
