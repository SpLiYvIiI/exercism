use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    pub minutes: i32
}

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { 
            minutes: (((hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY) + MINUTES_PER_DAY) % MINUTES_PER_DAY
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.minutes / MINUTES_PER_HOUR, self.minutes % MINUTES_PER_HOUR)
    }
}
