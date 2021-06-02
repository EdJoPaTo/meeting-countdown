use clap::{App, AppSettings, Arg};

#[must_use]
pub fn build() -> App<'static, 'static> {
    App::new("Meeting Countdown")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
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
        .arg(
            Arg::with_name("near end blink")
                .long("blink")
                .value_name("INT")
                .takes_value(true)
                .default_value("300")
                .help("Seconds before end where the time should blink. 0 to disable"),
        )
}
