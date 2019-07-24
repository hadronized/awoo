use awoo::scheduler::RandomAccessScheduler;
use awoo::time::simple::SimpleF32TimeGenerator;
use awoo::window::Window;
use serde_json::from_str;
use std::collections::HashMap;

const WINDOWS: &str = r#"
{
  "a": {
    "start": 0,
    "end":   3
  },
  "b": {
    "start": 3,
    "end":  10
  }
}"#;

fn main() {
  let shared_resource = "Hello, world!".to_owned();
  let windows: HashMap<String, Window<f32>> = from_str(WINDOWS).expect("cannot deserialize windows");
  let a = windows.get("a").unwrap().map(|t| println!("{} in a: {}", shared_resource, t));
  let b = windows.get("b").unwrap().map(|t| println!("{} in b: {}", shared_resource, t));

  let mut scheduler =
    RandomAccessScheduler::new(
      SimpleF32TimeGenerator::new(0., 1.),
      vec![a, b]
    ).expect("cannot create scheduler");

  scheduler.schedule();
}
