#[derive(Debug, PartialEq)]
pub enum Reduction {
    Amplify,
    Uncover,
}

#[derive(Debug, PartialEq)]
pub enum Primer {
    Turn,
    Reflect,
    Ground,
}

#[derive(Debug, PartialEq)]
pub enum Fixed {
    NegInf,
    NegOne,
    Zero,
    One,
    PosInf,
}
