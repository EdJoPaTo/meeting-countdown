use crate::display::Display;
use crate::remaining::Remaining;

pub struct Multiple<'a> {
    displays: Vec<Box<dyn Display + 'a>>,
}

impl<'a> Multiple<'a> {
    pub fn new<I>(displays: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Display>>,
    {
        let displays = displays.into_iter().collect::<Vec<_>>();
        Self { displays }
    }
}

impl<'a> Display for Multiple<'a> {
    fn show_remaining(&mut self, percentage: f32, remaining: Remaining) -> anyhow::Result<()> {
        for display in &mut self.displays {
            if let Err(err) = display.show_remaining(percentage, remaining) {
                println!("Display Error {}", err);
            }
        }
        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        for display in &mut self.displays {
            if let Err(err) = display.clear() {
                println!("Display Error {}", err);
            }
        }
        Ok(())
    }
}
