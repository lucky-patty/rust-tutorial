use std::fmt;

// Use this so that it can be compared
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        // Calculate from day to minutes 
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        
        Self {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print 2 digit
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }

}

// Add debugging
impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clock {{ hours: {}, minutes: {} }}", self.hours, self.minutes)
    }
}

