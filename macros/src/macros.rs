use heck::SnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Result};

#[derive(Deserialize)]
struct SpecSuite {
  overview: String,
  tests: Vec<Value>,
}

#[derive(Deserialize)]
struct SpecCase {
  name: String,
  desc: String,
}

struct TestBasis<'a> {
  func_ident: &'a syn::Ident,
  input_type: &'a syn::Type,
}

struct TestFnInput {
  input_value: syn::LitStr,
  test_name: syn::Ident,
  test_doc: String,
}

struct TestModInput {
  test_fn_inputs: Vec<TestFnInput>,
  mod_name: syn::Ident,
  mod_doc: String,
}

struct TestSpecArgs {
  file_names: Punctuated<syn::LitStr, syn::Token![,]>,
}

impl Parse for TestSpecArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(TestSpecArgs {
      file_names: Punctuated::parse_terminated(input)?,
    })
  }
}

fn get_first_input_type(item: &syn::ItemFn) -> Result<&syn::Type> {
  let inputs = &item.sig.inputs;
  match inputs.first() {
    Some(syn::FnArg::Typed(arg)) => Ok(&arg.ty),
    Some(reciever) => Err(Error::new(reciever.span(), "Non method function required")),
    None => Err(Error::new(inputs.span(), "Requires at least one argument")),
  }
}

fn new_test_basis(base_fn: &syn::ItemFn) -> Result<TestBasis<'_>> {
  let func_ident = &base_fn.sig.ident;
  let input_type = get_first_input_type(&base_fn)?;

  Ok(TestBasis {
    func_ident,
    input_type,
  })
}

fn snake_case_ident(name: &str) -> syn::Ident {
  format_ident!("{}", name.to_snake_case())
}

fn path_to_file_stem(file_name: &str, span: &Span) -> Result<syn::Ident> {
  Path::new(file_name)
    .file_stem()
    .and_then(|s| s.to_str())
    .map(snake_case_ident)
    .ok_or_else(|| Error::new(*span, "Unable to procress file name"))
}

fn read_to_string(file_name: &str, span: &Span) -> Result<String> {
  fs::read_to_string(file_name).map_err(|err| Error::new(*span, err))
}

fn spec_suite_from_string(file_data: &str, span: &Span) -> Result<SpecSuite> {
  serde_yaml::from_str::<SpecSuite>(&file_data).map_err(|err| Error::new(*span, err))
}

fn spec_case_from_value(value: Value, span: &Span) -> Result<SpecCase> {
  serde_yaml::from_value(value).map_err(|err| Error::new(*span, err))
}

fn value_to_lit_str(value: &Value, span: &Span) -> Result<syn::LitStr> {
  let data = serde_yaml::to_string(&value).map_err(|err| Error::new(*span, err))?;
  Ok(syn::LitStr::new(&data, *span))
}


fn new_test_fn_input(value: Value, span: &Span) -> Result<TestFnInput> {
  let input_value = value_to_lit_str(&value, span)?;
  let spec_case = spec_case_from_value(value, span)?;
  let test_name = snake_case_ident(&spec_case.name);
  let test_doc = spec_case.desc;

  Ok(TestFnInput {
    input_value,
    test_name,
    test_doc,
  })
}

fn new_test_mod_input(file_name: &syn::LitStr) -> Result<TestModInput> {
  let span = file_name.span();
  let file_name_value = file_name.value();
  let mod_name = path_to_file_stem(&file_name_value, &span)?;
  let file_data = read_to_string(&file_name_value, &span)?;
  let spec_suite = spec_suite_from_string(&file_data, &span)?;

  let mut test_fn_inputs: Vec<TestFnInput> = vec![];
  for value in spec_suite.tests {
    test_fn_inputs.push(new_test_fn_input(value, &span)?);
  }

  Ok(TestModInput {
    test_fn_inputs,
    mod_name,
    mod_doc: spec_suite.overview,
  })
}

fn new_test_spec(test_spec_args: &TestSpecArgs) -> Result<Vec<TestModInput>> {
  let mut test_mod_inputs: Vec<TestModInput> = vec![];
  for file_name in test_spec_args.file_names.iter() {
    test_mod_inputs.push(new_test_mod_input(&file_name)?)
  }
  Ok(test_mod_inputs)
}

fn quote_test_fn(test_basis: &TestBasis, test_fn_input: &TestFnInput) -> TokenStream {
  let TestBasis {
    func_ident,
    input_type,
  } = test_basis;
  let TestFnInput {
    input_value,
    test_name,
    test_doc,
  } = test_fn_input;

  quote! {
    #[doc = #test_doc]
    #[test]
    fn #test_name() {
      use serde_yaml;
      let _input = serde_yaml::from_str::<#input_type>(#input_value).unwrap();
      #func_ident(_input);

    }
  }
}

fn quote_test_mod(test_basis: &TestBasis, test_mod_input: &TestModInput) -> TokenStream {
  let TestModInput {
    test_fn_inputs,
    mod_name,
    mod_doc,
  } = test_mod_input;
  
  let test_fns = test_fn_inputs
    .iter()
    .map(|test_fn_input| quote_test_fn(&test_basis, &test_fn_input));

  quote! {
    #[doc = #mod_doc]
    #[cfg(test)]
    mod #mod_name {
      use super::*;

      #(#test_fns)*
    }
  }
}

pub fn test_spec(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
  let test_spec_args: TestSpecArgs = syn::parse2(attr)?;
  let base_fn: syn::ItemFn = syn::parse2(item)?;

  let test_basis = new_test_basis(&base_fn)?;
  let test_spec = new_test_spec(&test_spec_args)?;

  let test_mods = test_spec
    .iter()
    .map(|spec| quote_test_mod(&test_basis, &spec));

  Ok(quote! {
    #base_fn

    #(#test_mods)*
  })
}
