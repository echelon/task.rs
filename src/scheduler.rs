// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
use threadpool::ThreadPool;
use std::thread;
use std::time::Duration;
use crontab::Crontab;
use std::sync::RwLock;

struct TaskSpec {
  schedule: Crontab,
  //handle: Box<FnMut()>,
}

/// Scheduler manages scheduling of new jobs and maintains a threadpool
/// upon which the scheduled jobs run.
pub struct Scheduler {
  /// The threadpool.
  threadpool: ThreadPool,
  //tasks: Vec<TaskSpec>,
}

impl Scheduler {

  // TODO: Alternate CTOR using lifetimes to share an externally created threadpool.
  /// Create a new scheduler.
  /// Running the scheduler consumes a thread, so allocate a thread pool
  /// capacity of N+1 to have a capacity of N.
  pub fn new(pool_size: usize) -> Scheduler {
    Scheduler {
      threadpool: ThreadPool::new(pool_size),
      //tasks: Vec::new(),
    }
  }

  pub fn run(&self) -> ! {
    self.schedule_loop();
  }

  pub fn run_parallel(&self) {
    /*match self.threadpool.read() {
      Ok(lock) => {
        lock.execute(|| {
          self.schedule_loop();
        });
      },
      _ => {},
    }*/
    self.threadpool.execute(|| {
      loop {
        // TODO
        thread::sleep(Duration::from_secs(1))
      }
    })
  }

  fn schedule_job<F>(&self, schedule: &str, function: F)
      where F: FnMut() {

  }
}