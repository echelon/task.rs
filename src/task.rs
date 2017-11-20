use crontab::Crontab;
use crontab::Tm;
use std::cmp::Ordering;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::cmp::PartialOrd;

/// A task with a schedule.
pub (crate) struct RunnableTask<'a> {
  pub schedule: Crontab,
  pub handle: Box<FnMut() + 'a>,
}

/// The next execution of a named task.
pub (crate) struct NextExecution {
  pub scheduled_time: Tm,
  pub name: String,
}

impl PartialEq for NextExecution {
  fn eq(&self, other: &Self) -> bool {
    self.scheduled_time.eq(&other.scheduled_time)
  }

  fn ne(&self, other: &Self) -> bool {
    self.scheduled_time.ne(&other.scheduled_time)
  }
}

impl Eq for NextExecution {}

impl PartialOrd for NextExecution {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.scheduled_time.partial_cmp(&other.scheduled_time)
  }

  fn lt(&self, other: &Self) -> bool {
    self.scheduled_time.lt(&other.scheduled_time)
  }

  fn le(&self, other: &Self) -> bool {
    self.scheduled_time.le(&other.scheduled_time)
  }

  fn gt(&self, other: &Self) -> bool {
    self.scheduled_time.gt(&other.scheduled_time)
  }

  fn ge(&self, other: &Self) -> bool {
    self.scheduled_time.ge(&other.scheduled_time)
  }
}

impl Ord for NextExecution {
  fn cmp(&self, other: &Self) -> Ordering {
    self.scheduled_time.cmp(&other.scheduled_time)
  }

  fn max(self, other: Self) -> Self where Self: Sized {
    if self.scheduled_time.gt(&other.scheduled_time) {
      self
    } else {
      other
    }
  }

  fn min(self, other: Self) -> Self where Self: Sized {
    if self.scheduled_time.le(&other.scheduled_time) {
      self
    } else {
      other
    }
  }
}
