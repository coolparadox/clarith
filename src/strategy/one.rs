use crate::protocol;
use crate::strategy::Strategy;

pub struct One {
}

pub fn new() -> One {
    One {}
}

impl Strategy for One {

    fn egest(&self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}


