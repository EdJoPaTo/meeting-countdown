use chrono::{DateTime, Duration, Local, NaiveTime};
use url::Url;

use crate::display::{Display, Pixelmatrix};

mod cli;
mod display;
mod math;
mod remaining;
mod timeloop;

fn main() {
    let matches = cli::build().get_matches();

    let start = matches
        .get_one::<String>("starttime")
        .and_then(time_string_to_date_time)
        .expect("starttime could not be read from the command line");

    let mut end = matches
        .get_one::<String>("endtime")
        .and_then(time_string_to_date_time)
        .expect("endtime could not be read from the command line");

    let now = Local::now();

    if end.timestamp() - start.timestamp() <= 0 || end.timestamp() - now.timestamp() <= 0 {
        end = end
            .checked_add_signed(Duration::days(1))
            .expect("failed to assume end date tomorrow");
    }

    let display = {
        let mut displays: Vec<Box<dyn Display>> = Vec::new();

        if let Some(addr) = matches.get_one::<String>("pixelmatrix") {
            let target = Pixelmatrix::new(addr).expect("failed to connect to pixelmatrix");
            displays.push(Box::new(target));
        }

        if let Some(url) = matches.get_one::<Url>("http-textmatrix") {
            let target = display::Retry::new(
                display::HttpMatrix::new(url).expect("failed to connect to http textmatrix"),
            );
            displays.push(Box::new(target));
        }

        display::Multiple::new(displays)
    };

    println!("Now:   {}", now);
    println!("Start: {}", start);
    println!("End:   {}", end);

    timeloop::timeloop(&start, &end, display);
}

fn time_string_to_date_time(timestring: &String) -> Option<DateTime<Local>> {
    let today = chrono::offset::Local::now().date();
    let fmt = if timestring.len() > 5 {
        "%H:%M:%S"
    } else {
        "%H:%M"
    };
    NaiveTime::parse_from_str(timestring, fmt)
        .ok()
        .and_then(|t| today.and_time(t))
}
