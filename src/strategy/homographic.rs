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

use std::cmp::Ordering;
use crate::protocol;
use crate::strategy::Strategy;
use crate::strategy::ratio::Ratio;
use crate::Clog;
use crate::Number;

#[cfg(test)]
mod tests {

    use super::*;

    /*
    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn forbids_undefined_ratio() {
        new(true, 0, 0);
    }

    #[test]
    fn supports_negative_infinity() {
        if let (Some(protocol::Special::NegInf), None, None) = new(false, 1, 0) {
            return;
        }
        panic!();
    }
    */

    #[test]
    fn zero() {
        assert_eq!(Number::compare(Number::homographic(Number::ratio(0, 1), 1, 0, 0, 1), Number::ratio(0, 1)), Ordering::Equal);
    }
}

pub struct Homographic {
    x: Clog,
    pa: bool,
    a: usize,
    pb: bool,
    b: usize,
    c: usize,
    pd: bool,
    d: usize,
}

pub fn new(_x: Number, _pa: bool, _a: usize, _pb: bool, _b: usize, _pc: bool, _c: usize, _pd: bool, _d: usize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>) {
    (Some(protocol::Special::Zero), None, None, None)
}

impl Strategy for Homographic {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}
