use clap::{command, Arg, Command, ValueHint};

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Command<'static> {
    command!()
        .name("Meeting Countdown")
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
            Arg::new("pixelmatrix")
                .long("pixelmatrix")
                .env("MEETING_PIXELMATRIX")
                .value_name("ADDR")
                .value_hint(ValueHint::Hostname)
                .required_unless_present("pixelmatrix")
                .help("Target Pixelmatrix Address to display the rest time to")
                .long_help("Target Pixelmatrix Address to display the rest time to. Looks like `espPixelmatrix:1337`.\nSee https://github.com/EdJoPaTo/esp-remotecontrolled-led-matrix"),
        )
        .arg(
            Arg::new("http-textmatrix")
                .long("http-textmatrix")
                .env("MEETING_HTTP_TEXTMATRIX")
                .value_name("URL")
                .value_hint(ValueHint::Url)
                .required_unless_present("pixelmatrix")
                .help("Target HTTP Textmatrix Address to display the rest time to")
                .long_help("Target HTTP Textmatrix Address to display the rest time to. Looks like `http://esp-matrix/`.\nSee https://github.com/EdJoPaTo/esp-http-neomatrix-text")

        )
}

#[test]
fn verify() {
    build().debug_assert();
}
