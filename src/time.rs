//! Time managing.
//!
//! This module exports several implementation of several _time generator_. A _time generator_ is
//! an object which type implements [`TimeGenerator`]. The idea is that _schedulers_ will need
//! several properties about time:
//!
//!   - They obviously need to get access to _the current time_. That can represent anything, from
//!     a simulation frame to a real-world clock.
//!   - They need to _take into account_ the fact time passes. The speed at which it passes is
//!     entirely up to the implementation of [`TimeGenerator`] for the type you choose. Time can
//!     be ticked _forward_ but also _backwards_.
//!   - Resetting the time generator resets its internals to its initial state.
//!   - Time can be set explicitly, too.
//!   - Finally, because time generators often _tick_ or _untick_ (ticking backards), it’s possible
//!     to get a time difference between two ticks. That difference is called a _delta_ and it’s
//!     also possible to change it.
//!
//! [`TimeGenerator`]: crate::time::TimeGenerator

pub mod simple;

/// Set of types that can handle time.
///
/// A time generator provides a way to:
///
///   - Get the current time.
///   - Tick forward time in order to get the “next time”.
///   - Tick backwards (a.k.a. unticking).
///   - Resetting to its initial or normal value.
///   - Change the internal delta time used to tick / untick.
pub trait TimeGenerator {
  /// Type of time.
  type Time: PartialOrd + Copy;

  /// Get the current time.
  fn current(&self) -> Self::Time;

  /// Tick time forward.
  fn tick(&mut self) -> Self::Time;

  /// Tick time backwards.
  fn untick(&mut self) -> Self::Time;

  /// Reset the generator and time to their initial values.
  fn reset(&mut self);

  /// Set the generator to a given value.
  fn set(&mut self, value: Self::Time);

  /// Change the internal delta.
  fn change_delta(&mut self, delta: Self::Time);
}
