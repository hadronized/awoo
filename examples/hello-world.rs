use awoo::scheduler::RandomAccessScheduler;
use awoo::time::simple::SimpleF32TimeGenerator;
use awoo::window::Window;

fn main() {
  let a = Window::new(0., 3.).map(|t| println!("in a: {}", t));
  let b = Window::new(3., 10.).map(|t| println!("in b: {}", t));

  let mut scheduler =
    RandomAccessScheduler::new(
      SimpleF32TimeGenerator::new(0., 1.),
      vec![a, b]
    ).expect("cannot create scheduler");

  scheduler.schedule();
}
