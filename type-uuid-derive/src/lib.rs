use quote::{quote, ToTokens};
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

    let gen = if ast.generics.params.is_empty() {
        quote! {
            impl type_uuid::TypeUuid for #name {
                const UUID: type_uuid::Bytes = [
                    #( #bytes ),*
                ];
            }
        }
    } else {
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        let generic_idents = ast.generics.type_params().map(|param| &param.ident);
        let body = impl_generic(bytes, generic_idents);
        quote! {
            impl #impl_generics type_uuid::TypeUuid for #name #ty_generics #where_clause {
                #body
            }
        }
    };
    gen.into()
}

struct ExternalDeriveInput {
    path: Path,
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

    let ty_segment = path.segments.last().expect("Invalid type path");
    let gen = match &ty_segment.arguments {
        PathArguments::None => quote! {
            impl crate::TypeUuid for #path {
                const UUID: crate::Bytes = [
                    #( #bytes ),*
                ];
            }
        },

        PathArguments::AngleBracketed(args) => {
            let body = impl_generic(bytes, args.args.iter());
            let args = args.args.iter().map(|arg| quote! { #arg: crate::TypeUuid });
            quote! {
                impl< #( #args, )* > crate::TypeUuid for #path {
                    #body
                }
            }
        }

        PathArguments::Parenthesized(_) => panic!("Parenthesized args are unsupported"),
    };

    gen.into()
}

fn impl_generic<I: ToTokens>(
    bytes: impl Iterator<Item = LitInt>,
    generic_idents: impl Iterator<Item = I>,
) -> proc_macro2::TokenStream {
    quote! {
        const UUID: type_uuid::Bytes = {
            // Generate the initial buffer based on the base UUID for the type.
            let buffer = type_uuid::const_sha1::ConstBuffer::from_slice(&[
                #( #bytes ),*
            ]);

            // Append the UUID for each type parameter in order.
            #(
                let buffer = buffer.push_slice(&<#generic_idents as type_uuid::TypeUuid>::UUID);
            )*

            // Generate the digest and spit out the first 16 bytes as the UUID.
            let digest = type_uuid::const_sha1::sha1(&buffer).bytes();
            [
                digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
                digest[8], digest[9], digest[10], digest[11], digest[12], digest[13], digest[14],
                digest[15],
            ]
        };
    }
}
