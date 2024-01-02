use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd, Ord, PartialEq)]
pub enum Lifetime {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_to_string() {
        assert_eq!(Lifetime::XS.to_string(), "xs");
        assert_eq!(Lifetime::S.to_string(), "s");
        assert_eq!(Lifetime::M.to_string(), "m");
        assert_eq!(Lifetime::L.to_string(), "l");
        assert_eq!(Lifetime::XL.to_string(), "xl");
        assert_eq!(Lifetime::XXL.to_string(), "xxl");
    }

    #[test]
    fn test_create_lifetime_from_str() {
        assert_eq!(Lifetime::from_str("xs"), Ok(Lifetime::XS));
        assert_eq!(Lifetime::from_str("s"), Ok(Lifetime::S));
        assert_eq!(Lifetime::from_str("m"), Ok(Lifetime::M));
        assert_eq!(Lifetime::from_str("l"), Ok(Lifetime::L));
        assert_eq!(Lifetime::from_str("xl"), Ok(Lifetime::XL));
        assert_eq!(Lifetime::from_str("xxl"), Ok(Lifetime::XXL));
        assert_eq!(Lifetime::from_str("unknown"), Err(Error::InvalidLifetime));
    }

    #[test]
    fn test_ord_lifetime() {
        // equal
        assert!(Lifetime::XS == Lifetime::XS);
        assert!(Lifetime::XS != Lifetime::M);

        // cmp
        assert!(Lifetime::XS < Lifetime::S);
        assert!(Lifetime::S < Lifetime::M);
        assert!(Lifetime::M < Lifetime::L);
        assert!(Lifetime::L < Lifetime::XL);
        assert!(Lifetime::XL < Lifetime::XXL);
    }
}
