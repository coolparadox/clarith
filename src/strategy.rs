pub mod ratio;

use crate::protocol;

pub trait Strategy {
    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>>;
}
