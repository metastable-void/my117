
use crate::voice::Part;
use crate::chrono::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn validate_hour(hour: u8) -> Result<u8, &'static str> {
        if hour > 23 {
            Err("Hour must be less than 24")
        } else {
            Ok(hour)
        }
    }

    fn validate_minute(minute: u8) -> Result<u8, &'static str> {
        if minute > 59 {
            Err("Minute must be less than 60")
        } else {
            Ok(minute)
        }
    }

    fn validate_second(second: u8) -> Result<u8, &'static str> {
        if second > 59 {
            Err("Second must be less than 60")
        } else {
            Ok(second)
        }
    }

    pub fn new(hour: u8, minute: u8, second: u8) -> Result<Time, &'static str> {
        Ok(Time {
            hour: Time::validate_hour(hour)?,
            minute: Time::validate_minute(minute)?,
            second: Time::validate_second(second)?,
        })
    }

    pub fn now() -> Time {
        let now = Local::now();
        Time {
            hour: now.hour() as u8,
            minute: now.minute() as u8,
            second: now.second() as u8,
        }
    }

    pub fn from_local(local: DateTime<Local>) -> Time {
        Time {
            hour: local.hour() as u8,
            minute: local.minute() as u8,
            second: local.second() as u8,
        }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    fn second_round_up_to_10_second(&self) -> u8 {
        let second = self.second;
        second + 10 - second % 10
    }

    fn minute_round_up_to_10_second(&self) -> u8 {
        let second = self.second_round_up_to_10_second();
        if second == 60 {
            self.minute + 1
        } else {
            self.minute
        }
    }

    fn hour_round_up_to_10_second(&self) -> u8 {
        let minute = self.minute_round_up_to_10_second();
        if minute == 60 {
            (self.hour + 1) % 24
        } else {
            self.hour
        }
    }

    fn tuple_round_up_to_10_second(&self) -> (u8, u8, u8) {
        let hour = self.hour_round_up_to_10_second();
        let minute = self.minute_round_up_to_10_second();
        let second = self.second_round_up_to_10_second();
        (hour, minute % 60, second % 60)
    }

    pub fn get_next_voice_parts(&self) -> Vec<Part> {
        let (hour, minute, second) = self.tuple_round_up_to_10_second();
        let mut result = Vec::new();
        if hour == 12 && minute == 0 && second == 0 {
            result.push(Part::VoiceShogoWo);
            result.push(Part::VoiceOshiraseshimasu);
            return result;
        }
        if hour < 12 {
            result.push(Part::VoiceGozen);
        } else {
            result.push(Part::VoiceGogo);
        }
        let hour12 = hour % 12;
        match hour12 {
            0 => result.push(Part::Voice0ji),
            1 => result.push(Part::Voice1ji),
            2 => result.push(Part::Voice2ji),
            3 => result.push(Part::Voice3ji),
            4 => result.push(Part::Voice4ji),
            5 => result.push(Part::Voice5ji),
            6 => result.push(Part::Voice6ji),
            7 => result.push(Part::Voice7ji),
            8 => result.push(Part::Voice8ji),
            9 => result.push(Part::Voice9ji),
            10 => result.push(Part::Voice10ji),
            11 => result.push(Part::Voice11ji),
            _ => unreachable!(),
        }
        
        if minute != 0 {
            let ten_minute = minute / 10;
            let minute = minute % 10;
            if ten_minute != 0 {
                if minute == 0 {
                    match ten_minute {
                        1 => result.push(Part::Voice10fun),
                        2 => result.push(Part::Voice20fun),
                        3 => result.push(Part::Voice30fun),
                        4 => result.push(Part::Voice40fun),
                        5 => result.push(Part::Voice50fun),
                        _ => unreachable!(),
                    }
                } else {
                    match ten_minute {
                        1 => result.push(Part::Voice10),
                        2 => result.push(Part::Voice20),
                        3 => result.push(Part::Voice30),
                        4 => result.push(Part::Voice40),
                        5 => result.push(Part::Voice50),
                        _ => unreachable!(),
                    }
                }
            }

            if minute != 0 {
                match minute {
                    1 => result.push(Part::Voice1fun),
                    2 => result.push(Part::Voice2fun),
                    3 => result.push(Part::Voice3fun),
                    4 => result.push(Part::Voice4fun),
                    5 => result.push(Part::Voice5fun),
                    6 => result.push(Part::Voice6fun),
                    7 => result.push(Part::Voice7fun),
                    8 => result.push(Part::Voice8fun),
                    9 => result.push(Part::Voice9fun),
                    _ => unreachable!(),
                }
            }
        }
        if second == 0 {
            result.push(Part::VoiceChodoWo);
        } else {
            let ten_second = second / 10;
            
            if ten_second != 0 {
                match ten_second {
                    1 => result.push(Part::Voice10byoWo),
                    2 => result.push(Part::Voice20byoWo),
                    3 => result.push(Part::Voice30byoWo),
                    4 => result.push(Part::Voice40byoWo),
                    5 => result.push(Part::Voice50byoWo),
                    _ => unreachable!(),
                }
            }
        }

        result.push(Part::VoiceOshiraseshimasu);

        result
    }
}

impl From<DateTime<Local>> for Time {
    fn from(local: DateTime<Local>) -> Time {
        Time::from_local(local)
    }
}
