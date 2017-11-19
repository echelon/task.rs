use crontab::Tm;
use std::cmp::Ordering;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::cmp::PartialOrd;

pub (crate) struct NextExecution {
  time: Tm,
}

impl PartialEq for NextExecution {
  fn eq(&self, other: &Self) -> bool {
    self.time.eq(&other.time)
  }

  fn ne(&self, other: &Self) -> bool {
    self.time.ne(&other.time)
  }
}

impl Eq for NextExecution {
}

impl PartialOrd for NextExecution {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.time.partial_cmp(&other.time)
  }

  fn lt(&self, other: &Self) -> bool {
    self.time.lt(&other.time)
  }

  fn le(&self, other: &Self) -> bool {
    self.time.le(&other.time)
  }

  fn gt(&self, other: &Self) -> bool {
    self.time.gt(&other.time)
  }

  fn ge(&self, other: &Self) -> bool {
    self.time.ge(&other.time)
  }
}

impl Ord for NextExecution {
  fn cmp(&self, other: &Self) -> Ordering {
    self.time.cmp(&other.time)
  }

  fn max(self, other: Self) -> Self where Self: Sized {
    if self.time.gt(&other.time) {
      self
    } else {
      other
    }
  }

  fn min(self, other: Self) -> Self where Self: Sized {
    if self.time.le(&other.time) {
      self
    } else {
      other
    }
  }
}
