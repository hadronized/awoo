#[cfg(feature = "json")] use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Window<T> {
  pub start: T,
  pub end: T,
}

impl<T> Window<T> {
  pub fn new(start: T, end: T) -> Self {
    Window {
      start,
      end,
    }
  }

  pub fn map<'a, F>(self, f: F) -> MappedWindow<'a, T> where F: FnMut(T) + 'a {
    MappedWindow {
      window: self,
      carry: Box::new(f)
    }
  }
}

pub struct MappedWindow<'a, T> {
  pub window: Window<T>,
  pub carry: Box<dyn FnMut(T) + 'a>
}
