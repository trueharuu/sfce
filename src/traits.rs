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

pub fn contiguous_subsequences<I, T>(input: I) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T>,
    T: Clone,
{
    let input: Vec<T> = input.into_iter().collect(); // Collect the input into a Vec
    let mut out = vec![];

    for start in 0..input.len() {
        for end in start..input.len() {
            out.push(input[start..=end].to_vec());
        }
    }

    out
}

// TODO: make this really fast
pub fn contiguous_cut_seqs<I, T>(input: I) -> Vec<(Vec<T>, Vec<T>, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    T: Clone,
{
    let input: Vec<T> = input.into_iter().collect(); // Collect the input into a Vec
    let mut out = vec![];

    for start in 0..input.len() {
        for end in start..input.len() {
            out.push((
                input[..start].to_vec(),
                input[start..=end].to_vec(),
                input[end + 1..].to_vec(),
            ));
        }
    }

    out
}

pub fn do_until_same<T>(mut value: T, mut func: impl FnMut(T) -> T) -> T
where
    T: PartialEq + Clone,
{
    loop {
        let new_value = func(value.clone());
        if new_value == value {
            break;  // Exit the loop if the value didn't change
        }
        value = new_value;  // Update the value with the new value
    }
    value  // Return the final value
}