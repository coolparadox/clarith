pub mod protocol;
mod strategy;

use crate::strategy::Strategy;

pub struct Number {
    strategy: Box<dyn Strategy>,
}

impl Number {
    pub fn egest(&mut self) -> Option<protocol::Reduction> {
        match self.strategy.egest() {
            Ok(message) => message,
            Err(new_strategy) => {
                self.strategy = new_strategy;
                self.egest()
            }
        }
    }
}

pub enum Value {
    Special(protocol::Fixed),
    Other(Option<protocol::Primer>, Number),
}

pub fn ratio(num: isize, den:isize) -> Value {
    ratio_u(
        (num >= 0 && den >= 0) || (num < 0 && den < 0),
        if num >= 0 { num } else { -num } as usize,
        if den >= 0 { den } else { -den } as usize)
}

pub fn ratio_u(positive: bool, num: usize, den: usize) -> Value {
    let (special, primer, ratio) = strategy::ratio::new(positive, num, den);
    if let Some(fixed) = special {
        return Value::Special(fixed);
    }
    Value::Other(primer, Number{strategy: Box::new(ratio.unwrap())})
}

pub fn one() -> Value {
    Value::Other(None, Number{strategy: Box::new(strategy::one::new())})
}

#[cfg(test)]
mod tests {

    use crate::one;
    use crate::Value;

    #[test]
    fn test_one() {
        if let Value::Other(None, mut number) = one() {
            assert_eq!(number.egest(), None);
        }
        else {
            panic!("unexpected initialization");
        }
    }

}
