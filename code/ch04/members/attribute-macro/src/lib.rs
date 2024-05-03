// use proc_macro2::TokenStream; 

// #[proc_macro_attribute] 
// pub fn say_hello(_attr: TokenStream, item: TokenStream) -> TokenStream { 
//   // The `say_hello` attribute can be applied to any item (e.g. a function, struct, etc.) 
//   // The `item` argument is the token stream representing the item being decorated 
  
//   // Define the code to insert before the item 
//   let prefix = "println!(\"Hello, world!\");\n"; 
  
//   // Convert the prefix code to a token stream 
//   let prefix_tokens = proc_macro2::TokenStream::from_str(prefix).unwrap();

//   // Concatenate the prefix tokens and the item tokens 
//     let expanded = quote::format_ident!("{} {}", prefix_tokens, item); 
    
//     // Return the expanded token stream 
//     TokenStream::from(expanded) 
//   }