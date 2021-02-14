use chrono::{Duration, Local};

mod cli;
mod publish;
mod timeloop;
mod topic;

fn main() {
    let matches = cli::build().get_matches();

    let verbose = matches.is_present("verbose");

    let start = matches
        .value_of("starttime")
        .and_then(cli::time_string_to_date_time)
        .expect("starttime could not be read from the command line");

    let mut end = matches
        .value_of("endtime")
        .and_then(cli::time_string_to_date_time)
        .expect("endtime could not be read from the command line");

    let start_text = matches.value_of("start text");

    let end_text = matches
        .value_of("end text")
        .expect("end text could not be read from command line");

    let blink_near_end_seconds = matches
        .value_of("near end blink")
        .and_then(|s| s.parse::<u32>().ok())
        .expect("blink seconds could not be read from command line");

    let now = Local::now();

    if end.timestamp() - start.timestamp() <= 0 || end.timestamp() - now.timestamp() <= 0 {
        end = end
            .checked_add_signed(Duration::days(1))
            .expect("failed to assume end date tomorrow");
    }

    println!("# Now:   {}", now.to_string());
    println!("# Start: {}", start.to_string());
    println!("# End:   {}", end.to_string());

    timeloop::timeloop(
        &start,
        &end,
        start_text,
        end_text,
        blink_near_end_seconds,
        verbose,
    );
}
