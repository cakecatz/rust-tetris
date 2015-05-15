use time::*;

pub struct Clock {
    prev: Tm,
}

impl Clock {
    pub fn new() -> Clock {
        return Clock{ prev: now() };
    }

    pub fn check(&mut self) -> bool {
        let current_time = now();
        if self.prev.tm_sec < current_time.tm_sec {
            self.prev = current_time;
            return true;
        } else if self.prev.tm_sec == 59 && current_time.tm_sec == 0 {
            self.prev = current_time;
            return true;
        } else {
            return false;
        }
    }
}