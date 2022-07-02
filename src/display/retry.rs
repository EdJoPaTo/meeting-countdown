use crate::display::Display;
use crate::remaining::Remaining;

pub struct Retry<D: Display> {
    other: D,
}

impl<D: Display> Retry<D> {
    pub const fn new(display: D) -> Self {
        Self { other: display }
    }
}

impl<D: Display> Display for Retry<D> {
    fn show_remaining(&mut self, percentage: f32, remaining: Remaining) -> anyhow::Result<()> {
        retry::retry(retry::delay::Fixed::from_millis(20).take(2), || {
            self.other.show_remaining(percentage, remaining)
        })
        .map_err(|err| anyhow::anyhow!("{}", err))
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        retry::retry(retry::delay::Fixed::from_millis(20).take(2), || {
            self.other.clear()
        })
        .map_err(|err| anyhow::anyhow!("{}", err))
    }
}
