#[derive(Debug, PartialEq)]
pub enum Reduction {
    Amplify,
    Uncover,
}

pub enum Primer {
    Turn,
    Reflect,
    Ground,
}

pub enum Fixed {
    Zero,
    NegInf,
    PosInf,
}
