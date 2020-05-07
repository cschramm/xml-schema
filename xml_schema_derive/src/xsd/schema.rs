use crate::xsd::{complex_type, element, import, qualification, simple_type, XsdContext};
use log::{debug, info};
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root="schema"
  prefix="xs",
  namespace="xs: http://www.w3.org/2001/XMLSchema",
)]
pub struct Schema {
  #[yaserde(rename = "targetNamespace", attribute)]
  pub target_namespace: Option<String>,
  #[yaserde(rename = "elementFormDefault", attribute)]
  pub element_form_default: qualification::Qualification,
  #[yaserde(rename = "attributeFormDefault", attribute)]
  pub attribute_form_default: qualification::Qualification,
  #[yaserde(rename = "import")]
  pub imports: Vec<import::Import>,
  #[yaserde(rename = "element")]
  pub elements: Vec<element::Element>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Vec<simple_type::SimpleType>,
  #[yaserde(rename = "complexType")]
  pub complex_type: Vec<complex_type::ComplexType>,
}

impl Schema {
  pub fn get_implementation(
    &self,
    target_prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let namespace_definition =
      match (target_prefix, &self.target_namespace) {
        (None, None) => quote!(),
        (None, Some(_target_namespace)) => panic!("undefined prefix attribute, a target namespace is defined"),
        (Some(_prefix), None) => panic!("a prefix attribute, but no target namespace is defined, please remove the prefix parameter"),
        (Some(prefix), Some(target_namespace)) => {
          let namespace = format!("{}: {}", prefix, target_namespace);
          quote!(#[yaserde(prefix=#prefix, namespace=#namespace)])
        }
      };

    info!("Generate elements");
    let elements: TokenStream = self
      .elements
      .iter()
      .map(|element| element.get_implementation(&namespace_definition, target_prefix, context))
      .collect();

    info!("Generate simple types");
    let simple_types: TokenStream = self
      .simple_type
      .iter()
      .map(|simple_type| {
        simple_type.get_implementation(&namespace_definition, target_prefix, context)
      })
      .collect();

    info!("Generate complex types");
    let complex_types: TokenStream = self
      .complex_type
      .iter()
      .map(|complex_type| {
        complex_type.get_implementation(&namespace_definition, target_prefix, context)
      })
      .collect();

    quote!(
      #simple_types
      #complex_types
      #elements
    )
  }
}