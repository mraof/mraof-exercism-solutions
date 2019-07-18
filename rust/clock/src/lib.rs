use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: roll_over(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: roll_over(self.minutes + minutes)
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

const MAX_MINUTES: i32 = 24 * 60;

fn roll_over(minutes: i32) -> i32 {
    ((minutes % MAX_MINUTES) + MAX_MINUTES) % MAX_MINUTES
}
