pub use self::http_matrix::HttpMatrix;
pub use self::pixelmatrix::Pixelmatrix;
pub use self::retry::Retry;
use crate::remaining::Remaining;

mod http_matrix;
mod pixelmatrix;
mod retry;

pub trait Display {
    fn show_remaining(&mut self, percentage: f32, remaining: Remaining) -> anyhow::Result<()>;
    fn clear(&mut self) -> anyhow::Result<()>;
}
