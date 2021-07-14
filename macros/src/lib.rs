extern crate proc_macro;

mod macros;

#[proc_macro_attribute]
pub fn test_spec(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let attr2 = proc_macro2::TokenStream::from(attr);
  let item2 = proc_macro2::TokenStream::from(item);
  let output2 = match macros::test_spec(attr2, item2) {
    Ok(result) => result,
    Err(error) => error.to_compile_error(),
  };
  
  proc_macro::TokenStream::from(output2)
}
