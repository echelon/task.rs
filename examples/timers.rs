// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Set up some recurring tasks that will print the time on a set cron schedule.

extern crate task;
extern crate time;

use task::Scheduler;
use time::now;

pub fn main() {
  let mut scheduler = Scheduler::new(4);

  scheduler.schedule_job("Every second", "* * * * *", || {
    let time = now();
    println!("Executes every second: {}", time.rfc3339());
  });

  scheduler.schedule_job("Every two seconds", "0,2,4,6,8,10,12,14,16,18,20,22,24 * * * *", || {
    let time = now();
    println!("Executes two seconds: {}", time.rfc3339());
  });

  println!("Running scheduler example.");
  scheduler.run();
}