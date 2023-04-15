use proc_macro::TokenStream;

#[proc_macro_derive(ValidationErrorType, attributes(validation_error_type))]
pub fn derive_validation_error_type(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(ValidationErrorType)]
    struct MyStruct {
        field1: String,
        field2: String,
    }    

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }    
}
