// Copyright (c) 2017 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
use crontab::Crontab;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use task::NextExecution;
use std::sync::Arc;
use std::sync::Mutex;
use task::RunnableTask;
use threadpool::ThreadPool;

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

  pub fn schedule_job<F>(&mut self, name: &str, schedule: &str, function: F)
    where F: FnMut() + Send + Sync + 'static {

    let crontab = Crontab::parse(schedule).ok().unwrap(); // FIXME

    let taskspec = RunnableTask {
      schedule: crontab,
      handle: Box::new(function)
    };

    //self.tasks.insert(name.into(), taskspec); //TODO
  }

  pub fn run(&mut self) -> ! {
    loop {
      // TODO: Schedule everything that is unscheduled.

      let tasks = self.tasks.clone();

      if let Some(next_task) = self.pop_next_runnable_task() {

        self.threadpool.execute(move || {
          let mut tasks2 = tasks.lock().unwrap();

          match tasks2.get_mut(&next_task.name) {
            None => { /* This should be unreachable! */ },
            Some(task) => {
              let next = task.schedule.find_next_event().unwrap(); // FIXME

              // Reschedule
              let next_execution = NextExecution {
                scheduled_time: next,
                name: next_task.name.to_string(),
              };

              //self.next_schedule.push(next_execution);

              (*task.handle)();
            },
          }
        });
      }

      thread::sleep(Duration::from_secs(1));
    }
  }

  // FIXME: Clean this up, fix error semantics.
  fn pop_next_runnable_task(&mut self) -> Option<NextExecution> {
    match self.next_schedule.peek() {
      None => return None,
      Some(task) => {
        // TODO - check time.
      }
    }
    self.next_schedule.pop()
  }
}