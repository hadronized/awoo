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

/// Blend two `A` to form a new one.
pub struct Blend<A> {
  blend_f: Box<Fn(&A, &A) -> A>
}

impl<A> Blend<A> {
  pub fn blend(&self, a: &A, b: &A) -> A {
    (self.blend_f)(a, b)
  }
}

/// A cut in a behavior at given time (`T`).
///
/// Cuts represent slice to behaviors, identified by the `C` type variable, with a given start and
/// stop times, identified by the the `T` type variable. The difference between the times gives the
/// duration of the cut.
///
/// A cut also embeds a special object that is responsible in _blending_ cuts. Each cut’s blending
/// object is responsible to blend the current cut and the very next one active at the next time.
/// So if two cuts `cut0` and `cut1` are active at the same time, `cut1`’s blending function will be
/// used to blend `cut1` and `cut2`, and `cut2`’s blending function is ignored in this situation.
pub struct Cut<T, A> {
  /// The behavior the cut refers to.
  pub behavior: Behavior<T, A>,
  /// Time (including) at which the cut starts in the behavior.
  pub start_t: T,
  /// Time (including) at which the cut stops in the behavior.
  pub stop_t: T,
  /// The blending function.
  pub blending: Option<Blend<A>>
}

impl< T, A> Cut<T, A> {
  pub fn new(start_t: T, stop_t: T, behavior: Behavior<T, A>) -> Option<Self> where T: PartialOrd {
    guard!(start_t <= stop_t);

    let blending = None;

    Some(Cut { behavior, start_t, stop_t, blending })
  }

  pub fn dur(&self) -> T where T: Copy + Sub<T, Output = T> {
    self.stop_t - self.start_t
  }

  pub fn react(&self, t: T) -> Option<A> {
    self.behavior.react(t)
  }

  /// React with blending with another cut.
  pub fn react_blend(value: Option<A>, b: &Self, t: T) -> Option<A> where T: Copy {
    match value {
      None => b.react(t),
      Some(a) => {
        match b.blending {
          None => Some(a),
          Some(ref blending) => {
            if let Some(b_result) = b.react(t) {
              Some(blending.blend(&a, &b_result))
            } else {
             Some(a)
            }
          }
        }
      }
    }
  }
}

/// A collection of cuts.
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

/// A type that can generate time when asked.
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

  fn current(&self) -> Self::Time {
    self.current
  }

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
pub struct Scheduler<A, G> where G: TimeGenerator {
  pub tracks: Vec<Track<G::Time, A>>,
  pub time_generator: G,
}

impl<A, G> Scheduler<A, G> where G: TimeGenerator {
  pub fn new<X>(tracks: X, time_generator: G) -> Self where X: Into<Vec<Track<G::Time, A>>> {
    let tracks = tracks.into();
    Scheduler { tracks, time_generator }
  }

  pub fn active_cuts<'a>(&'a self, t: G::Time) -> impl Iterator<Item = &'a Cut<G::Time, A>> + 'a {
    self.tracks.iter().map(move |tr| tr.active(t)).flatten()
  }

  pub fn next(&mut self) -> Option<A> {
    let t = self.time_generator.tick();
    let mut cuts = self.active_cuts(t);

    cuts.next().and_then(move |first_cut| {
      cuts.fold(first_cut.react(t), |value, cut| {
        Cut::react_blend(value, cut, t)
      })
    })
  }
}
