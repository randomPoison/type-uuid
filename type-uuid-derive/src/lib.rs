extern crate proc_macro;

use quote::quote;
use syn::parse::*;
use syn::*;
use uuid::Uuid;

/// UUID namespace for type-uuid
#[cfg(feature = "autogen")]
const UUID_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0,
    0x4f, 0xd4, 0x30, 0xc8,
]);

#[proc_macro_derive(TypeUuid, attributes(uuid, auto_uuid))]
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
            _ => panic!("uuid attribute must take the form `#[uuid = \"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"]`"),
        };

        uuid = Some(
            Uuid::parse_str(&uuid_str.value())
                .expect("Value specified to `#[uuid]` attribute is not a valid UUID"),
        );
    }

    #[cfg(feature = "autogen")]
    let uuid =
        uuid.unwrap_or_else(|| uuid::Uuid::new_v5(
            &UUID_NAMESPACE,
            name.to_string().as_bytes(),
        ));
    #[cfg(not(feature = "autogen"))]
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
