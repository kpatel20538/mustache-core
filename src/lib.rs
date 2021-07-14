mod emitter;
mod parser;
mod resolver;
mod types;

pub use types::*;

pub fn render<P>(text: &str, context: &Value, partials: P) -> Result<String>
where
  P: Fn(&str) -> Option<String>,
{
  let (template, _) = parser::template(text)?;
  let mut bytes: Vec<u8> = vec![];
  emitter::emit(&mut bytes, &template, &context, partials).map_err(|err| format!("{}", err))?;
  String::from_utf8(bytes).map_err(|err| format!("{}", err))
}
