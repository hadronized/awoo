//! Animate and schedule code.
//!
//! This crate provides the functionality of running code at given times, in the same way as
//! animation artists create animated movies. The idea of this crate is to ease building code-driven
//! artistic animated programs.
//!
//! # Concepts

#![feature(try_trait)]

use std::ops::Sub;
use try_guard::guard;

/// A cut in a clip (`C`) at given time (`T`).
///
/// Cuts represent slice to clips, identified by the `C` type variable, with a given start and
/// stop times, identified by the the `T` type variable. The difference between the times gives the
/// duration of the cut.
///
/// A cut also embed transactions. Basically, it’s possible that several cuts are triggered at the
/// same time. In that case, each cut contains some additional information about how to deal with
/// such overlapping.
pub struct Cut<'a, C, T> {
  /// The clip the cut refers to.
  pub clip: &'a C,
  /// Time (including) at which the cut starts in the clip.
  pub start_in: T,
  /// Time (including) at which the cut stops in the clip.
  pub stop_in: T,
}

impl<'a, C, T> Cut<'a, C, T> {
  fn new(clip: &'a C, start_in: T, stop_in: T) -> Option<Self> where T: PartialOrd {
    guard!(stop_in < start_in);

    Some(Cut { clip, start_in, stop_in })
  }

  fn dur(&self) -> T where T: Copy + Sub<T, Output = T> {
    self.stop_in - self.start_in
  }
}

/// A collection of cuts.
pub struct Track<'c, C, T> {
  cuts: Vec<Cut<'c, C, T>>
}

/// A collection of tracks.
pub struct Timeline<'c, C, T> {
  tracks: Vec<Track<'c, C, T>>
}

/// A type that can generate time when asked.
pub trait TimeGenerator {
  type Time;

  /// Tick time forward.
  fn tick(&mut self) -> Self::Time;

  /// Tick time backwards.
  fn untick(&mut self) -> Self::Time;

  /// Reset the generator and time to their initial values.
  fn reset(&mut self);

  /// Change the internal delta.
  fn change_delta(&mut self, delta: Self::Time);
}

/// A simple generator that generates `f32` times by delta.
struct SimpleF32TimeGenerator {
  current: f32,
  reset_value: f32,
  delta: f32
}

impl TimeGenerator for SimpleF32TimeGenerator {
  type Time = f32;

  fn tick(&mut self) -> Self::Time {
    let t = self.current;
    self.current += self.delta;
    t
  }

  fn untick(&mut self) -> Self::Time {
    let t = self.current;
    self.current -= self.delta;
    t
  }

  fn reset(&mut self) {
    self.current = self.reset_value
  }

  fn change_delta(&mut self, delta: Self::Time) {
    self.delta = delta;
  }
}

/// In the lack of a better name, I’ll call that shit Scheduler. And I’m drunk.
pub struct Scheduler<'a, C, T, G> {
  timeline: Timeline<'a, C, T>,
  time_generator: G,
}
