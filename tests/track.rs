use awoo::{Behavior, Cut, Track};

#[test]
fn successful_building() {
  let behavior = Behavior::from_fn(|t| Some(t * 2.));

  let track = Track::new(vec![
    Cut::new(0., 1., behavior.clone()).unwrap(),
    Cut::new(1., 2., behavior.clone()).unwrap(),
    Cut::new(3., 20., behavior).unwrap()
  ]);

  assert!(track.is_some());
}

#[test]
fn fail_building() {
  let behavior = Behavior::from_fn(|t| Some(t * 2.));

  let track = Track::new(vec![
    Cut::new(0., 1., behavior.clone()).unwrap(),
    Cut::new(0.5, 2., behavior.clone()).unwrap()
  ]);

  assert!(track.is_none());
}

#[test]
fn active() {
  let behavior = Behavior::from_fn(|t| Some(t * 2.));

  let track = Track::new(vec![
    Cut::new(0., 1., behavior.clone()).unwrap(),
    Cut::new(1., 2., behavior.clone()).unwrap(),
    Cut::new(3., 20., behavior).unwrap()
  ]).unwrap();

  assert_eq!(track.active(0.).unwrap().stop_t, 1.);
  assert_eq!(track.active(0.1).unwrap().stop_t, 1.);
  assert_eq!(track.active(2.).unwrap().stop_t, 2.);
  assert_eq!(track.active(3.).unwrap().stop_t, 20.);
  assert_eq!(track.active(20.).unwrap().stop_t, 20.);
  assert!(track.active(20.1).is_none());
}
