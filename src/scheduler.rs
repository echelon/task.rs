// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
use threadpool::ThreadPool;
use std::thread;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::time::Duration;
use std::sync::Mutex;
use crontab::Crontab;
use std::sync::RwLock;
use task::NextExecution;

struct TaskSpec <'a> {
  schedule: Crontab,
  handle: Box<FnMut() + 'a>,
}

/// Scheduler manages scheduling of new jobs and maintains a threadpool
/// upon which the scheduled jobs run.
pub struct Scheduler <'a> {
  /// The threadpool.
  threadpool: ThreadPool,
  tasks: HashMap<String, TaskSpec<'a>>,
  next_schedule: Mutex<BinaryHeap<NextExecution>>,
}

impl <'a> Scheduler <'a> {

  // TODO: Alternate CTOR using lifetimes to share an externally created threadpool.
  /// Create a new scheduler.
  /// Running the scheduler consumes a thread, so allocate a thread pool
  /// capacity of N+1 to have a capacity of N.
  pub fn new(pool_size: usize) -> Scheduler<'a> {
    Scheduler {
      threadpool: ThreadPool::new(pool_size),
      tasks: HashMap::new(),
      next_schedule: Mutex::new(BinaryHeap::new()),
    }
  }

  /*pub fn run(&self) -> ! {
    self.schedule_loop();
  }*/

  // FIXME: Clean this up, fix error semantics.
  fn pop_next_runnable_task(&self) -> Option<NextExecution> {
    match self.next_schedule.lock() {
      Err(_) => {
        return None;
      },
      Ok(mut next_schedule) => {
        match next_schedule.peek() {
          None => return None,
          Some(task) => {
            
          }
        }
        return next_schedule.pop();
      },
    }
  }

  pub fn run_parallel(&self) {
    self.threadpool.execute(|| {
      loop {
        /*if let Some(task) = self.next_schedule.peek() {
          if task.scheduled_time > current_time {
            thread::sleep(Duration::from_secs(1))
            continue;
          }

          let task  = self.next_schedule.pop();
        }*/



        thread::sleep(Duration::from_secs(1))
      }
    })
  }

  fn schedule_job<F>(&mut self, name: &str, schedule: &str, function: F)
      where F: FnMut() + 'a {

    let crontab = Crontab::parse(schedule).ok().unwrap();

    let taskspec = TaskSpec {
      schedule: crontab,
      handle: Box::new(function)
    };
    self.tasks.insert(name.into(), taskspec);
  }
}