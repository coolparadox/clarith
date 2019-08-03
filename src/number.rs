use crate::protocol;
use crate::strategy::Strategy;

pub struct Number {
    pub strategy: Box<dyn Strategy>,
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
