use awoo::window::Window;

fn main() {
  let a = Window::new(0., 3.);
  let b = Window::new(3., 10.);

  println!("{}:{}", a.start, a.end);
  println!("{}:{}", b.start, b.end);
}
