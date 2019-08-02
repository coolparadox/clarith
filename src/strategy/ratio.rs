use crate::protocol;
use crate::strategy::Strategy;
use crate::Value;

pub struct Ratio {
    num: u64,
    den: u64,
}

impl Ratio {

    fn new(positive: bool, num: u64, den:u64) -> Value {
        // if num == 0 && den == 0 {
            // panic!("undefined ratio");
        // }
        if den == 0 {
            // if positive {
                // Value::PosInf
            // }
            Value::NegInf
        }
        else {
            Value::Zero
        }
    }

}

impl Strategy for Ratio {

    fn egest(&self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}


