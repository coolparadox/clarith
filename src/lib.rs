/// Building elements of rational values.
pub mod protocol;
mod strategy;

use crate::strategy::Strategy;

/// A rational value greater than zero and lesser than one.
pub struct Number {
    strategy: Box<dyn Strategy>,
}

impl Number {

    /// Removes information from a Number instance.
    /// Calling this method changes Number as a side effect.
    /// Returns the next information component extracted from Number, or None if Number is one half.
    pub fn egest(&mut self) -> Option<protocol::Reduction> {
        match self.strategy.egest() {
            Ok(reduction) => reduction,
            Err(new_strategy) => {
                self.strategy = new_strategy;
                self.egest()
            }
        }
    }

}

/// A rational value with unbounded range and precision.
pub enum Value {
    Special(protocol::Fixed),
    Other(Option<protocol::Primer>, Number),
}

/// Construct a rational value from the ratio of two signed machine integers.
pub fn ratio(num: isize, den:isize) -> Value {
    ratio_u(
        (num >= 0 && den >= 0) || (num < 0 && den < 0),
        if num >= 0 { num } else { -num } as usize,
        if den >= 0 { den } else { -den } as usize)
}

/// Construct a rational value from the ratio of two unsigned machine integers.
pub fn ratio_u(positive: bool, num: usize, den: usize) -> Value {
    let (special, primer, ratio) = strategy::ratio::new(positive, num, den);
    if let Some(fixed) = special {
        return Value::Special(fixed);
    }
    Value::Other(primer, Number{strategy: Box::new(ratio.unwrap())})
}

