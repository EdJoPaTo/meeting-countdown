use chrono::{DateTime, Local, Timelike};
use std::thread::sleep;
use std::time::Duration;

use crate::display::Display;
use crate::math::calc_relative_position;
use crate::remaining::Remaining;

pub const TIMEFORMAT: &str = "%_H:%M:%S";

pub fn timeloop(
    start: &DateTime<chrono::Local>,
    end: &DateTime<chrono::Local>,
    mut displays: Vec<Box<dyn Display>>,
) {
    if let Ok(duration) = start.signed_duration_since(Local::now()).to_std() {
        println!("wait till start");
        for d in &mut displays {
            if let Err(err) = d.clear() {
                println!("Display Error {err}");
            }
        }
        sleep(duration);
    }

    loop {
        let now = Local::now();
        let Ok(duration) = (*end - now).to_std() else {
            break;
        };

        let remaining: Remaining = duration.into();
        let position = calc_relative_position(start.timestamp(), end.timestamp(), now.timestamp());

        let remaining_seconds = duration.as_secs_f32();
        println!(
            "{} {:8.1} sec {:6.1} min {:2.2}% {:?}",
            now.format(TIMEFORMAT),
            remaining_seconds,
            remaining_seconds / 60.0,
            position * 100.0,
            remaining,
        );

        let update_interval = remaining.update_interval();

        for d in &mut displays {
            if let Err(err) = d.show_remaining(position, remaining) {
                println!("Display Error {err}");
            }
        }

        sleep_until_second(update_interval);
    }

    println!("{} end!", Local::now().format(TIMEFORMAT));
}

fn sleep_until_second(modulo: u32) {
    let now = Local::now();
    let remaining_nanoseconds = 1_000_000_000 - now.nanosecond();

    let current_second = now.second();
    let remaining_seconds = modulo - (current_second % modulo) - 1;

    sleep(Duration::new(
        u64::from(remaining_seconds),
        remaining_nanoseconds,
    ));
}
