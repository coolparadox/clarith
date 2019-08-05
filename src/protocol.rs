/// Basic components of a Number.
#[derive(Debug, PartialEq)]
pub enum Reduction {
    /// Number was greater than zero and lesser than one half, and was doubled.
    Amplify,
    /// Number was greater than one half and no greater than one, was reciprocated and then had one subtracted from itself.
    Uncover,
}

/// Ways to increase the representation range of a Number.
#[derive(Debug, PartialEq)]
pub enum Primer {
    /// The original value was greater than one and was reciprocated. 
    Turn,
	/// The original value was lesser than zero but no lesser than minus one, and was negated. 
    Reflect,
	/// The original value was lesser than minus one, was negated and then was reciprocated.
    Ground,
}

/// Values that cannot be uniquely represented by the combination of a Primer and a Number.
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
