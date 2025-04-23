use clap::{Parser, ValueHint};
use url::Url;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    /// Start time of the Meeting.
    ///
    /// From then the remaining time is published. Defaults to the current time.
    #[arg(long, value_hint = ValueHint::Other)]
    pub starttime: Option<String>,

    /// End time of the Meeting.
    ///
    /// Until then the remaining time is published.
    #[arg(value_hint = ValueHint::Other)]
    pub endtime: String,

    /// Target Pixelmatrix Address to display the rest time to.
    ///
    /// Looks like `espPixelmatrix:1337`.
    ///
    /// See <https://github.com/EdJoPaTo/esp-remotecontrolled-led-matrix>
    #[arg(
        long,
        env = "MEETING_PIXELMATRIX",
        value_name = "ADDR",
        value_hint = ValueHint::Hostname,
        required_unless_present = "http_textmatrix",
    )]
    pub pixelmatrix: Option<String>,

    /// Target HTTP Textmatrix Address to display the rest time to.
    ///
    /// Looks like `http://esp-matrix/`.
    ///
    /// See <https://github.com/EdJoPaTo/esp-http-neomatrix-text>
    #[arg(
        long,
        env = "MEETING_HTTP_TEXTMATRIX",
        value_name = "URL",
        value_hint = ValueHint::Url,
        required_unless_present = "pixelmatrix",
    )]
    pub http_textmatrix: Option<Url>,
}

#[test]
fn verify() {
    use clap::CommandFactory as _;
    Cli::command().debug_assert();
}
