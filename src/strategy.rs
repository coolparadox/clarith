pub mod one;
pub mod ratio;

use crate::protocol;

pub trait Strategy {
    fn egest(&self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>>;
}
