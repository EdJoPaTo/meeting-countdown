use std::time::Duration;

use crate::display::Display;
use crate::math::interpolate_u16;
use crate::remaining::Remaining;

pub struct HttpMatrix {
    agent: ureq::Agent,

    set_on: String,
    set_hue: String,
    set_sat: String,
    set_text: String,
}

impl HttpMatrix {
    pub fn new(url: &url::Url) -> anyhow::Result<Self> {
        let set_on = url.join("/on")?.to_string();
        let set_hue = url.join("/hue")?.to_string();
        let set_sat = url.join("/sat")?.to_string();
        let set_text = url.join("/text")?.to_string();

        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_millis(100))
            .build();

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

        self.agent.post(&self.set_text).send_string(&text)?;
        self.agent
            .post(&self.set_hue)
            .send_string(&hue.to_string())?;
        self.agent.post(&self.set_sat).send_string("100")?;
        self.agent.post(&self.set_on).send_string("1")?;

        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        self.agent.post(&self.set_text).send_string("")?;
        Ok(())
    }
}
