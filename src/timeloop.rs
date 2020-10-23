use chrono::{DateTime, Local, Timelike};
use std::thread::sleep;
use std::time::Duration;

use crate::publish::publish;
use crate::topic::Topic;

mod math;

pub const TIMEFORMAT: &str = "%_H:%M:%S";

pub fn timeloop(
    start: DateTime<chrono::Local>,
    end: DateTime<chrono::Local>,
    start_text: Option<&str>,
    end_text: &str,
    verbose: bool,
) {
    if let Some(duration) = math::duration_until(Local::now(), start) {
        println!("wait till start");

        if let Some(text) = start_text {
            publish(&Topic::Hue, "240");
            publish(&Topic::Sat, "100");
            publish(&Topic::Text, text);
        }

        sleep(duration);
    }

    loop {
        let now = Local::now();
        let remaining_seconds = end.timestamp() - now.timestamp();
        let remaining_minutes = remaining_seconds / 60;
        if remaining_seconds <= 0 {
            break;
        }

        let position =
            math::calc_relative_position(start.timestamp(), end.timestamp(), now.timestamp());
        let hue = math::interpolate(80, 0, position);

        if verbose {
            println!(
                "# {} {:6} sec {:4} min {:2.2}%",
                now.format(TIMEFORMAT),
                remaining_seconds,
                remaining_minutes,
                position * 100.0,
            );
        }

        let text = if remaining_minutes > 99 {
            format!("{:4}m", remaining_minutes)
        } else if remaining_seconds > 99 {
            format!("{:2}min", remaining_minutes)
        } else {
            format!("{:2}sec", remaining_seconds)
        };

        publish(&Topic::Text, &text);
        publish(&Topic::Hue, &format!("{}", hue));
        publish(&Topic::Sat, "100");

        let modulo = if remaining_seconds <= 20 {
            1
        } else if remaining_seconds < 100 {
            5
        } else {
            30
        };
        sleep_until_second(modulo);
    }

    if verbose {
        println!("# {} end!", Local::now().format(TIMEFORMAT));
    }

    publish(&Topic::Text, end_text);
    publish(&Topic::Hue, "240");
    publish(&Topic::Sat, "100");
}

fn sleep_until_second(modulo: u32) {
    let now = Local::now();
    let remaining_nanoseconds = 1_000_000_000 - now.nanosecond();

    let current_second = now.second();
    let remaining_seconds = modulo - (current_second % modulo) - 1;

    sleep(Duration::new(
        remaining_seconds as u64,
        remaining_nanoseconds,
    ));
}
