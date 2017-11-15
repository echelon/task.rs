use crontab::Crontab;
use crontab::Tm;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;

/// A task with a schedule.
pub (crate) struct RunnableTask {
  pub schedule: Crontab,
  pub handle: Box<FnMut() + Send + Sync>, // TODO: Drop 'Sync' ?
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

// Orders are reversed so that BinaryHeap is a min-heap.
impl PartialOrd for NextExecution {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.scheduled_time.partial_cmp(&other.scheduled_time)
        .map(|order| order.reverse())
  }

  fn lt(&self, other: &Self) -> bool {
    !self.scheduled_time.lt(&other.scheduled_time) // Reversed
  }

  fn le(&self, other: &Self) -> bool {
    !self.scheduled_time.le(&other.scheduled_time) // Reversed
  }

  fn gt(&self, other: &Self) -> bool {
    !self.scheduled_time.gt(&other.scheduled_time) // Reversed
  }

  fn ge(&self, other: &Self) -> bool {
    !self.scheduled_time.ge(&other.scheduled_time) // Reversed
  }
}

// Orders are reversed so that BinaryHeap is a min-heap.
impl Ord for NextExecution {
  fn cmp(&self, other: &Self) -> Ordering {
    self.scheduled_time.cmp(&other.scheduled_time).reverse()
  }

  fn max(self, other: Self) -> Self where Self: Sized {
    // Reversed
    if self.scheduled_time.gt(&other.scheduled_time) {
      other
    } else {
      self
    }
  }

  fn min(self, other: Self) -> Self where Self: Sized {
    // Reversed
    if self.scheduled_time.le(&other.scheduled_time) {
      other
    } else {
      self
    }
  }
}
