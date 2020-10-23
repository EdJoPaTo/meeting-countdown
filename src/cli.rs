use chrono::{DateTime, Local, NaiveTime};
use clap::{App, AppSettings, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("Meeting Countdown")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Show each time tick on stdout"),
        )
        .arg(
            Arg::with_name("starttime")
                .required(true)
                .value_name("STARTTIME")
                .help("Start time of the Meeting. From then the remaining time is published."),
        )
        .arg(
            Arg::with_name("endtime")
                .required(true)
                .value_name("ENDTIME")
                .help("End time of the Meeting. Until then the remaining time is published."),
        )
        .arg(
            Arg::with_name("start text")
                .long("start-text")
                .value_name("STRING")
                .takes_value(true)
                .help("Text which is displayed before countdown starts."),
        )
        .arg(
            Arg::with_name("end text")
                .long("end-text")
                .value_name("STRING")
                .takes_value(true)
                .help("Text which is displayed when the countdown ends.")
                .default_value("Meeting is over. Have a nice day!"),
        )
}

pub fn time_string_to_date_time(timestring: &str) -> Option<DateTime<Local>> {
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
