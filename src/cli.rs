use clap::{command, Arg, Command, ValueHint};

#[must_use]
pub fn build() -> Command<'static> {
    command!()
        .name("Meeting Countdown")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show each time tick on stdout"),
        )
        .arg(
            Arg::new("starttime")
                .value_name("STARTTIME")
                .value_hint(ValueHint::Other)
                .required(true)
                .help("Start time of the Meeting. From then the remaining time is published."),
        )
        .arg(
            Arg::new("endtime")
                .value_name("ENDTIME")
                .value_hint(ValueHint::Other)
                .required(true)
                .help("End time of the Meeting. Until then the remaining time is published."),
        )
        .arg(
            Arg::new("start text")
                .long("start-text")
                .value_name("STRING")
                .value_hint(ValueHint::Other)
                .takes_value(true)
                .help("Text which is displayed before countdown starts."),
        )
        .arg(
            Arg::new("end text")
                .long("end-text")
                .value_hint(ValueHint::Other)
                .value_name("STRING")
                .takes_value(true)
                .default_value("Meeting is over. Have a nice day!")
                .help("Text which is displayed when the countdown ends."),
        )
        .arg(
            Arg::new("near end blink")
                .long("blink")
                .value_name("INT")
                .value_hint(ValueHint::Other)
                .takes_value(true)
                .default_value("300")
                .help("Seconds before end where the time should blink. 0 to disable"),
        )
}

#[test]
fn verify() {
    build().debug_assert();
}
