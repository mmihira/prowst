use time;
use time::Duration;

pub struct Counter {
    pub counter: time::SteadyTime,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { counter: time::SteadyTime::now() }
    }

    pub fn elapsed(&self) -> time::Duration {
        time::SteadyTime::now() - self.counter
    }

    pub fn reset(&mut self) {
        self.counter = time::SteadyTime::now();
    }

    pub fn elapsed_gt(&self, msecs: i64) -> bool {
        self.elapsed() >= time::Duration::milliseconds(msecs)
    }
}
