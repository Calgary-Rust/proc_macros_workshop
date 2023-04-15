use proc_macro::TokenStream;

#[proc_macro_derive(ValidationErrorType, attributes(validation_error_type))]
pub fn derive_validation_error_type(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
