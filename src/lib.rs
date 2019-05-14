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
/// A behavior is just whatever function that can provide a value at any time.
#[derive(Clone)]
pub struct Behavior<T, A> {
  behavior: Rc<FnMut(T) -> Option<A>>
}

impl<T, A> Behavior<T, A> {
  /// Build a behavior from a function.
  ///
  /// Returning [`None`] makes the behavior disabled. This means that its value make any [`Cut`]
  /// using it at this time unactive.
  pub fn from_fn<F>(f: F) -> Self where F: 'static + FnMut(T) -> Option<A> {
    Behavior {
      behavior: Rc::new(f)
    }
  }

  /// React and provide a value for the given time.
  ///
  /// Returning [`None`] makes the behavior disabled. This means that its value make any [`Cut`]
  /// using it at this time unactive.
  pub fn react(&self, t: T) -> Option<A> {
    (self.behavior)(t)
  }
}

/// Blend two `A` to form a new one.
///
/// This type allows you to combine two values that are triggered at the same time.
pub struct Blend<A> {
  blend_f: Box<FnMut(&A, &A) -> A>
}

impl<A> Blend<A> {
  /// Blend two values.
  pub fn blend(&self, a: &A, b: &A) -> A {
    (self.blend_f)(a, b)
  }
}

/// A cut in a [`Behavior`] at given time (`T`).
///
/// Cuts represent slices to behaviors, identified by the `C` type variable, with a given start and
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
  /// Time (including) at which the cut starts in the [`Behavior`].
  pub start_t: T,
  /// Time (including) at which the cut stops in the [`Behavior`].
  pub stop_t: T,
  /// The [`Blend`] function to use if blending must happen.
  pub blending: Option<Blend<A>>
}

impl< T, A> Cut<T, A> {
  /// Create a new [`Cut`].
  pub fn new(start_t: T, stop_t: T, behavior: Behavior<T, A>) -> Option<Self> where T: PartialOrd {
    guard!(start_t <= stop_t);

    let blending = None;

    Some(Cut { behavior, start_t, stop_t, blending })
  }

  /// Get the duration of this [`Cut`].
  pub fn dur(&self) -> T where T: Copy + Sub<T, Output = T> {
    self.stop_t - self.start_t
  }

  /// Invoke the underlying [`Behavior`] this cut references and make it react with time.
  pub fn react(&self, t: T) -> Option<A> {
    self.behavior.react(t)
  }

  /// React by blending with another cut.
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

/// A collection of [`Cut`]s.
///
/// [`Cut`]s are organized by [`Track`]s by sorting them in ascending order (based on their start
/// time). That yields timeline-like structure that can have several [`Cut`]s active at the same
/// time. Such situations are resolved by a scheduler.
pub struct Track<T, A> {
  // [`Cut`]s, sorted by start time
  cuts: Vec<Cut<T, A>>
}

impl<T, A> Track<T, A> {
  /// Create a new [`Track`].
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

/// A simple [`TimeGenerator`] that generates `f32` times.
pub struct SimpleF32TimeGenerator {
  current: f32,
  reset_value: f32,
  delta: f32
}

impl SimpleF32TimeGenerator {
  /// Create a new [`SimpleF32TimeGenerator`].
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
    self.set(self.reset_value);
  }

  fn set(&mut self, value: Self::Time) {
    self.current = value;
  }

  fn change_delta(&mut self, delta: Self::Time) {
    self.delta = delta;
  }
}

/// Schedule [`Behavior`]s referenced by [`Cut`]s in [`Track`]s using a [`TimeGenerator`].
///
/// A [`Scheduler`] is responsible in serving values of type `A` in time. They may be value or
/// `None`.
///
/// Schedulers are currently mutable by design and you should use the [`Iterator`] interface.
/// However, it is also possible to use them in a *random-access* fashion way by using the
/// [`Scheduler::value_at`] method.
pub struct Scheduler<A, G> where G: TimeGenerator {
  tracks: Vec<Track<G::Time, A>>,
  time_generator: G,
}

impl<A, G> Scheduler<A, G> where G: TimeGenerator {
  /// Create a new [`Scheduler`].
  pub fn new<X>(tracks: X, time_generator: G) -> Self where X: Into<Vec<Track<G::Time, A>>> {
    let tracks = tracks.into();
    Scheduler { tracks, time_generator }
  }

  /// Get the current active [`Cut`]s for a given time.
  fn active_cuts<'a>(&'a self, t: G::Time) -> impl Iterator<Item = &'a Cut<G::Time, A>> + 'a {
    self.tracks.iter().map(move |tr| tr.active(t)).flatten()
  }

  /// Get the value at the given time.
  pub fn value_at(&self, t: G::Time) -> Option<A> {
    let mut cuts = self.active_cuts(t);

    cuts.next().and_then(move |first_cut| {
      cuts.fold(first_cut.react(t), |value, cut| {
        Cut::react_blend(value, cut, t)
      })
    })
  }

  /// Get the next value generated.
  pub fn next_value(&mut self) -> Option<A> {
    let t = self.time_generator.tick();
    self.value_at(t)
  }

  /// Get the previous value generated.
  pub fn prev_value(&mut self) -> Option<A> {
    let t = self.time_generator.untick();
    self.value_at(t)
  }
}

impl<A, G> Iterator for Scheduler<A, G> where G: TimeGenerator {
  type Item = A;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_value()
  }
}
