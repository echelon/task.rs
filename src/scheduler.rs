// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
use crontab::Crontab;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use task::NextExecution;
use task::RunnableTask;
use threadpool::ThreadPool;
use time::now;

/// Scheduler manages scheduling of new jobs and maintains a threadpool
/// upon which the scheduled jobs run.
pub struct Scheduler {
  /// The threadpool.
  threadpool: ThreadPool,
  tasks: Arc<Mutex<HashMap<String, RunnableTask>>>,
  next_schedule: BinaryHeap<NextExecution>,
}

impl <'a> Scheduler {
  // TODO: Alternate CTOR to share an externally created thread pool.
  /// Create a new scheduler.
  pub fn new(pool_size: usize) -> Scheduler {
    Scheduler {
      threadpool: ThreadPool::new(pool_size),
      tasks: Arc::new(Mutex::new(HashMap::new())),
      next_schedule: BinaryHeap::new(),
    }
  }

  // TODO: Make lifetime bound on closure non-static.
  /// Schedule a new job for execution.
  pub fn schedule_job<F>(&mut self, name: &str, schedule: &str, function: F)
    where F: FnMut() + Send + Sync + 'static {

    let crontab = Crontab::parse(schedule).ok().unwrap(); // FIXME

    let taskspec = RunnableTask {
      schedule: crontab,
      handle: Box::new(function)
    };

    if let Ok(mut t) = self.tasks.lock() {
      t.insert(name.into(), taskspec); // FIXME CLEANUP
    }
  }

  /// Run the jobs.
  pub fn run(&mut self) -> ! {
    loop {
      self.schedule_jobs();
      self.run_applicable_jobs();
      thread::sleep(Duration::from_secs(1));
    }
  }

  // FIXME: inefficient
  fn schedule_jobs(&mut self) {
    let mut already_scheduled_jobs = HashSet::new();

    for scheduled in self.next_schedule.iter() {
      already_scheduled_jobs.insert(scheduled.name.clone());
    }

    // FIXME CLEANUP
    if let Ok(tasks2) = self.tasks.lock() {
      for (job_name, runnable_task) in tasks2.iter() {
        if !already_scheduled_jobs.contains(job_name) {
          let next = runnable_task.schedule.find_next_event().unwrap(); // FIXME

          let next_execution = NextExecution {
            scheduled_time: next,
            name: job_name.to_string(),
          };

          self.next_schedule.push(next_execution);
        }
      }
    }
  }

  fn run_applicable_jobs(&mut self) {
    let tasks = self.tasks.clone();

    while let Some(next_task) = self.pop_next_runnable_task() {
      let tasks2 = tasks.clone();

      self.threadpool.execute(move || {
        let mut tasks3 = tasks2.lock().unwrap(); // TODO

        match tasks3.get_mut(&next_task.name) {
          None => { /* This should be unreachable! */ },
          Some(task) => {
            let next = task.schedule.find_next_event().unwrap(); // FIXME

            (*task.handle)();
          },
        }
      });
    }
  }

  // FIXME: Clean this up, fix error semantics.
  fn pop_next_runnable_task(&mut self) -> Option<NextExecution> {
    match self.next_schedule.peek() {
      None => return None,
      Some(task) => {
        // TODO: Handle timezones.
        // TODO: Fake clock injection for testing.
        let time = now();
        if time < task.scheduled_time {
          return None;
        }
      }
    }
    self.next_schedule.pop()
  }
}