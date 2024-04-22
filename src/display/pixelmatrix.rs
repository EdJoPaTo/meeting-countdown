use embedded_graphics::mono_font::ascii::{FONT_4X6, FONT_6X12};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use esp_remotecontrolled_led_matrix_client::sync::Client;

use crate::display::Display;
use crate::math::{hue_to_rgb, interpolate_u16};
use crate::remaining::Remaining;

pub struct Pixelmatrix {
    client: Client,
}

impl Pixelmatrix {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let client = Client::connect(addr)?;
        Ok(Self { client })
    }
}

impl Display for Pixelmatrix {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn show_remaining(&mut self, percentage: f32, remaining: Remaining) -> anyhow::Result<()> {
        let (text, unit, x) = match remaining {
            Remaining::ManyHours(hours) => (format!("{hours:>5}"), "h", -1),
            Remaining::SingleDigitHours(hours, minutes) => {
                (format!("{hours}:{minutes:02}"), "h", 5)
            }
            Remaining::Minutes(min) => (format!("{min:>3}"), "min", 1),
            Remaining::Seconds(sec) => (format!("{sec:>3}"), "sec", 1),
        };

        let hue = interpolate_u16(80, 0, percentage);
        let (red, green, blue) = hue_to_rgb(hue);
        let color = Rgb888::new(red, green, blue);
        self.client.fill(0, 0, 0)?;
        Text::new(
            unit,
            Point::new(32 - (4 * unit.len() as i32) + 1, 6),
            MonoTextStyle::new(&FONT_4X6, color),
        )
        .draw(&mut self.client)?;
        Text::new(
            &text,
            Point::new(x, 6),
            MonoTextStyle::new(&FONT_6X12, color),
        )
        .draw(&mut self.client)?;

        self.client.flush()?;
        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        self.client.fill(0, 0, 0)?;
        Ok(())
    }
}
