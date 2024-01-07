use crate::components::tools::ctz::Ctz;
use std::{str::FromStr, fmt::Display};

pub trait SomeString {
    fn by_str(input: &str) -> Ctz {
        Ctz::from_str(input).unwrap()
    }

    fn to_string(ctx: Ctz) -> String {
        format!("{:?}", ctx)
    } 
}

impl SomeString for Ctz {}


