#![feature(duration_constructors)]

use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, fs, thread};

// just some little aliases
const SECONDS_PER_HOUR: u64 = 60 * 60;
const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;

fn main() {
    let home = env::var("HOME").unwrap();
    let string = fs::read_to_string(format!("{home}/.config/sleep-timer")).unwrap();
    let mut lines = string.lines();

    let hour_offset: i64 = lines.next().unwrap().parse().unwrap();

    let mut times: Vec<(u64, u64)> = lines
        .filter_map(|str| str.split_once(':'))
        .map(|(hour, minute)| (hour.parse().unwrap(), minute.parse().unwrap()))
        .collect();

    let timer_offset = times.pop().unwrap();

    assert!(times.len() == 7);

    loop {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let days = duration.as_secs() / SECONDS_PER_DAY;
        let day_of_week = (days + 4) % 7; // 0 = Monday

        let seconds_into_day = duration.as_secs() % SECONDS_PER_DAY;

        let hour = ((seconds_into_day / SECONDS_PER_HOUR) as i64 + hour_offset) as u64;
        let minute = (seconds_into_day % SECONDS_PER_HOUR) / 60;

        let (target_hour, target_minute) = times[day_of_week as usize];

        let duration = {
            let hours = Duration::from_hours(if target_hour < hour {
                24 - (hour - target_hour)
            } else {
                target_hour - hour
            });
            if target_minute < minute {
                hours - Duration::from_mins(minute - target_minute)
            } else {
                hours + Duration::from_mins(target_minute - minute)
            }
        }
        .checked_sub(Duration::from_mins(timer_offset.0 * 60 + timer_offset.1))
        .unwrap_or(Duration::from_mins(15));

        println!("Waiting for: {} min", duration.as_secs() / 60);

        thread::sleep(duration);

        Command::new("poweroff").spawn().unwrap().wait().unwrap();
    }
}
