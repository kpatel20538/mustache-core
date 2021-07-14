use super::types::{Value, KeySlice, Result};

fn resolve_key<'a>(
  context: &'a Value,
  key: &KeySlice<'_>,
) -> Result<&'a Value> {
  let mut root = context;
  for prop in key {
    root = root
      .get(prop)
      .ok_or_else(|| format!("context miss for key '{}' ", key.join(".")))?;
  }
  Ok(root)
}

fn resolve_hit<'a>(
  context: &'a Value,
  key: &KeySlice<'_>,
) -> bool {
  match key.get(0) {
    Some(lead) => context.get(lead).is_some(),
    None => true
  }
}

pub fn resolve<'a>(
  context_stack: &[&'a Value],
  key: &KeySlice<'_>,
) -> Result<&'a Value> {
  for context in context_stack.iter().rev() {
    if resolve_hit(context, key) {
      return resolve_key(context, key);
    }
  }

  Err(format!("resolve miss for key '{}' ", key.join(".")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;
  use crate::types::Key;


  #[test]
  fn resolve_key_base() -> Result<()> {
    let context = json!({
      "alpha": {
        "beta": "gamma"
      }
    });
    let key = vec!["alpha", "beta"];

    let value = resolve_key(&context, &key)?;
    assert_eq!(value, &json!("gamma"));
    Ok(())
  }

  #[test]
  fn resolve_key_single() -> Result<()> {
    let context = json!({ "test": 4 });
    let key = vec!["test"];

    let value = resolve_key(&context, &key)?;

    assert_eq!(value, &json!(4));
    Ok(())
  }

  #[test]
  fn resolve_key_empty() -> Result<()> {
    let context = json!({ "test": 4 });
    let key: Key = vec![];

    let value = resolve_key(&context, &key)?;

    assert_eq!(value, &json!({ "test": 4 }));
    Ok(())
  }

  #[test]
  fn resolve_key_miss() {
    let context = json!({ "test": 4 });
    let key: Key = vec!["alpha"];

    assert!(resolve_key(&context, &key).is_err());
  }
}
