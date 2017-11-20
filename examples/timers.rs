// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Set up some recurring tasks that will print the time on a set cron schedule.

extern crate task;
extern crate time;

use task::Scheduler;
use time::now;

pub fn main() {
  let mut scheduler = Scheduler::new(4);

  //scheduler.schedule_job("Every two minutes", "* * * * *", || {
  scheduler.schedule_job("Every 2 minutes", "0,2,4,6,8,10,12,14,16,18,20,24,26,28,30,32,34,36,38,40,42,44,46,48,50,52,54,56,58 * * * *", || {
    let time = now();
    println!("Executes two minutes: {}", time.rfc3339());
  });

  scheduler.schedule_job("Every 1 minutes", "* * * * *", || {
    let time = now();
    println!("Executes every minute: {}", time.rfc3339());
  });

  println!("Running scheduler example. {}", now().rfc3339());
  scheduler.run();
}