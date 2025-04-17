use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Type, Data, DeriveInput, Fields, FieldsNamed, Ident};
use syn::__private::TokenStream2;

fn map_field<F>(fields: &Fields, func: F) -> TokenStream2
    where 
    F: FnMut((Ident, Type)) -> TokenStream2
{   
    let iter_stream = fields.iter()
        .map(|f| f.clone())
        .map(|field| (field.ident.unwrap(), field.ty))
        .map(func);
    TokenStream2::from_iter(iter_stream)
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_ident = format_ident!("{}Builder", ident);
    
    if let Data::Struct(data_struct) = input.data {
        let fields = data_struct.fields;
        if matches!(fields, Fields::Named(_)) {
            // let builder_fields = fields.iter()
            //     .map(|f| f.clone())
            //     .map(|field| (field.ident.unwrap(), field.ty))
            //     .map();
            let build_fields = map_field(&fields, |(ident, ty)| quote! {#ident: Option<#ty>, });
            let build_set_method = map_field(&fields, |(ident, ty)| quote! {
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
            });
        }
    }

    quote! {
        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident
            }
        }
        pub struct #builder_ident;
    }.into()
}


// #[cfg(test)]
// mod tests_macro {
//     #[test]
//     fn it_words() {
//         let result = 2+ 2;
//         println!("results: {}", result);
//     }
// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     macro_rules! assert_parse {
//         ($a: tt) => {
//             assert_eq!($a("Hello, world!"), "Hello, world!")
//         };
//     }

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);

//         println!();
//     }
// }

