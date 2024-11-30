pub trait CollectVec {
    type Item;
    fn vec(self) -> Vec<Self::Item>;
}

impl<T> CollectVec for T
where
    T: Iterator,
{
    type Item = T::Item;
    fn vec(self) -> Vec<Self::Item> {
        // dumb
        self.collect()
    }
}

pub trait GetWith: IntoIterator {
    fn get_with<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        self.into_iter().find(f)
    }
}

impl<T> GetWith for T where T: IntoIterator {}