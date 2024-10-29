#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
use std::fmt;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Calculate total minutes from the input hours and minutes
        let total_minutes = hours * 60 + minutes;

        // Normalize total minutes to be within a day's worth of minutes
        let total_minutes = total_minutes % (24 * 60);

        // Adjust if total_minutes is negative to wrap around correctly
        let total_minutes = if total_minutes < 0 {
            total_minutes + (24 * 60)
        } else {
            total_minutes
        };

        // Calculate normalized hours and minutes
        let normalized_hours = (total_minutes / 60) % 24;
        let normalized_minutes = total_minutes % 60;

        Clock {
            hours: normalized_hours,
            minutes: normalized_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = (self.hours * 60 + self.minutes) + minutes;
        let total_minutes = total_minutes % (24 * 60);

        let total_minutes = if total_minutes < 0 {
            total_minutes + (24 * 60)
        } else {
            total_minutes
        };

        let normalized_hours = (total_minutes / 60) % 24;
        let normalized_minutes = total_minutes % 60;

        Clock {
            hours: normalized_hours,
            minutes: normalized_minutes,
        }
    }

    pub fn subtract_minutes(&self, minutes: i32) -> Self {
        self.add_minutes(-minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
  }
    fn past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
}
}
