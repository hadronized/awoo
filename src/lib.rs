//! Animate and schedule code.
//!
//! This crate provides the functionality of running code at given times, in the same way as
//! animation artists create animated movies. The idea of this crate is to ease building code-driven
//! artistic animated programs.
//!
//! # Concepts

#![feature(try_trait)]

use std::cmp::Ordering;
use std::ops::Sub;
use std::rc::Rc;
use try_guard::guard;

/// A behavior that gives values of type `A` varying over time `T`.
///
/// A behavior is just whatever function that can provide a value at any time of `T`.
#[derive(Clone)]
pub struct Behavior<T, A> {
  behavior: Rc<Fn(T) -> Option<A>>
}

impl<T, A> Behavior<T, A> {
  pub fn from_fn<F>(f: F) -> Self where F: 'static + Fn(T) -> Option<A> {
    Behavior {
      behavior: Rc::new(f)
    }
  }

  pub fn react(&self, t: T) -> Option<A> {
    (self.behavior)(t)
  }
}

/// A cut in a behavior at given time (`T`).
///
/// Cuts represent slice to behaviors, identified by the `C` type variable, with a given start and
/// stop times, identified by the the `T` type variable. The difference between the times gives the
/// duration of the cut.
///
/// A cut also embed transactions. Basically, it’s possible that several cuts are triggered at the
/// same time. In that case, each cut contains some additional information about how to deal with
/// such overlapping.
#[derive(Clone)]
pub struct Cut<T, A> {
  /// The behavior the cut refers to.
  pub behavior: Behavior<T, A>,
  /// Time (including) at which the cut starts in the behavior.
  pub start_t: T,
  /// Time (including) at which the cut stops in the behavior.
  pub stop_t: T,
}

impl< T, A> Cut<T, A> {
  pub fn new(start_t: T, stop_t: T, behavior: Behavior<T, A>) -> Option<Self> where T: PartialOrd {
    guard!(start_t <= stop_t);

    Some(Cut { behavior, start_t, stop_t })
  }

  pub fn dur(&self) -> T where T: Copy + Sub<T, Output = T> {
    self.stop_t - self.start_t
  }
}

/// A collection of cuts.
#[derive(Clone)]
pub struct Track<T, A> {
  // Cuts, sorted by start time
  cuts: Vec<Cut<T, A>>
}

impl<T, A> Track<T, A> {
  /// Create a new track.
  ///
  /// This will fail if two cuts overlap. In such a situation, move one cut to another track.
  pub fn new<C>(cuts: C) -> Option<Self> where C: Into<Vec<Cut<T, A>>>, T: PartialOrd {
    let mut cuts = cuts.into();

    cuts.sort_by(|a, b| a.start_t.partial_cmp(&b.start_t).unwrap_or(Ordering::Less));

    // ensure there’s no overlapping
    let overlapping = cuts.iter().zip(cuts.iter().skip(1)).any(|(a, b)| b.start_t < a.stop_t);
    guard!(!overlapping);

    Some(Track { cuts })
  }

  /// Return the currently active cut at a given time, if any.
  pub fn active(&self, t: T) -> Option<&Cut<T, A>> where T: PartialOrd {
    let x = self.cuts.binary_search_by(|cut| {
      match cut.start_t.partial_cmp(&t).unwrap_or(Ordering::Less) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Greater,

        Ordering::Less => match t.partial_cmp(&cut.stop_t).unwrap_or(Ordering::Less) {
          Ordering::Less | Ordering::Equal => Ordering::Equal,
          Ordering::Greater => Ordering::Less
        }
      }
    }).ok()?;

    self.cuts.get(x)
  }
}

/// A collection of tracks.
#[derive(Clone)]
pub struct Timeline<T, A> {
  tracks: Vec<Track<T, A>>
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
pub struct SimpleF32TimeGenerator {
  current: f32,
  reset_value: f32,
  delta: f32
}

impl SimpleF32TimeGenerator {
  pub fn new(reset_value: f32, delta: f32) -> Self {
    SimpleF32TimeGenerator {
      current: reset_value,
      reset_value,
      delta
    }
  }
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
#[derive(Clone)]
pub struct Scheduler<T, A, G> {
  pub timeline: Timeline<T, A>,
  pub time_generator: G,
}

impl<T, A, G> Scheduler<T, A, G> {
  pub fn new(timeline: Timeline<T, A>, time_generator: G) -> Self {
    Scheduler { timeline, time_generator }
  }
}

impl<T, A, G> Iterator for Scheduler<T, A, G> where G: TimeGenerator {
  type Item = A;

  fn next(&mut self) -> Option<Self::Item> {
    let t = self.time_generator.tick();
    unimplemented!()
  }
}
