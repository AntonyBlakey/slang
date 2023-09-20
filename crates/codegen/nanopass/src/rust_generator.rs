use infra_utils::commands::Command;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::type_model::*;

impl TypeModel {
    pub fn generate_rust(&self) -> Result<String, anyhow::Error> {
        let declarations = self.0.iter().map(|(name, ty)| ty.generate_rust(&name));
        let contents = quote! { #(#declarations)* }.to_string();
        Command::new("rustfmt")
            .property("--emit", "stdout")
            .evaluate_with_input(contents)
    }
}

impl NamedType {
    fn generate_rust(&self, name: &str) -> TokenStream {
        match self {
            Self::Struct(fields) => {
                let name = format_ident!("{}", name);
                let fields = fields.iter().map(|(name, ty)| {
                    let ty = ty.generate_rust();
                    let name = format_ident!("{}", name);
                    quote! { #name: #ty }
                });
                quote! { struct #name { #(#fields),* } }
            }
            Self::Enum(fields) => {
                let name = format_ident!("{}", name);
                let fields = fields.iter().map(|(name, ty)| {
                    let name = format_ident!("{}", name);
                    if *ty == AnonymousType::Void {
                        quote! { #name }
                    } else {
                        let ty = ty.generate_rust();
                        quote! { #name(#ty) }
                    }
                });
                quote! { enum #name { #(#fields),* } }
            }
            Self::Alias(ty) => {
                let name = format_ident!("{}", name);
                let ty = ty.generate_rust();
                quote! { type #name = #ty; }
            }
        }
    }
}

impl AnonymousType {
    fn generate_rust(&self) -> TokenStream {
        match self {
            Self::Option(ty) => {
                let ty = ty.generate_rust();
                quote! { Option<#ty> }
            }
            Self::Vec(ty) => {
                let ty = ty.generate_rust();
                quote! { Vec<#ty> }
            }
            Self::Map(types) => {
                let (key_type, value_type) = types.as_ref();
                let key_type = key_type.generate_rust();
                let value_type = value_type.generate_rust();
                quote! { HashMap<#key_type, #value_type> }
            }
            Self::Set(key_type) => {
                let key_type = key_type.generate_rust();
                quote! { HashSet<#key_type> }
            }
            Self::Reference(type_name) | Self::External(type_name) => {
                let type_name = format_ident!("{}", type_name);
                quote! { #type_name }
            }
            Self::Void => quote! { () },
            Self::Bool => quote! { bool },
            Self::Int => quote! { u32 },
            Self::String => quote! { String },
        }
    }
}

#[cfg(test)]
use {indoc::indoc, std::collections::BTreeMap, std::rc::Rc};

#[cfg(test)]
fn example_type_model() -> TypeModel {
    let mut type_model = TypeModel::default();

    type_model.upsert_type(
        "MyStruct".to_string(),
        NamedType::Struct(BTreeMap::from([
            ("bar".to_string(), AnonymousType::Int),
            (
                "baz".to_string(),
                AnonymousType::Vec(Rc::new(AnonymousType::String)),
            ),
        ])),
    );

    type_model.upsert_type(
        "MyEnum".to_string(),
        NamedType::Enum(BTreeMap::from([
            ("bar".to_string(), AnonymousType::Bool),
            ("baz".to_string(), AnonymousType::Void),
        ])),
    );

    type_model
}

#[test]
fn baseline_generate_rust() -> Result<(), anyhow::Error> {
    let type_model = example_type_model();
    let rust = type_model.generate_rust().unwrap();
    let expected = indoc! { "
        enum MyEnum {
            bar(bool),
            baz,
        }
        struct MyStruct {
            bar: u32,
            baz: Vec<String>,
        }
    "};
    assert_eq!(rust, expected);

    Ok(())
}

#[test]
fn transform_model() -> Result<(), anyhow::Error> {
    let mut type_model = example_type_model();

    type_model.upsert_field("MyStruct", "foo", AnonymousType::Bool)?;
    type_model.remove_field("MyStruct", "bar")?;
    type_model.upsert_field("MyEnum", "bar", AnonymousType::Void)?;
    type_model.upsert_field("MyEnum", "foo", AnonymousType::Bool)?;

    let rust = type_model.generate_rust().unwrap();
    let expected = indoc! { "
        enum MyEnum {
            bar,
            baz,
            foo(bool),
        }
        struct MyStruct {
            baz: Vec<String>,
            foo: bool,
        }
    "};
    assert_eq!(rust, expected);

    Ok(())
}
