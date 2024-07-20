use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::repeat_with;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, LitStr, Token};

fn gcd(a: u32, b: u32) -> u32 {
    if a > b {
        ordered_gcd(a, b)
    } else {
        ordered_gcd(b, a)
    }
}

fn ordered_gcd(big: u32, small: u32) -> u32 {
    if small == 0 {
        big
    } else {
        let new_small = big % small;
        ordered_gcd(small, new_small)
    }
}

#[proc_macro]
pub fn fizz_buzz_generator(input: TokenStream) -> TokenStream {
    let FizzBuzzConfig(configurations, separator) = parse_macro_input!(input as FizzBuzzConfig);

    let modulo = configurations
        .iter()
        .fold(1, |acc, conf| acc * conf.divisor / gcd(acc, conf.divisor));

    let mut mapping = HashMap::new();

    for conf in configurations.iter() {
        let mut last_value = 0;
        for multiple in repeat_with(|| {
            let tmp = last_value;
            last_value += conf.divisor;
            tmp
        })
        .take_while(|value| *value < modulo)
        {
            if let Some(mapping_entry) = mapping.get_mut(&multiple) {
                *mapping_entry = Cow::from(format!(
                    "{}{}{}",
                    *mapping_entry, separator, conf.replacement
                ));
            } else {
                mapping.insert(multiple, Cow::from(&conf.replacement));
            }
        }
    }

    let mut reverse_map: HashMap<Cow<'_, str>, Vec<u32>> = HashMap::new();
    for (div, cow) in mapping.into_iter() {
        if let Some(entry) = reverse_map.get_mut(&cow) {
            entry.push(div);
        } else {
            reverse_map.insert(cow, vec![div]);
        }
    }

    let patterns = reverse_map.into_iter().map(|(str, pattern)| {
        quote! { #(#pattern) | * => #str.into() }
    });

    let expanded = quote! {
        |input: u32| -> std::borrow::Cow<'_, str> {
            match input % #modulo {
                #(#patterns),*,
                _ => format!("{input}").into(),
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
    divisor: u32,
    replacement: String,
}

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct _ReadMe;
