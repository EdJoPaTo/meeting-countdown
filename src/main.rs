use chrono::{DateTime, Duration, Local, NaiveTime};
use clap::Parser as _;

use crate::display::{Display, Pixelmatrix};

mod cli;
mod display;
mod math;
mod remaining;
mod timeloop;

fn main() {
    let matches = cli::Cli::parse();

    let now = Local::now();

    let start = matches.starttime.as_ref().map_or(now, |starttime| {
        time_string_to_date_time(starttime)
            .expect("starttime could not be read from the command line")
    });

    let mut end = time_string_to_date_time(&matches.endtime)
        .expect("endtime could not be read from the command line");

    if end.timestamp() - start.timestamp() <= 0 || end.timestamp() - now.timestamp() <= 0 {
        end = end
            .checked_add_signed(Duration::days(1))
            .expect("failed to assume end date tomorrow");
    }

    let displays = {
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

        displays
    };

    println!("Now:   {now}");
    println!("Start: {start}");
    println!("End:   {end}");

    timeloop::timeloop(&start, &end, displays);
}

fn time_string_to_date_time(timestring: &str) -> Option<DateTime<Local>> {
    let fmt = if timestring.len() > 5 {
        "%H:%M:%S"
    } else {
        "%H:%M"
    };
    let time = NaiveTime::parse_from_str(timestring, fmt).ok()?;
    Local::now().with_time(time).single()
}
