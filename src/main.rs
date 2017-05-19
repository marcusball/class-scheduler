extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::str::FromStr;
use std::ops::Range;

#[derive(Debug, Deserialize)]
struct ScheduleOptions {
    /// List of all possible periods
    periods: Vec<u8>,

    /// List of all possible classes
    classes: Vec<Class>,
}

#[derive(Debug, Deserialize)]
struct Class {
    /// The name of this class
    name: String,

    /// The periods each available session takes up
    sections: Vec<Section>,
}

type Section = Vec<String>;

#[derive(Debug, PartialEq)]
struct Period(Day, u8);

struct Session {
    class: String,

    periods: Vec<Period>,
}

#[derive(Debug, PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Day {
    fn from_char(c: char) -> Day {
        match c {
            'M' => Day::Monday,
            'T' => Day::Tuesday,
            'W' => Day::Wednesday,
            'R' => Day::Thursday,
            'F' => Day::Friday,
            c => panic!("Unknown day character '{}'!", c),
        }
    }
}

fn parse_period_string(period: &str) -> Vec<Period> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([MTWRF]{1,5})(\\d{1,2})(?:-(\\d{1,2}))?").unwrap();
    }

    let mut periods = Vec::new();

    for cap in RE.captures_iter(period) {
        // Get the letters representing the days of the string. eg: "MWF"
        let days = &cap[1];

        // The time period of this session, or the beginning time period of a multi-period session
        let start_period = u8::from_str(&cap[2]).unwrap();

        // Either the same as `start_period` or the end period of a multi-period session
        let end_period = cap.get(3)
            .map(|end| u8::from_str(end.as_str()).unwrap())
            .unwrap_or(start_period);

        // Range over all of the periods this session spans 
        let period_range = Range {
            start: start_period,
            end: end_period + 1,
        };

        for period in period_range {
            for day_char in days.chars() {
                periods.push(Period(Day::from_char(day_char), period));
            }
        }
    }

    periods
}

fn main() {
    let mut file = File::open("Classes.toml").expect("Failed to find Classes.toml!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read Classes.toml!");

    let schedule_options: ScheduleOptions = toml::from_str(contents.as_str())
        .expect("Error parsing Classes.toml!");

    for class in &schedule_options.classes {
        for section in &class.sections {
            for session in section {
                parse_period_string(session.as_str());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_period_string() {
        assert_eq!(
            parse_period_string("MWF3"),
            vec![
                Period(Day::Monday, 3),
                Period(Day::Wednesday, 3),
                Period(Day::Friday, 3)
            ]
        );

        assert_eq!(
            parse_period_string("TR5-6"),
            vec![
                Period(Day::Tuesday,5),
                Period(Day::Thursday, 5),
                Period(Day::Tuesday,6),
                Period(Day::Thursday, 6),
            ]
        );
    }
}