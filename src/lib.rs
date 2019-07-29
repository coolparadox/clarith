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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
