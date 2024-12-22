use std::fmt::Debug;

/// Similar to a `HashSet` that allows you to determine what makes two elements equal
pub struct Set<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    predicate: F,
    values: Vec<T>,
}

impl<T, F> Set<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    pub fn new(f: F) -> Self {
      Self { predicate: f, values: vec![] }
    }

    pub fn eq(&self, x: &T, y: &T) -> bool {
        (self.predicate)(x, y)
    }

    pub fn insert(&mut self, t: T) {
      if !self.has(&t) {
        self.values.push(t);
      }
    }

    pub fn has(&self, t: &T) -> bool {
        self.values.iter().any(|x| self.eq(x, t))
    }

    pub fn values(&self) -> &[T] {
      &self.values
    }

    pub fn size(&self) -> usize {
      self.values.len()
    }

    pub fn is_empty(&self) -> bool {
      self.size() == 0
    }
}

impl<T, F> Debug for Set<T, F> where T: Debug, F: Fn(&T, &T) -> bool {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_tuple("Set").field(&self.values).finish()
  }
}