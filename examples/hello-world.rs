use awoo::scheduler::RandomAccessScheduler;
use awoo::time::simple::SimpleF32TimeGenerator;
use awoo::window::Window;

fn main() {
  let shared_resource = "Hello, world!".to_owned();
  let a = Window::new(0., 3.).map(|t| println!("{} in a: {}", shared_resource, t));
  let b = Window::new(3., 10.).map(|t| println!("{} in b: {}", shared_resource, t));

  let scheduler =
    RandomAccessScheduler::new(
      SimpleF32TimeGenerator::new(0., 1.),
      vec![a, b]
    ).expect("cannot create scheduler");

  scheduler.schedule();
}
