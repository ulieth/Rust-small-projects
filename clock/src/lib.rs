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
    #[test]
    fn past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }
    #[test]
    fn hour_rolls_over_continuously() {
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }
    #[test]
    fn sixty_minutes_is_next_hour() {
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }
    #[test]
    fn negative_hour() {
        assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    }
    #[test]
    fn negative_hour_rolls_over() {
        assert_eq!(Clock::new(-25, 0).to_string(), "23:00");
    }
    #[test]
    fn negative_hour_rolls_over_continuously() {
        assert_eq!(Clock::new(-91, 0).to_string(), "05:00");
    }
    #[test]
    fn negative_minutes() {
        assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    }
    #[test]
    fn negative_minutes_roll_over() {
        assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    }
    #[test]
    fn negative_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
    }
    #[test]
    fn add_minutes() {
        let clock = Clock::new(10, 0).add_minutes(3);
        assert_eq!(clock.to_string(), "10:03");
    }
    #[test]
    fn add_no_minutes() {
        let clock = Clock::new(6, 41).add_minutes(0);
        assert_eq!(clock.to_string(), "06:41");
    }
}
