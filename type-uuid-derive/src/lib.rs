extern crate proc_macro;

use quote::quote;
use syn::parse::*;
use syn::*;
use uuid::Uuid;

#[proc_macro_derive(TypeUuid, attributes(uuid))]
pub fn type_uuid_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let name = &ast.ident;

    let mut uuid = None;
    for attribute in ast.attrs.iter().filter_map(|attr| attr.parse_meta().ok()) {
        let name_value = if let Meta::NameValue(name_value) = attribute {
            name_value
        } else {
            continue;
        };

        if name_value
            .path
            .get_ident()
            .map(|i| i != "uuid")
            .unwrap_or(true)
        {
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
        .map(|byte_str| syn::parse_str::<LitInt>(&byte_str).unwrap());

    let gen = quote! {
        impl type_uuid::TypeUuid for #name {
            const UUID: type_uuid::Bytes = [
                #( #bytes ),*
            ];
        }
    };
    gen.into()
}

struct ExternalDeriveInput {
    path: ExprPath,
    uuid_str: LitStr,
}

impl Parse for ExternalDeriveInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = input.parse()?;
        input.parse::<Token![,]>()?;
        let uuid_str = input.parse()?;
        Ok(Self { path, uuid_str })
    }
}

#[proc_macro]
pub fn external_type_uuid(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ExternalDeriveInput { path, uuid_str } = parse_macro_input!(tokens as ExternalDeriveInput);

    let uuid = Uuid::parse_str(&uuid_str.value()).expect("Value was not a valid UUID");

    let bytes = uuid
        .as_bytes()
        .iter()
        .map(|byte| format!("{:#X}", byte))
        .map(|byte_str| syn::parse_str::<LitInt>(&byte_str).unwrap());

    let gen = quote! {
        impl crate::TypeUuid for #path {
            const UUID: crate::Bytes = [
                #( #bytes ),*
            ];
        }
    };
    gen.into()
}
