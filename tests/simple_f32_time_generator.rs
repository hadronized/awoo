use awoo::time::TimeGenerator;
use awoo::time::simple::SimpleF32TimeGenerator;

// f32 equality test
#[inline]
fn f32eq(a: f32, b: f32) -> bool {
  if a == b {
    true
  } else {
    if b == 0. {
      a == 0.
    } else {
      (a - b).abs() / b <= 2. * std::f32::EPSILON
    }
  }
}

#[test]
fn forward() {
  let mut gen = SimpleF32TimeGenerator::new(0., 0.1);

  for i in 0..10 {
    let delta = i as f32 / 10.;
    assert!(f32eq(gen.tick(), delta));
  }
}

#[test]
fn backward() {
  let mut gen = SimpleF32TimeGenerator::new(0., 0.1);

  for i in 0..10 {
    let delta = -i as f32 / 10.;
    assert!(f32eq(gen.untick(), delta));
  }
}

#[test]
fn reset() {
  let mut gen = SimpleF32TimeGenerator::new(0., 0.1);

  for i in 0..10 {
    let delta = i as f32 / 10.;
    assert!(f32eq(gen.tick(), delta));
  }

  gen.reset();
  assert!(f32eq(gen.tick(), 0.));
}
