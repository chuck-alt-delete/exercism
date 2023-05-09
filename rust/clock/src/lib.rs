use std::fmt;
use num_integer::Integer;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        // Ignore multiples of 1440 minutes (1 day), since clocks are equivalent modulo days
        minutes = minutes % 1440;
        // consolidate hours from minutes
        let (extra_hours, mut minutes): (i32, i32) = minutes.div_rem(&60);
        hours += extra_hours;
        // ignore multiples of 24 hours (1 day)
        hours = hours % 24; 
        // convert negatives
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        if hours < 0 {
            hours += 24;
        }
        Clock {
            hours: hours,
            minutes: minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_sum: i32 = self.minutes + minutes;
        let (mut h, m): (i32, i32) = (minutes_sum).div_rem(&60);
        h += self.hours;
        Clock::new(h, m)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
