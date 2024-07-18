use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, LitStr, Token};

#[proc_macro]
pub fn fizz_buzz_generator(input: TokenStream) -> TokenStream {
    let generator = parse_macro_input!(input as FizzBuzzConfig);

    let matches: Vec<_> = generator
        .0
        .into_iter()
        .map(move |configuration| {
            let divisor = configuration.divisor as u32;
            let replacement = configuration.replacement;
            quote! {
            if i % #divisor == 0 {
                matching_numbers.push(String::from(#replacement));
            } }
        })
        .collect();

    let separator = generator.1;
    let max_matches = matches.len();

    let expanded = quote! {
        |i: u32| {
            let mut matching_numbers = Vec::with_capacity(#max_matches);
            #(#matches);*
            if matching_numbers.is_empty() {
                format!("{i}")
            }else{
                matching_numbers.join(#separator)
            }
        }
    };

    TokenStream::from(expanded)
}

struct FizzBuzzConfig(Vec<Configuration>, String);

impl Parse for FizzBuzzConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut configuration = vec![];
        let mut separator = "".to_string();

        loop {
            if input.peek(LitInt) {
                let divisor = input.parse::<LitInt>()?.base10_parse()?;
                input.parse::<Token![=>]>()?;
                let replacement = input.parse::<LitStr>()?.value();
                configuration.push(Configuration {
                    divisor,
                    replacement,
                });
            } else {
                separator = input.parse::<LitStr>()?.value();
            }
            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        Ok(Self(configuration, separator))
    }
}

#[derive(Debug)]
struct Configuration {
    divisor: u8,
    replacement: String,
}
