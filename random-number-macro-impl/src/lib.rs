extern crate proc_macro_hack;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, RangeLimits};

struct RandomBuilder {
    min: Option<Box<Expr>>,
    max: Option<Box<Expr>>,
    rng: Option<Box<Expr>>,
    exclusive: bool,
    cmp: bool,
}

impl Parse for RandomBuilder {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.is_empty() {
            Ok(RandomBuilder {
                min: None,
                max: None,
                rng: None,
                exclusive: false,
                cmp: false,
            })
        } else {
            let expr: Expr = input.parse()?;

            if let Expr::Range(range) = expr {
                let exclusive = match range.limits {
                    RangeLimits::HalfOpen(_) => true,
                    RangeLimits::Closed(_) => false,
                };

                let min = range.from;
                let max = range.to;

                if input.is_empty() {
                    Ok(RandomBuilder {
                        min,
                        max,
                        rng: None,
                        exclusive,
                        cmp: false,
                    })
                } else {
                    input.parse::<Token!(,)>()?;

                    let expr: Expr = input.parse()?;

                    Ok(RandomBuilder {
                        min,
                        max,
                        rng: Some(Box::new(expr)),
                        exclusive,
                        cmp: false,
                    })
                }
            } else if input.lookahead1().peek(Token!(,)) {
                input.parse::<Token!(,)>()?;

                let expr2: Expr = input.parse()?;

                if input.is_empty() {
                    Ok(RandomBuilder {
                        min: Some(Box::from(expr)),
                        max: Some(Box::from(expr2)),
                        rng: None,
                        exclusive: false,
                        cmp: true,
                    })
                } else {
                    input.parse::<Token!(,)>()?;

                    let expr3: Expr = input.parse()?;

                    Ok(RandomBuilder {
                        min: Some(Box::from(expr)),
                        max: Some(Box::from(expr2)),
                        rng: Some(Box::new(expr3)),
                        exclusive: false,
                        cmp: true,
                    })
                }
            } else {
                Ok(RandomBuilder {
                    min: None,
                    max: None,
                    rng: Some(Box::new(expr)),
                    exclusive: false,
                    cmp: false,
                })
            }
        }
    }
}

#[proc_macro_hack]
pub fn random(input: TokenStream) -> TokenStream {
    let rb = parse_macro_input!(input as RandomBuilder);

    let random = match rb.min.as_ref() {
        Some(min) => {
            match rb.max.as_ref() {
                Some(max) => {
                    if rb.exclusive {
                        match rb.rng.as_ref() {
                            Some(rng) => {
                                quote! {
                                    $crate::random_exclusively_with_rng(#min, #max, &mut #rng)
                                }
                            }
                            None => {
                                quote! {
                                    $crate::random_exclusively(#min, #max)
                                }
                            }
                        }
                    } else if rb.cmp {
                        match rb.rng.as_ref() {
                            Some(rng) => {
                                quote! {
                                    $crate::random_inclusively_cmp_with_rng(#min, #max, &mut #rng)
                                }
                            }
                            None => {
                                quote! {
                                    $crate::random_inclusively_cmp(#min, #max)
                                }
                            }
                        }
                    } else {
                        match rb.rng.as_ref() {
                            Some(rng) => {
                                quote! {
                                    $crate::random_inclusively_with_rng(#min, #max, &mut #rng)
                                }
                            }
                            None => {
                                quote! {
                                    $crate::random_inclusively(#min, #max)
                                }
                            }
                        }
                    }
                }
                None => {
                    match rb.rng.as_ref() {
                        Some(rng) => {
                            quote! {
                                $crate::random_at_least_with_rng(#min, &mut #rng)
                            }
                        }
                        None => {
                            quote! {
                                $crate::random_at_least(#min)
                            }
                        }
                    }
                }
            }
        }
        None => {
            match rb.max.as_ref() {
                Some(max) => {
                    if rb.exclusive {
                        match rb.rng.as_ref() {
                            Some(rng) => {
                                quote! {
                                    $crate::random_at_most_exclusively_with_rng(#max, &mut #rng)
                                }
                            }
                            None => {
                                quote! {
                                    $crate::random_at_most_exclusively(#max)
                                }
                            }
                        }
                    } else {
                        match rb.rng.as_ref() {
                            Some(rng) => {
                                quote! {
                                    $crate::random_at_most_with_rng(#max, &mut #rng)
                                }
                            }
                            None => {
                                quote! {
                                    $crate::random_at_most(#max)
                                }
                            }
                        }
                    }
                }
                None => {
                    match rb.rng.as_ref() {
                        Some(rng) => {
                            quote! {
                                $crate::random_with_rng(&mut #rng)
                            }
                        }
                        None => {
                            quote! {
                                $crate::random()
                            }
                        }
                    }
                }
            }
        }
    };

    random.into()
}
