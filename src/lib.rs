enum EgestionCode {
    Amplify,
    Uncover,
}

enum Primer {
    Turn,
    Reflect,
    Ground,
}

enum Value {
    Zero,
    NonZero(Option<Primer>, Number),
}

struct Number {
    strategy: Box<dyn Strategy>,
}

trait Strategy {
    fn egest(&self) -> StrategyOutput;
}

enum StrategyOutput {
    Egestion(Option<EgestionCode>),
    Exhaustion(Box<dyn Strategy>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
