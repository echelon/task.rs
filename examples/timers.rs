// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Set up some recurring tasks that will print the time on a set cron schedule.

extern crate task;
extern crate time;

use task::Scheduler;
use time::now;

pub fn main() {
  let mut scheduler = Scheduler::new(4);

  scheduler.schedule_job("Every 1 minutes", "* * * * *", || {
    println!("Executes every minute: {}", now().rfc3339());
  });

  scheduler.schedule_job("Every 2 minutes", &every_n_minutes(2), || {
    println!("Executes two minutes: {}", now().rfc3339());
  });

  scheduler.schedule_job("Every 3 minutes", &every_n_minutes(3), || {
    println!("Executes three minutes: {}", now().rfc3339());
  });

  scheduler.schedule_job("Every 5 minutes", &every_n_minutes(5), || {
    println!("Executes five minutes: {}", now().rfc3339());
  });

  println!("Running scheduler example: {}", now().rfc3339());
  scheduler.run();
}

fn every_n_minutes(divisor: usize) -> String {
  let mut minutes = Vec::new();
  for i in 0..60 {
    if i % divisor == 0 {
      minutes.push(i.to_string());
    }
  }

  minutes.join(",") + " * * * *"
}