use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) enum Lifetime {
    XS,
    S,
    M,
    L,
    XL,
    XXL,
}

impl FromStr for Lifetime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "xs" => Ok(Lifetime::XS),
            "s" => Ok(Lifetime::S),
            "m" => Ok(Lifetime::M),
            "l" => Ok(Lifetime::L),
            "xl" => Ok(Lifetime::XL),
            "xxl" => Ok(Lifetime::XXL),
            _ => Err(Error::InvalidLifetime),
        }
    }
}

impl ToString for Lifetime {
    fn to_string(&self) -> String {
        match self {
            Lifetime::XS => "xs".to_string(),
            Lifetime::S => "s".to_string(),
            Lifetime::M => "m".to_string(),
            Lifetime::L => "l".to_string(),
            Lifetime::XL => "xl".to_string(),
            Lifetime::XXL => "xxl".to_string(),
        }
    }
}
