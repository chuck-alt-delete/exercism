use std::fmt;
use num_integer::Integer;

#[derive(Debug)]
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
        let (mut h, mut m): (i32, i32) = (minutes_sum).div_rem(&60);
        h += self.hours;
        if minutes_sum < 0 {
            h -= 1;
            m += 60;
        }
        Clock::new(h, m)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let normalized_self: Clock = Clock::new(self.hours, self.minutes);
        let normalized_other: Clock = Clock::new(other.hours, other.minutes);
        normalized_self.hours == normalized_other.hours && normalized_self.minutes == normalized_other.minutes
    }
}