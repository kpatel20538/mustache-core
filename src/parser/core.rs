use crate::types::Result;

pub fn take(text: &str, n: usize) -> Result<(&str, &str)> {
  Ok((&text[0..n], &text[n..]))
}

pub fn string<'a, 'b>(text: &'a str, value: &'b str) -> Result<(&'a str, &'a str)> {
  if text.starts_with(value) {
    take(text, value.len())
  } else {
    Err(format!("'{}' string not found", value))
  }
}

pub fn many_chars<P>(text: &str, pred: P) -> Result<(&str, &str)>
where
  P: Fn(char) -> bool,
{
  for (i, c) in text.char_indices() {
    if !pred(c) {
      return take(text, i);
    }
  }
  Ok((text, ""))
}

pub fn take_until<P>(text: &str, pred: P) -> Result<(&str, &str)>
where
  P: Fn(&str) -> bool,
{
  for (i, _) in text.char_indices() {
    if pred(&text[i..]) {
      return take(text, i);
    }
  }
  Ok((text, ""))
}

pub fn some_chars<P>(text: &str, pred: P) -> Result<(&str, &str)>
where
  P: Fn(char) -> bool,
{
  let (value, text) = many_chars(text, pred)?;
  if value.is_empty() {
    Err("No chars found with the predicate".to_string())
  } else {
    Ok((value, text))
  }
}

pub fn sep_by<'a, P, Q, A, B>(text: &'a str, content: P, separator: Q) -> Result<(Vec<A>, &'a str)>
where
  P: Fn(&'a str) -> Result<(A, &'a str)>,
  Q: Fn(&'a str) -> Result<(B, &'a str)>,
{
  if let Ok((lead, text0)) = content(text) {
    let mut items = vec![lead];
    let mut needle = text0;
    while let Ok((_, needle0)) = separator(needle) {
      if let Ok((value, needle1)) = content(needle0) {
        items.push(value);
        needle = needle1;
      } else {
        break;
      }
    }
    return Ok((items, needle));
  }
  Err("Seperated content not found ".to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn take_base() -> Result<()> {
    let (prefix, text) = take("alpha", 3)?;

    assert_eq!("alp", prefix);
    assert_eq!("ha", text);
    Ok(())
  }

  #[test]
  fn string_base() -> Result<()> {
    let (prefix, text) = string("beta", "bet")?;

    assert_eq!("a", text);
    assert_eq!("bet", prefix);
    Ok(())
  }

  #[test]
  fn many_chars_base() -> Result<()> {
    let (prefix, text) = many_chars("123alpha", char::is_numeric)?;

    assert_eq!("alpha", text);
    assert_eq!("123", prefix);
    Ok(())
  }

  #[test]
  fn many_chars_empty() -> Result<()> {
    let (prefix, text) = many_chars("123alpha", char::is_alphabetic)?;

    assert_eq!("123alpha", text);
    assert_eq!("", prefix);
    Ok(())
  }

  #[test]
  fn take_until_base() -> Result<()> {
    let (prefix, text) = take_until("abbaabb", |t| t.starts_with("aa"))?;

    assert_eq!("aabb", text);
    assert_eq!("abb", prefix);
    Ok(())
  }

  #[test]
  fn take_until_empty() -> Result<()> {
    let (prefix, text) =
      take_until("aabbaabb", |t| t.starts_with("aa"))?;

    assert_eq!("aabbaabb", text);
    assert_eq!("", prefix);
    Ok(())
  }

  #[test]
  fn some_chars_base() -> Result<()> {
    let (prefix, text) = some_chars("123alpha", char::is_numeric)?;

    assert_eq!("alpha", text);
    assert_eq!("123", prefix);
    Ok(())
  }

  #[test]
  fn some_chars_empty() {
    assert!(some_chars("123alpha", char::is_alphabetic).is_err())
  }

  #[test]
  fn sep_by_base() -> Result<()> {
    let (values, text) = sep_by("a,a,a.", |text| string(text, "a"), |text| string(text, ","))?;

    assert_eq!(".", text);
    assert_eq!(vec!["a", "a", "a"], values);
    Ok(())
  }

  #[test]
  fn sep_by_empty() {
    assert!(sep_by(".", |text| string(text, "a"), |text| string(text, ","),).is_err())
  }

  #[test]
  fn sep_by_one() -> Result<()> {
    let (values, text) =
      sep_by("a.", |text| string(text, "a"), |text| string(text, ","))?;

    assert_eq!(".", text);
    assert_eq!(vec!["a"], values);
    Ok(())
  }

  #[test]
  fn sep_by_trailing() -> Result<()> {
    let (values, text) = sep_by("a,a,.", |text| string(text, "a"), |text| string(text, ","))?;

    assert_eq!(",.", text);
    assert_eq!(vec!["a", "a"], values);
    Ok(())
  }
}
