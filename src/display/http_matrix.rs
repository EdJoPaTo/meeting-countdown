use std::str::FromStr as _;
use std::time::Duration;

use ureq::http::Uri;

use crate::display::Display;
use crate::math::interpolate_u16;
use crate::remaining::Remaining;

pub struct HttpMatrix {
    agent: ureq::Agent,

    set_on: Uri,
    set_hue: Uri,
    set_sat: Uri,
    set_text: Uri,
}

impl HttpMatrix {
    pub fn new(url: &url::Url) -> anyhow::Result<Self> {
        let set_on = Uri::from_str(url.join("/on")?.as_str())?;
        let set_hue = Uri::from_str(url.join("/hue")?.as_str())?;
        let set_sat = Uri::from_str(url.join("/sat")?.as_str())?;
        let set_text = Uri::from_str(url.join("/text")?.as_str())?;

        let agent = ureq::Agent::new_with_config(
            ureq::Agent::config_builder()
                .timeout_global(Some(Duration::from_millis(100)))
                .build(),
        );

        // check if the remote exists and works
        agent.get(url.as_str()).call()?;

        Ok(Self {
            agent,
            set_on,
            set_hue,
            set_sat,
            set_text,
        })
    }
}

impl Display for HttpMatrix {
    fn show_remaining(&mut self, percentage: f32, remaining: Remaining) -> anyhow::Result<()> {
        let text = match remaining {
            Remaining::ManyHours(hours) => format!("{hours:>4}h"),
            Remaining::SingleDigitHours(hours, minutes) => format!("{hours}:{minutes:02}h"),
            Remaining::Minutes(min) if min >= 100 => format!("{min:>4}m"),
            Remaining::Minutes(min) => format!("{min:>2}min"),
            Remaining::Seconds(sec) if sec >= 100 => format!("{sec:>4}s"),
            Remaining::Seconds(sec) => format!("{sec:>2}sec"),
        };
        let hue = interpolate_u16(80, 0, percentage);

        self.agent.post(&self.set_text).send(text)?;
        self.agent.post(&self.set_hue).send(hue.to_string())?;
        self.agent.post(&self.set_sat).send("100")?;
        self.agent.post(&self.set_on).send("1")?;

        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        self.agent.post(&self.set_text).send_empty()?;
        Ok(())
    }
}
