// extern crate proc_macro;

// use proc_macro::TokenStream; 
// use quote::quote; 
// use syn::{parse_macro_input, DeriveInput};

// #[proc_macro_derive(Debug)] 
// pub fn debug_derive(input: TokenStream) -> TokenStream { 
//   // Parse the input tokens into a syntax tree 
//   let ast: DeriveInput = parse_macro_input!(input); 
//   // Build the output tokens 
//   let expanded = quote! { 
//     impl ::std::fmt::Debug for #ast { 
//       fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { 
//         // Define the formatting for the struct 
//         let mut f = f.debug_struct(stringify!(#ast)); 
//         // Add each field to the output 
//         #(f.field(stringify!(#ast.#ast_field_name), &self.#ast_field_name);)* 
//         // Finish the output and return it 
//         f.finish() 
//       } 
//     } 
//   }; 
//   // Return the expanded tokens 
//   TokenStream::from(expanded) 
// }