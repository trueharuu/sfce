use std::{
    fmt::{Debug, Display},
    ops::Deref,
    str::FromStr,
};

use anyhow::anyhow;

#[derive(Clone, Debug, Copy)]
pub struct Text<T>(T);

impl<T> FromStr for Text<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(f) = s.strip_prefix("file:") {
            Ok(Self(
                std::fs::read_to_string(f)?
                    .parse()
                    .map_err(|x| anyhow!("{x}"))?,
            ))
        } else {
            Ok(Self(s.parse().map_err(|x| anyhow!("{x}"))?))
        }
    }
}

impl<T> Deref for Text<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Text<T> {
    pub fn contents(self) -> T {
        self.0
    }
}
