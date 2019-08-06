mod strategy;

/// Building elements of numbers.
pub mod protocol;

use crate::strategy::Strategy;

/// A number greater than zero and lesser than one with unbounded precision.
pub struct Clog {
    strategy: Box<dyn Strategy>,
}

impl Clog {

    /// Extract information from a Clog instance.
    /// This method modifies the called instance, leaving it with less embedded continued logarithm information.
    /// Returns the next continued logarithm component extracted from the Clog, or None if the Clog is one half.
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

/// A number with unbounded range and precision.
pub enum Number {
    Special(protocol::Special),
    Other(Option<protocol::Primer>, Clog),
}

/// Construct a Number from the ratio of two signed machine integers.
pub fn ratio(num: isize, den:isize) -> Number {
    ratio_u(
        (num >= 0 && den >= 0) || (num < 0 && den < 0),
        if num >= 0 { num } else { -num } as usize,
        if den >= 0 { den } else { -den } as usize)
}

/// Construct a Number from the ratio of two unsigned machine integers.
pub fn ratio_u(positive: bool, num: usize, den: usize) -> Number {
    let (special, primer, ratio) = strategy::ratio::new(positive, num, den);
    if let Some(fixed) = special {
        return Number::Special(fixed);
    }
    Number::Other(primer, Clog{strategy: Box::new(ratio.unwrap())})
}

