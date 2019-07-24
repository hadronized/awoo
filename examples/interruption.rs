use awoo::scheduler::{Interrupt, RandomAccessScheduler};
use awoo::time::simple::SimpleF32TimeGenerator;
use awoo::window::Window;
use std::io::stdin;
use std::sync::mpsc::sync_channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn sleepy<F>(mut f: F) where F: FnMut() {
  f();
  sleep(Duration::from_millis(1000));
}

fn main() {
  let a = Window::new(0., 3.).map(|t| sleepy(|| println!("in a: {}", t)));
  let b = Window::new(3., 10.).map(|t| sleepy(|| println!("in b: {}", t)));

  let (sx, rx) = sync_channel(0);
  let mut scheduler =
    RandomAccessScheduler::new(
      SimpleF32TimeGenerator::new(0., 1.),
      vec![a, b]
    ).expect("cannot create scheduler");

  // handle interruptions here
  spawn(move || {
    let mut dummy = String::new();
    let _ = stdin().read_line(&mut dummy);
    let _ = sx.send(());
  });

  scheduler.interruptible_with(move |t| {
    if let Ok(_) = rx.try_recv() {
      println!("scheduler interrupted at {}", t);
      Interrupt::Break
    } else {
      Interrupt::Continue
    }
  });

  scheduler.schedule();
}
