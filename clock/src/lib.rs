#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock{hour: 0, minute: 0};
        clock.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut clock = Clock{hour: self.hour, minute: self.minute};

        let hours = minutes / 60;
        clock.hour += hours;

        clock.minute += minutes - hours * 60;
        if clock.minute >= 60 {
            clock.hour += 1;
            clock.minute -= 60;
        }
        else if clock.minute < 0 {
            clock.hour -= 1;
            clock.minute += 60;
        }

        if clock.hour >= 24 {
            let full_days = clock.hour / 24;
            clock.hour -= full_days * 24;
        }

        if clock.hour < 0 {
            let full_days = -clock.hour / 24;
            clock.hour += full_days * 24;

            if clock.hour < 0 {
                clock.hour += 24;
            }
        }

        clock
    }

    pub fn to_string(&self) -> String {
        return format!("{:02}:{:02}", self.hour, self.minute);
    }
}
