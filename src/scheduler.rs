// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
use crontab::Crontab;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use task::NextExecution;
use threadpool::ThreadPool;

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
  next_schedule: BinaryHeap<NextExecution>,
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
      next_schedule: BinaryHeap::new(),
    }
  }

  /*pub fn run(&self) -> ! {
    self.schedule_loop();
  }*/

  // FIXME: Clean this up, fix error semantics.
  fn pop_next_runnable_task(&self) -> Option<NextExecution> {
    /*match self.next_schedule.lock() {
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
    }*/
    None
  }

  // TODO/FIXME: Oh god, this is rough
  fn pop_runnable_task(next_schedules: &Arc<Mutex<BinaryHeap<NextExecution>>>) -> Option<NextExecution> {
    match next_schedules.lock() {
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

  /*pub fn run_parallel(&self) {
    let next_schedules = self.next_schedule.clone();
    let tasks : Arc<HashMap<String, TaskSpec<'a>>> = self.tasks.clone();

    self.threadpool.execute(move || {
      loop {

        if let Some(next_task) = Self::pop_runnable_task(&next_schedules) {
          match tasks.get(&next_task.name) {
            None => { /* This should be unreachable! */ },
            Some(task) => {

              let next = task.schedule.find_next_event().unwrap(); // FIXME

              // Reschedule
              match next_schedules.lock() {
                Err(_) => { /* Should not happen. */ },
                Ok(mut next_schedule) => {
                  let next_execution = NextExecution {
                    scheduled_time: next,
                    name: next_task.name.to_string(),
                  };
                  (*next_schedule).push(next_execution)
                },
              }
            },
          }
        }

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
  }*/

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