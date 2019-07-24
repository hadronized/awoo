//! Scheduler implementations.
//!
//! Schedulers are used to order and schedule [`MappedWindow`]s by using a [`TimeGenerator`]. You
//! typically create a scheduler along with a time generator and a list ([`Vec`]) of time windows.
//! Note that it’s possible that the creation of a scheduler fails because of the time windows. For
//! instance, overlapping time windows are forbidden.
//!
//! Once the scheduler created, you can use it to schedule the mapped action in the windows.
//!
//! [`MappedWindow`]: crate::window::MappedWindow
//! [`TimeGenerator`]: crate::time::TimeGenerator

use std::cmp::Ordering;
use try_guard::guard;

use crate::time::TimeGenerator;
use crate::window::MappedWindow;

/// A random-access scheduler.
///
/// A random-access scheduler gives you an interesting property: given any time, it will perform
/// resolution of the action to use based on the logarithm of the number of windows being scheduled.
/// That gives good performance for someone who is constantly changing time without ticking or
/// unticking with a small delta.
///
/// On the opposite hand, if you don’t need that random-access property, then a sequential
/// scheduler will make a way better job for you (it will give you a _O(1)_ runtime performance
/// instead of _O(log N)_).
///
/// > Note: if you use a sequential scheduler by doing random-accesses, you are basically ruining
/// > the initial concept of a sequential scheduler (it will run in _O(N)_ at worst).
pub struct RandomAccessScheduler<'a, G> where G: TimeGenerator {
  time_gen: G,
  windows: Vec<MappedWindow<'a, G::Time>>,
  interrupt: Option<Box<FnMut(G::Time) -> Interrupt + 'a>>
}

impl<'a, G> RandomAccessScheduler<'a, G> where G: TimeGenerator {
  /// Create a new random-access scheduler.
  ///
  /// This function might fail if the time windows are overlapping.
  pub fn new<W>(
    time_gen: G,
    windows: W
  ) -> Option<Self>
  where W: Into<Vec<MappedWindow<'a, G::Time>>> {
    let mut windows = windows.into();

    windows.sort_by(|a, b| a.window.start.partial_cmp(&b.window.start).unwrap_or(Ordering::Less));
    //
    // ensure there’s no overlapping
    let overlapping = windows.iter().zip(windows.iter().skip(1)).any(|(a, b)| {
      b.window.start < a.window.end
    });
    guard!(!overlapping);

    Some(RandomAccessScheduler {
      time_gen,
      windows,
      interrupt: None
    })
  }

  fn active_window_index(&self, t: G::Time) -> Option<usize> {
    self.windows.binary_search_by(|win| {
      match win.window.start.partial_cmp(&t).unwrap_or(Ordering::Less) {
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Greater,

        Ordering::Less => match t.partial_cmp(&win.window.end).unwrap_or(Ordering::Less) {
          Ordering::Less | Ordering::Equal => Ordering::Equal,
          Ordering::Greater => Ordering::Less
        }
      }
    }).ok()
  }

  /// Schedule the mapped windows.
  pub fn schedule(&mut self) {
    self.time_gen.reset();
    let mut t = self.time_gen.current();

    loop {
      if let Some(ref mut interrupt) = self.interrupt {
        if let Interrupt::Break = (interrupt)(t) {
          break;
        }
      }

      let win_ix = self.active_window_index(t);

      if let Some(win_ix) = win_ix {
        ((&mut self.windows[win_ix]).carry)(t);
      }

      self.time_gen.tick();
      t = self.time_gen.current();

      // check whether the simulation is done
      if let Some(last_win) = self.windows.last() {
        if t >= last_win.window.end {
          break
        }
      }
    }
  }

  /// Make the scheduler interruptible with the given function
  ///
  /// > Note: the function must not block and return as soon as possible.
  pub fn interruptible_with<F>(&mut self, interrupt: F) where F: FnMut(G::Time) -> Interrupt + 'a {
    self.interrupt = Some(Box::new(interrupt));
  }
}

/// Interruption mechanism.
///
/// A scheduler has to check when an interruption has occurred. If one does, it must return from the
/// `schedule` method and give back control-flow.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Interrupt {
  Break,
  Continue
}
