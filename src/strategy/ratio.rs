use crate::protocol;
use crate::strategy::Strategy;
use crate::Value;

pub struct Ratio {
    num: u64,
    den: u64,
}

impl Ratio {

    fn new(is_positive: bool, num: u64, den:u64) -> Value {
        if num == 0 && den == 0 {
            panic!("undefined ratio")
        }
        Value::Zero
    }

}

impl Strategy for Ratio {

    fn egest(&self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}


