use chrono::{DateTime, Duration, Local, NaiveTime};
use clap::Parser;

use crate::display::{Display, Pixelmatrix};

mod cli;
mod display;
mod math;
mod remaining;
mod timeloop;

fn main() {
    let matches = cli::Cli::parse();

    let start = time_string_to_date_time(&matches.starttime)
        .expect("starttime could not be read from the command line");

    let mut end = time_string_to_date_time(&matches.endtime)
        .expect("endtime could not be read from the command line");

    let now = Local::now();

    if end.timestamp() - start.timestamp() <= 0 || end.timestamp() - now.timestamp() <= 0 {
        end = end
            .checked_add_signed(Duration::days(1))
            .expect("failed to assume end date tomorrow");
    }

    let display = {
        let mut displays: Vec<Box<dyn Display>> = Vec::new();

        if let Some(addr) = &matches.pixelmatrix {
            let target = Pixelmatrix::new(addr).expect("failed to connect to pixelmatrix");
            displays.push(Box::new(target));
        }

        if let Some(url) = &matches.http_textmatrix {
            let target = display::Retry::new(
                display::HttpMatrix::new(url).expect("failed to connect to http textmatrix"),
            );
            displays.push(Box::new(target));
        }

        display::Multiple::new(displays)
    };

    println!("Now:   {now}");
    println!("Start: {start}");
    println!("End:   {end}");

    timeloop::timeloop(&start, &end, display);
}

fn time_string_to_date_time(timestring: &str) -> Option<DateTime<Local>> {
    let today = Local::now().date_naive();
    let fmt = if timestring.len() > 5 {
        "%H:%M:%S"
    } else {
        "%H:%M"
    };
    let time = NaiveTime::parse_from_str(timestring, fmt).ok()?;
    today.and_time(time).and_local_timezone(Local).single()
}
