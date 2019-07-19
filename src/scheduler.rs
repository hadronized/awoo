use std::cmp::Ordering;
use try_guard::guard;

use crate::time::TimeGenerator;
use crate::window::MappedWindow;

pub struct RandomAccessScheduler<'a, G> where G: TimeGenerator {
  time_gen: G,
  windows: Vec<MappedWindow<'a, G::Time>>,
}

impl<'a, G> RandomAccessScheduler<'a, G> where G: TimeGenerator {
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

    Some(RandomAccessScheduler { time_gen, windows })
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

  pub fn schedule(mut self) {
    self.time_gen.reset();
    let mut t = self.time_gen.current();

    loop {
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
}

/*

    let mut cuts = cuts.into();

    cuts.sort_by(|a, b| a.start_t.partial_cmp(&b.start_t).unwrap_or(Ordering::Less));

    // ensure there’s no overlapping
    let overlapping = cuts.iter().zip(cuts.iter().skip(1)).any(|(a, b)| b.start_t < a.stop_t);
    guard!(!overlapping);

    Some(Track { cuts })
    */
