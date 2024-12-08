use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct Ranged<T>
where
    T: PartialOrd,
{
    lo: Option<T>,
    hi: Option<T>,
}

impl<T> Ranged<T>
where
    T: PartialOrd,
{
    pub fn new(lo: Option<T>, hi: Option<T>) -> Self {
        Self { lo, hi }
    }

    pub fn contains(self, t: &T) -> bool {
        self.lo.is_none_or(|x| t >= &x) && self.hi.is_none_or(|x| t <= &x)
    }
}

impl<T> FromStr for Ranged<T>
where
    T: PartialOrd + FromStr,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = s.split_once("..");
        if let Some((l, h)) = t {
          
            let lo = if l.is_empty() { None } else { Some(l.parse()?) };
            let ho = if h.is_empty() { None } else { Some(h.parse()?) };

            Ok(Self::new(lo, ho))
        } else {
            let o = s.parse()?;
            let t = s.parse()?;
            Ok(Self::new(Some(o), Some(t)))
        }
    }
}
