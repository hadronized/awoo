use crate::time::TimeGenerator;
use crate::window::MappedWindow;

pub struct SequentialScheduler<'a, G, T> {
  time_generator: G,
  windows: Vec<MappedWindow<'a, T>>
}
