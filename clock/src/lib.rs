use std::fmt;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Clock {
    hours : i32,
    minutes : i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock: Clock = Self {
            hours,
            minutes,
        };
        clock.convert_time();
        clock
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.convert_time();
        Self {
            hours : self.hours,
            minutes : self.minutes,
        }
    }

    fn convert_time(&mut self){
        let inc_hour = ((self.minutes as f32) / 60.0).floor();
        self.hours  += inc_hour as i32;
        self.hours = self.hours % 24;
        if self.hours < 0 {
            self.hours += 24;
        }
        self.minutes = self.minutes % 60;
        if self.minutes < 0 {
            self.minutes += 60;
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
