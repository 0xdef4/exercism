use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour : i32,
    minute : i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hour, minute) = Self::calculate_hour_minute(hours, minutes);

        Clock {
            hour,
            minute
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hour, minute) = Self::calculate_hour_minute(self.hour, self.minute + minutes);

        Clock {
            hour,
            minute
        }
    }

    fn calculate_hour_minute(_hour: i32, _minute: i32) ->(i32, i32) {
        let hour;
        let minute;

        let total_min = _hour * 60 +  _minute;

        if total_min > 0 {
            hour = (total_min / 60) % 24;
            minute = total_min % 60;
        } else {
           let temp = (24 * 60) + (((total_min / 60) % 24) * 60  +  (total_min % 60));
            hour = (temp / 60) % 24;
            minute = temp % 60;
        }

        (hour, minute)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        format!("{:02}:{:02}", clock.hour, clock.minute)
    }
}
