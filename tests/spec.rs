extern crate mustache_core;

#[cfg(test)]
mod spec {
  use macros::test_spec;
  use mustache_core::{render, Value};
  use serde::Deserialize;
  use std::collections::HashMap;

  #[derive(Deserialize)]
  struct MustacheInput {
    desc: String,
    data: Value,
    template: String,
    partials: Option<HashMap<String, String>>,
    expected: String,
  }

  #[test_spec(
    "spec/comments.yml",
    "spec/delimiters.yml",
    "spec/interpolation.yml",
    "spec/inverted.yml",
    "spec/partials.yml",
    "spec/sections.yml"
  )]
  fn base_test(input: MustacheInput) {
    let partials = input.partials.unwrap_or_default();
    match render(&input.template, &input.data, |key| {
      partials.get(key).cloned()
    }) {
      Ok(actual) => assert_eq!(actual, input.expected, "Spec Panic: {}", &input.desc),
      Err(err) => panic!("Render Panic: {}", err),
    };
  }
}
