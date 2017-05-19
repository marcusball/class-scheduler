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
use std::collections::HashMap;

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

/// A section, defined by a list of strings denoting days/periods
/// 
/// # Example
///
/// ```
/// // A class in period 4 on Mon, Wed, and Fri,
/// // and periods 5 and 6 on Tuesday. 
/// let section: Section = vec!["MWF4", "T5-6"];
/// ```
type Section = Vec<String>;

/// A pair of a single day and the numeric period 
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Period(Day, u8);

struct Session {
    /// The name of the class for this session
    class: String,

    /// The periods this session spans
    periods: Vec<Period>,
}

/// Just a map of Periods each day and the class during that period 
type Schedule = HashMap<Period, String>;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

/// Gets all of the days/periods that the specified `section` spans. 
///
/// # Parameters
///
/// - section: Vector of strings denoting the days and periods of a section
#[allow(ptr_arg)]
fn get_section_periods(section: &Section) -> Vec<Period> {
    let mut periods = Vec::new();

    for session in section {
        periods.append(&mut parse_period_string(session.as_str()));
    }

    periods
}

/// Begins the recursive process of generating all possible schedules
///
/// # Paramters 
///
/// - options:     ScheduleOptions reference containing all classes, and available periods 
fn generate_schedules(options: &ScheduleOptions) {
    generate_schedule_recursive(options, 0, Schedule::new());
}

/// Recursive function for generating a schedule
///
/// # Paramters 
///
/// - options:     ScheduleOptions reference containing all classes, and available periods 
/// - class_index: The index of the class from the classes Vec to use for this level of recursion
///                Should begin at zero. 
/// - schedule:    The currently generated schedule at this level of recursion.
fn generate_schedule_recursive(options: &ScheduleOptions, class_index: usize, schedule: Schedule) {
    // If we've iterated over all available classes
    if class_index == options.classes.len() {
        // Print the schedule and complete this recursion 
        print_schedule(options, schedule);
        return;
    }

    let class = &options.classes[class_index];

    // Iterate over all of the sections available for this class
    for section in &class.sections {
        let mut new_schedule = schedule.clone();
        let mut has_conflict = false;

        // Get the days/periods this section spans during the week
        let periods = get_section_periods(section);

        // Attempt to add all of this section's periods to the schedule
        for period in periods.into_iter() {
            let conflict = new_schedule.insert(period, class.name.clone());

            // If there was already a class during that day/period, then there's a conflict
            if conflict.is_some() {
                has_conflict = true;
                break;
            }
        }

        // If there's no conflict, continue and try to add the next class to the schedule
        if !has_conflict {
            generate_schedule_recursive(options, class_index + 1, new_schedule);
        }
    }
}

/// Prints a schedule in a nicely formatted table
fn print_schedule(options: &ScheduleOptions, schedule: Schedule) {
    let days = vec![Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday, Day::Friday];

    println!("┌───┬───────────┬───────────┬───────────┬───────────┬───────────┐");
    print!("│ # │");
    for day in &days {
        print!(" {:^9} │", format!("{:?}", day));
    }
    println!();
    println!("├───┼───────────┼───────────┼───────────┼───────────┼───────────┤");
    
    for period in &options.periods {
        print!("│{:2} │", period);
        for day in &days {
            match schedule.get(&Period(day.clone(), *period)) {
                Some(class) => print!(" {:^9} │", class),
                None => print!(" {:^9} │", " ")
            } 
        }
        println!();
    }

    println!("└───┴───────────┴───────────┴───────────┴───────────┴───────────┘");
    println!();
}

fn main() {
    let mut file = File::open("Classes.toml").expect("Failed to find Classes.toml!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read Classes.toml!");

    let schedule_options: ScheduleOptions = toml::from_str(contents.as_str())
        .expect("Error parsing Classes.toml!");

    generate_schedules(&schedule_options);
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

    #[test]
    fn test_get_section_periods() {
        assert_eq!(
            get_section_periods(&vec!["MWF3".to_string()]),
            vec![
                Period(Day::Monday, 3),
                Period(Day::Wednesday, 3),
                Period(Day::Friday, 3)
            ]
        );

        assert_eq!(
            get_section_periods(&vec![
                "MWF3".to_string(),
                "TR3".to_string()
            ]),
            vec![
                Period(Day::Monday, 3),
                Period(Day::Wednesday, 3),
                Period(Day::Friday, 3),
                Period(Day::Tuesday, 3),
                Period(Day::Thursday, 3)
            ]
        );
    }
}