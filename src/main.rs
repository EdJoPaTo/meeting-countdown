use chrono::Local;
use timeloop::TIMEFORMAT;

mod cli;
mod timeloop;
mod topic;

fn main() {
    let matches = cli::build_cli().get_matches();

    let verbose = matches.is_present("verbose");

    let start = matches
        .value_of("starttime")
        .and_then(cli::time_string_to_date_time)
        .expect("starttime could not be read from the command line");

    let end = matches
        .value_of("endtime")
        .and_then(cli::time_string_to_date_time)
        .expect("endtime could not be read from the command line");

    let end_text = matches
        .value_of("end text")
        .expect("end text could not be read from command line");

    let now = Local::now();
    println!("# Now:   {}", now.format(TIMEFORMAT));
    println!("# Start: {}", start.format(TIMEFORMAT));
    println!("# End:   {}", end.format(TIMEFORMAT));

    assert!(
        end.timestamp() - start.timestamp() > 0,
        "endtime has to be after starttime"
    );
    assert!(
        end.timestamp() - now.timestamp() > 0,
        "endtime has to be in the future"
    );

    timeloop::timeloop(start, end, end_text, verbose, publish);
}

fn publish(topic: topic::Topic, value: &str) {
    let verb = topic::get_verb(&topic);
    println!("{} {}", verb, value);
}
