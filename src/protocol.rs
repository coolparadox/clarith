/// Basic components of a Clog value.
#[derive(Debug, PartialEq)]
pub enum Reduction {
    /// Clog value was greater than zero and lesser than one half, and was doubled.
    Amplify,
    /// Clog value was greater than one half and lesser than one, was reciprocated and then had one subtracted from itself.
    Uncover,
}

/// Ways to increase the representation range of a Clog value.
#[derive(Debug, PartialEq)]
pub enum Primer {
    /// The associated Clog value must be reciprocated in order to represent the intended value.
    Turn,
    /// The associated Clog value must be negated in order to represent the intended value.
    Reflect,
    /// The associated Clog value must be reciprocated and negated in order to represent the intended value.
    Ground,
}

/// Values that cannot be uniquely represented by the combination of a Primer and a Clog.
#[derive(Debug, PartialEq)]
pub enum Special {
	/// Negative infinity
    NegInf,
	/// Minus one
    NegOne,
	/// Zero
    Zero,
	/// One
    One,
	/// Positive infinity
    PosInf,
}
