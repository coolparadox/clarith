mod protocol;
mod strategy;

use crate::strategy::Strategy;

enum Value {
    Zero,
    NonZero(Option<protocol::Primer>, Number),
}

struct Number {
    strategy: Box<dyn Strategy>,
}

fn one() -> Value {
    Value::NonZero(None, Number{strategy: Box::new(strategy::one::new())})
}

impl Number {
    fn egest(&mut self) -> Option<protocol::Reduction> {
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
        if let Value::NonZero(None, mut number) = one() {
            assert_eq!(number.egest(), None);
        }
        else {
            panic!("unexpected initialization");
        }
    }

}
