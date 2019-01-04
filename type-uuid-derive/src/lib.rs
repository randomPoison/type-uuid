extern crate proc_macro;

use proc_macro::*;
use quote::quote;
use syn::*;
use uuid::Uuid;

#[proc_macro_derive(TypeUuid, attributes(uuid))]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut uuid = None;
    for attribute in ast.attrs.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let name_value = if let Meta::NameValue(name_value) = attribute {
            name_value
        } else {
            continue;
        };

        if name_value.ident != "uuid" {
            continue;
        }

        let uuid_str = match name_value.lit {
            Lit::Str(lit_str) => lit_str,
            _ => panic!("uuid attribute must take the form `#[uuid = \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"`"),
        };

        uuid = Some(
            Uuid::parse_str(&uuid_str.value())
                .expect("Value specified to `#[uuid]` attribute is not a valid UUID"),
        );
    }

    let uuid =
        uuid.expect("No `#[uuid = \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"` attribute found");
    let bytes = uuid
        .as_bytes()
        .iter()
        .map(|byte| format!("{:#X}", byte))
        .map(|byte_str| syn::parse_str::<proc_macro2::Literal>(&byte_str).unwrap());

    let gen = quote! {
        impl type_uuid::TypeUuid for #name {
            const UUID: type_uuid::Bytes = [
                #( #bytes ),*
            ];
        }
    };
    gen.into()
}
