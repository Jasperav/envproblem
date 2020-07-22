use proc_macro::TokenStream;

#[proc_macro_derive(MyProcMacro)]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    if std::env::var("SOME_ENV_VALUE").is_err() {
        panic!("Failed to retrieve env value")
    }

    TokenStream::new()
}
