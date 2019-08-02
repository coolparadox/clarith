pub mod protocol;
mod strategy;

use crate::strategy::Strategy;

pub enum Value {
    Special(protocol::Fixed),
    Other(Option<protocol::Primer>, Number),
}

pub struct Number {
    strategy: Box<dyn Strategy>,
}

pub fn one() -> Value {
    Value::Other(None, Number{strategy: Box::new(strategy::one::new())})
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
