//! Time windows and related.
//!
//! A time window is a just a simple pair of time points, giving a range of time that is inclusive
//! in its left (start) bound and exclusive in its right (end) part. The type of time is a type
//! variable on the [`Window`] type.
//!
//! A time window can then be mapped to generate a [`MappedWindow`], which is akin to a
//! varying value inside that initial time window. [`MappedWindow`] are keys in the architecture of
//! `awoo`.
//!
//! [`Window`]: crate::window::Window
//! [`MappedWindow`]: crate::window::MappedWindow

#[cfg(feature = "json")] use serde::{Deserialize, Serialize};

/// A pure time window.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Window<T> {
  /// Start time (inclusive) of the window.
  pub start: T,
  /// End time (exclusive) of the window.
  pub end: T,
}

impl<T> Window<T> {
  /// Create a new time window.
  pub fn new(start: T, end: T) -> Self {
    Window {
      start,
      end,
    }
  }

  /// Map an action to perform inside the [`Window`].
  pub fn map<'a, F>(self, f: F) -> MappedWindow<'a, T> where F: FnMut(T) + 'a {
    MappedWindow {
      window: self,
      carry: Box::new(f)
    }
  }
}

/// Action scoped to time windows.
pub struct MappedWindow<'a, T> {
  /// Window into which execute an action.
  pub window: Window<T>,
  pub(crate) carry: Box<dyn FnMut(T) + 'a>
}
