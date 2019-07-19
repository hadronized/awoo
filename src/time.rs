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

