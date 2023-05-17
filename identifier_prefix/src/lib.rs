use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn identifier_prefix(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // attrs must be a single identifier
    let prefix = match attrs.into_iter().collect::<Vec<TokenTree>>().as_slice() {
        [TokenTree::Ident(ident)] => Literal::string(&ident.to_string()),
        _ => panic!("The identifier_prefix attribute expectes exactly one attribute (e.g., #[identifier_prefix(pfx)]")
    };

    let tagged_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = tagged_struct.clone().ident;

    let declaration = quote!{
        impl dtdb::data::identifiers::IdentifierPrefix for #struct_name {
            fn identifier_prefix() -> &'static str {
                #prefix
            }
        }
    };

    quote!{
        #tagged_struct
        #declaration
    }.into()
}