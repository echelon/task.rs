// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Task.rs is a library for running lightweight tasks on a schedule.
#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(unused_qualifications)]
#![deny(unused_variables)]

#[macro_use]
extern crate log;

extern crate crontab;
extern crate time;
extern crate threadpool;

mod scheduler;
mod task;

pub use scheduler::Scheduler;
