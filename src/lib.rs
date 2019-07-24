//! Animate and schedule code.
//!
//! This crate provides a very simple mechanism to animate some code. Consider:
//!
//! ```
//! let mut time = 0.; // time that passes
//! let a = 0.;
//! let b = 1.;
//! let c = 2.;
//!
//! // main loop of our application
//! loop {
//!   if time < a {
//!     // do something until time passes a point in time a
//!   } else if time < b {
//!     // do something until time passes a point in time b
//!   } else if time < c {
//!     // do something until time passes a point in time c
//!   } // etc. etc.
//!
//!   // time advances here
//!   time += 0.1;
//!
//!   if time >= c {
//!     break
//!   }
//! }
//! ```
//!
//! That might sound surprising to you, but _a lot_ of demoscene productions are written with that kind
//! of `if / else if` blocks. It’s pretty bad and ugly though, I agree. For several reasons:
//!
//!   - It doesn’t compose at all. If you want to do something in between α and β, you will have to
//!     break the whole block and change every timings.
//!   - Dynamic branching will get worse and worse as time passes, since you’re going to make more and
//!     more tests.
//!   - The code is just ugly!
//!
//! Instead of writing that kind of code, we can do better:
//!
//! ```
//! use awoo::window::Window;
//!
//! let pre_alpha = Window::new(0., 1.); // do something until time passes 1.
//! let pre_beta = Window::new(1., 2.); // do something until time passes 2.
//! let pre_gamma = Window::new(2., 3.); // do something until time passes 3.
//! ```
//!
//! By default, [`Window<T>`] is just a window of time `T`. We can map actions to completely
//! do the same thing as above:
//!
//! ```
//! # use awoo::window::Window;
//! let pre_alpha = Window::new(0., 1.).map(|time| println!("time is {}", time));
//! ```
//!
//! The type of that [`MappedWindow<_>`] is determined by what you return from your closure in the
//! `map` call. Once all windows are created, you can schedule them:
//!
//! ```
//! # use awoo::window::Window;
//! use awoo::scheduler::RandomAccessScheduler;
//! use awoo::time::simple::SimpleF32TimeGenerator;
//!
//! # let pre_alpha = Window::new(0., 1.).map(|t| println!("pre α: {}", t));
//! # let pre_beta = Window::new(1., 2.).map(|t| println!("pre β: {}", t));
//! # let pre_gamma = Window::new(2., 3.).map(|t| println!("pre γ: {}", t));
//! let mut scheduler = RandomAccessScheduler::new(
//!   SimpleF32TimeGenerator::new(0., 0.1), // a generator that generates linear time starting at 0 and incrementing by 0.1
//!   vec![pre_alpha, pre_beta, pre_gamma] // our mapped windows
//! ).unwrap();
//!
//! scheduler.schedule();
//! ```
//!
//! What’s interesting is that we can get the windows from a file, for instance, and map them on the
//! fly.
//!
//! [`Window<T>`]: crate::window::Window
//! [`MappedWindow<_>`]: crate::window::MappedWindow

pub mod scheduler;
pub mod time;
pub mod window;
