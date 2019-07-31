pub mod one;

use crate::protocol;

pub trait Strategy {
    fn egest(&self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>>;
}
