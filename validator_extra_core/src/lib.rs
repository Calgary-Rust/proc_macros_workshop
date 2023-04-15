use proc_macro2::TokenStream;

pub fn derive_validation_error_type(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use super::*;

    #[test]
    fn test_validation_error_type() {
        let input = quote! {
            struct MyStruct {
                field1: String,
                field2: String,
            }
        };
        let output = quote!();
        // We can even do pretty diff, if we want to
        // https://github.com/CarlKCarlK/anyinput/blob/d59d04146a8154015e21e2aaf924ee6140393073/anyinput-core/src/tests.rs#L31-L47
        assert_eq!(
            derive_validation_error_type(input).to_string(),
            output.to_string(),
        );
    }
}
