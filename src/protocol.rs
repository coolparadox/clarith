/*
 * Copyright 2019 Rafael Lorandi <coolparadox@gmail.com>
 *
 * This file is part of clarith, a library for performing arithmetic
 * in continued logarithm representation.
 * 
 * clarith is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * clarith is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with clarith.  If not, see <http://www.gnu.org/licenses/>
 */

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
