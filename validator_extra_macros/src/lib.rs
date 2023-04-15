use proc_macro::TokenStream;
use validator_extra_core as macro_core;

#[proc_macro_derive(ValidationErrorType, attributes(validation_error_type))]
pub fn derive_validation_error_type(input: TokenStream) -> TokenStream {
    macro_core::derive_validation_error_type(input.into()).into()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(ValidationErrorType)]
//     struct MyStruct {
//         field1: String,
//         field2: String,
//     }    

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }    
// }
