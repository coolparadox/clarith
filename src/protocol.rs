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

/*!
 * This module defines the required symbols for expressing unbounded, finite numeric values in a continued logarithm representation.
 *
 * The underlying symbols are given by the Reduction enum.
 * The Primer enum expands the representation domain to the whole numeric axis.
 * Finally, the Special enum covers the holes of the representation scheme.
 */

/**
 * The Reduction enum defines the symbols that allow any number greater than zero
 * and lesser than one to be represented in a continued logarithm format.
 *
 * There is a one-to-one relationship between any value of this domain and the sequence of Reduction
 * symbols that represent it.
 *  Also, a Reduction sequence is finite if and only if the represented value is a rational number
 *  (TODO: link to a formal proof).
 *
 * Examples:
 *  - 0.5 = '' (the empty sequence)
 *  - 0.1 = 'AAAUA'
 *  - 0.567 = 'UUAUUUUUUUAAAAUA'
 *  - 0.888888... = 'UAA'
 */
#[derive(Debug, PartialEq)]
pub enum Reduction {
    /// The value was greater than zero and lesser than one half, and was doubled.
    Amplify,
    /// The value was greater than one half and lesser than one, was reciprocated and then had one subtracted from itself.
    Uncover,
}

/**
 * The Primer enum defines transformations that can be applied to values represented by Reduction
 * messages, expanding the domain of represented values.
 * By optionally prepending a Primer to a sequence of Reduction symbols,
 * any finite value (with a few exceptions given by the Special enum) can be represented.
 *
 * Examples:
 *  - -0.5 = 'R'
 *  - 5 = 'TAAUA'
 *  - -3.14 = 'GAUUUAUUUAAUUUU'
 */
#[derive(Debug, PartialEq)]
pub enum Primer {
    /// The value was reciprocated.
    Turn,
    /// The value was negated.
    Reflect,
    /// The value was reciprocated and negated.
    Ground,
}

/**
 * Values that cannot be represented by the combination of an optional Primer and a Reduction sequence.
 */
#[derive(Debug, PartialEq)]
pub enum Special {
    /// The value is minus one.
    NegOne,
    /// The value is zero.
    Zero,
    /// The value is one.
    PosOne,
}
