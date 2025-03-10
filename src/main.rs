#![feature(duration_constructors)]

use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs, thread};

// just some little aliases
const SECONDS_PER_HOUR: u64 = 60 * 60;
const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;

fn main() {
    // Get the config file
    let config_string =
        fs::read_to_string(format!("{}/.config/sleep-timer", env::var("HOME").unwrap())).unwrap();

    let mut lines = config_string.lines();

    let hour_offset: i64 = lines.next().unwrap().parse().unwrap();

    let mut times: Vec<(u64, u64)> = lines
        .filter_map(|str| str.split_once(':'))
        .map(|(hour, minute)| (hour.parse().unwrap(), minute.parse().unwrap()))
        .collect();

    let timer_offset = times.pop().unwrap();

    assert!(times.len() == 7);

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let days = time.as_secs() / SECONDS_PER_DAY;
    let day_of_week = (days + 4) % 7; // 0 = Monday

    let seconds_into_day = time.as_secs() % SECONDS_PER_DAY;

    let hour = ((seconds_into_day / SECONDS_PER_HOUR) as i64 + hour_offset) as u64;
    let minute = (seconds_into_day % SECONDS_PER_HOUR) / 60;

    let (target_hour, target_minute) = times[day_of_week as usize];

    // Calculate duration until target hour
    let duration = {
        let hours = Duration::from_hours(if target_hour > hour {
            target_hour - hour
        } else {
            24 - (hour - target_hour)
        });
        if target_minute < minute {
            hours - Duration::from_mins(minute - target_minute)
        } else {
            hours + Duration::from_mins(target_minute - minute)
        }
    }
    // Apply timer (time-zone) offset
    .checked_sub(Duration::from_mins(timer_offset.0 * 60 + timer_offset.1))
    // If were past the sleep time already, wait for 15 minutes
    .unwrap_or(Duration::from_mins(15));

    println!("Waiting for: {} min", duration.as_secs() / 60);

    if env::args().nth(1) == Some(String::from("--dry-run")) {
        return;
    }

    thread::sleep(duration);

    println!("Powering off the system now");
    Command::new("poweroff").spawn().unwrap().wait().unwrap();
}
