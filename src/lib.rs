use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, LitStr, Token};

#[proc_macro]
pub fn fizz_buzz_generator(input: TokenStream) -> TokenStream {
    let FizzBuzzConfig(configurations, separator) = parse_macro_input!(input as FizzBuzzConfig);

    let max_string_size = configurations.iter().fold(0, |mut acc, add| {
        acc += add.replacement.len();
        acc
    });

    let matching_slice = configurations.into_iter().map(move |conf| {
        let divisor = conf.divisor as u32;
        let replacement = &conf.replacement;
        quote! { (#divisor, #replacement) }
    });

    let expanded = quote! {
        |input: u32| {
            let matching_slices = &[#(#matching_slice), *];
            let result = matching_slices
                .into_iter()
                .filter(|(d, _)| input % d == 0)
                .enumerate()
                .fold(String::with_capacity(#max_string_size), |mut carry, (i, (_, w))| {
                if i != 0 {
                    carry.push_str(#separator);
                }
                carry.push_str(w);
                carry
            });
            if result.is_empty() {
                format!("{input}")
            }else{
                result
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
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
