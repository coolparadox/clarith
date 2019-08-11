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

mod strategy;
mod compare;

/// Building elements of numbers.
pub mod protocol;

use std::cmp::Ordering;
use crate::strategy::Strategy;

/// A number greater than zero and lesser than one with unbounded precision.
pub struct Clog {
    strategy: Box<dyn Strategy>,
}

impl Clog {

    /// Extract information from a Clog instance.
    /// This method modifies the called instance, leaving it with less embedded continued logarithm information.
    /// Returns the next continued logarithm component extracted from the Clog, or None if the Clog is one half.
    pub fn egest(&mut self) -> Option<protocol::Reduction> {
        match self.strategy.egest() {
            Ok(reduction) => reduction,
            Err(new_strategy) => {
                self.strategy = new_strategy;
                self.egest()
            }
        }
    }

}

/// An unbounded number with unbounded precision.
pub enum Number {
    Special(protocol::Special),
    Other(Option<protocol::Primer>, Clog),
}

impl Number {

    /// Destructively compare two Numbers.
    pub fn compare(n1: Number, n2: Number) -> Ordering {
        compare::compare(n1, n2)
    }

    /// Construct a Number from the ratio of two signed machine integers.
    pub fn from_ratio(num: isize, den:isize) -> Number {
        Number::from_ratio_u(
            (num >= 0 && den >= 0) || (num < 0 && den < 0),
            if num >= 0 { num } else { -num } as usize,
            if den >= 0 { den } else { -den } as usize)
    }

    /// Construct a Number from the ratio of two unsigned machine integers.
    pub fn from_ratio_u(positive: bool, num: usize, den: usize) -> Number {
        let (special, primer, ratio) = strategy::ratio::new(positive, num, den);
        if let Some(fixed) = special {
            return Number::Special(fixed);
        }
        Number::Other(primer, Clog{strategy: Box::new(ratio.unwrap())})
    }

}
