pub mod protocol;
mod strategy;
pub mod number;

// use crate::strategy::Strategy;

pub enum Value {
    Special(protocol::Fixed),
    Other(Option<protocol::Primer>, number::Number),
}

pub fn one() -> Value {
    Value::Other(None, number::Number{strategy: Box::new(strategy::one::new())})
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
