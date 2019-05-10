use awoo::{Behavior, Cut, Scheduler, SimpleF32TimeGenerator, Track};

#[test]
fn simple() {
  let tgen = SimpleF32TimeGenerator::new(0., 0.1);
  let behavior = Behavior::from_fn(|t: f32| Some([t.cos(), t.sin()]));
  let track = Track::new(vec![
    Cut::new(0., 1., behavior).unwrap()
  ]).unwrap();
  let mut scheduler = Scheduler::new(vec![track], tgen);

  let mut i = 0;
  for _ in 0..10 {
    assert!(scheduler.next().is_some());
  }

  assert!(scheduler.next().is_none());
}
