use super::core;
use crate::types::{Result, Key};

fn dot(text: &str) -> Result<((), &str)> {
  let (_, text0) = core::string(text, ".")?;
  Ok(((), text0))
}

fn is_identifier_start(c: char) -> bool {
  char::is_alphabetic(c) | (c == '$') | (c == '_')
}

fn is_identifier_char(c: char) -> bool {
  char::is_alphanumeric(c) | (c == '$') | (c == '_')
}

fn identifier(text: &str) -> Result<(&str, &str)> {
  if !text.starts_with(is_identifier_start) {
    return Err("Leading character for identifier not found".to_string());
  }

  core::many_chars(text, is_identifier_char)
}

fn implicit(text: &str) -> Result<(Key<'_>, &str)> {
  let (_, text0) = core::string(text, ".")?;
  Ok((vec![], text0))
}

pub fn key(text: &str) -> Result<(Key<'_>, &str)> {
  core::sep_by(text, identifier, dot)
    .or_else(|_| implicit(text))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn key_base() -> Result<()> {
    let (path, text) = key("alpha")?;

    assert_eq!("", text);
    assert_eq!(vec!["alpha"], path);
    Ok(())
  }

  #[test]
  fn key_dot() -> Result<()> {
    let (path, text) = key(".")?;
    
    assert_eq!("", text);
    assert!(path.is_empty());
    Ok(())
  }

  #[test]
  fn key_empty() {
    assert!(key("").is_err())
  }

  #[test]
  fn key_single_dot() -> Result<()> {
    let (path, text) = key("alpha.beta")?;

    assert_eq!("", text);
    assert_eq!(vec!["alpha", "beta"], path);
    Ok(())
  }

  #[test]
  fn key_multiple_dot() -> Result<()> {
    let (path, text) = key("alpha.beta.gamma")?;

    assert_eq!("", text);
    assert_eq!(vec!["alpha", "beta", "gamma"], path);
    Ok(())
  }
}
