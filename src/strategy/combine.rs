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

use std::mem::swap;
use crate::protocol;
use crate::strategy::Strategy;
use crate::Number;
use crate::strategy::ratio::Ratio;
use crate::strategy::homographic;
use crate::strategy::homographic::Homographic;
use crate::Clog;

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn mul() {
        fn t(xn: isize, xd: isize, yn: isize, yd: isize, rn: isize, rd: isize) -> bool {
            Number::compare(Number::combine(Number::ratio(xn, xd), Number::ratio(yn, yd), 1, 0, 0, 0, 0, 0, 0, 1), Number::ratio(rn, rd)) == Ordering::Equal
        }
        assert!(t(-2, 1, -2, 1, 4, 1));
        assert!(t(-2, 1, -1, 1, 2, 1));
        assert!(t(-2, 1, -2, 3, 4, 3));
        assert!(t(-2, 1, -1, 2, 1, 1));
        assert!(t(-2, 1, -1, 4, 1, 2));
        assert!(t(-2, 1, 0, 1, 0, 1));
        assert!(t(-2, 1, 1, 4, -1, 2));
        assert!(t(-2, 1, 1, 2, -1, 1));
        assert!(t(-2, 1, 2, 3, -4, 3));
        assert!(t(-2, 1, 1, 1, -2, 1));
        assert!(t(-2, 1, 2, 1, -4, 1));
        assert!(t(-1, 1, -2, 1, 2, 1));
        assert!(t(-1, 1, -1, 1, 1, 1));
        assert!(t(-1, 1, -2, 3, 2, 3));
        assert!(t(-1, 1, -1, 2, 1, 2));
        assert!(t(-1, 1, -1, 4, 1, 4));
        assert!(t(-1, 1, 0, 1, 0, 1));
        assert!(t(-1, 1, 1, 4, -1, 4));
        assert!(t(-1, 1, 1, 2, -1, 2));
        assert!(t(-1, 1, 2, 3, -2, 3));
        assert!(t(-1, 1, 1, 1, -1, 1));
        assert!(t(-1, 1, 2, 1, -2, 1));
        assert!(t(-2, 3, -2, 1, 4, 3));
        assert!(t(-2, 3, -1, 1, 2, 3));
        assert!(t(-2, 3, -2, 3, 4, 9));
        assert!(t(-2, 3, -1, 2, 1, 3));
        assert!(t(-2, 3, -1, 4, 1, 6));
        assert!(t(-2, 3, 0, 1, 0, 1));
        assert!(t(-2, 3, 1, 4, -1, 6));
        assert!(t(-2, 3, 1, 2, -1, 3));
        assert!(t(-2, 3, 2, 3, -4, 9));
        assert!(t(-2, 3, 1, 1, -2, 3));
        assert!(t(-2, 3, 2, 1, -4, 3));
        assert!(t(-1, 2, -2, 1, 1, 1));
        assert!(t(-1, 2, -1, 1, 1, 2));
        assert!(t(-1, 2, -2, 3, 1, 3));
        assert!(t(-1, 2, -1, 2, 1, 4));
        assert!(t(-1, 2, -1, 4, 1, 8));
        assert!(t(-1, 2, 0, 1, 0, 1));
        assert!(t(-1, 2, 1, 4, -1, 8));
        assert!(t(-1, 2, 1, 2, -1, 4));
        assert!(t(-1, 2, 2, 3, -1, 3));
        assert!(t(-1, 2, 1, 1, -1, 2));
        assert!(t(-1, 2, 2, 1, -1, 1));
        assert!(t(-1, 4, -2, 1, 1, 2));
        assert!(t(-1, 4, -1, 1, 1, 4));
        assert!(t(-1, 4, -2, 3, 1, 6));
        assert!(t(-1, 4, -1, 2, 1, 8));
        assert!(t(-1, 4, -1, 4, 1, 16));
        assert!(t(-1, 4, 0, 1, 0, 1));
        assert!(t(-1, 4, 1, 4, -1, 16));
        assert!(t(-1, 4, 1, 2, -1, 8));
        assert!(t(-1, 4, 2, 3, -1, 6));
        assert!(t(-1, 4, 1, 1, -1, 4));
        assert!(t(-1, 4, 2, 1, -1, 2));
        assert!(t(0, 1, -2, 1, 0, 1));
        assert!(t(0, 1, -1, 1, 0, 1));
        assert!(t(0, 1, -2, 3, 0, 1));
        assert!(t(0, 1, -1, 2, 0, 1));
        assert!(t(0, 1, -1, 4, 0, 1));
        assert!(t(0, 1, 0, 1, 0, 1));
        assert!(t(0, 1, 1, 4, 0, 1));
        assert!(t(0, 1, 1, 2, 0, 1));
        assert!(t(0, 1, 2, 3, 0, 1));
        assert!(t(0, 1, 1, 1, 0, 1));
        assert!(t(0, 1, 2, 1, 0, 1));
        assert!(t(1, 4, -2, 1, -1, 2));
        assert!(t(1, 4, -1, 1, -1, 4));
        assert!(t(1, 4, -2, 3, -1, 6));
        assert!(t(1, 4, -1, 2, -1, 8));
        assert!(t(1, 4, -1, 4, -1, 16));
        assert!(t(1, 4, 0, 1, 0, 1));
        assert!(t(1, 4, 1, 4, 1, 16));
        assert!(t(1, 4, 1, 2, 1, 8));
        assert!(t(1, 4, 2, 3, 1, 6));
        assert!(t(1, 4, 1, 1, 1, 4));
        assert!(t(1, 4, 2, 1, 1, 2));
        assert!(t(1, 2, -2, 1, -1, 1));
        assert!(t(1, 2, -1, 1, -1, 2));
        assert!(t(1, 2, -2, 3, -1, 3));
        assert!(t(1, 2, -1, 2, -1, 4));
        assert!(t(1, 2, -1, 4, -1, 8));
        assert!(t(1, 2, 0, 1, 0, 1));
        assert!(t(1, 2, 1, 4, 1, 8));
        assert!(t(1, 2, 1, 2, 1, 4));
        assert!(t(1, 2, 2, 3, 1, 3));
        assert!(t(1, 2, 1, 1, 1, 2));
        assert!(t(1, 2, 2, 1, 1, 1));
        assert!(t(2, 3, -2, 1, -4, 3));
        assert!(t(2, 3, -1, 1, -2, 3));
        assert!(t(2, 3, -2, 3, -4, 9));
        assert!(t(2, 3, -1, 2, -1, 3));
        assert!(t(2, 3, -1, 4, -1, 6));
        assert!(t(2, 3, 0, 1, 0, 1));
        assert!(t(2, 3, 1, 4, 1, 6));
        assert!(t(2, 3, 1, 2, 1, 3));
        assert!(t(2, 3, 2, 3, 4, 9));
        assert!(t(2, 3, 1, 1, 2, 3));
        assert!(t(2, 3, 2, 1, 4, 3));
        assert!(t(1, 1, -2, 1, -2, 1));
        assert!(t(1, 1, -1, 1, -1, 1));
        assert!(t(1, 1, -2, 3, -2, 3));
        assert!(t(1, 1, -1, 2, -1, 2));
        assert!(t(1, 1, -1, 4, -1, 4));
        assert!(t(1, 1, 0, 1, 0, 1));
        assert!(t(1, 1, 1, 4, 1, 4));
        assert!(t(1, 1, 1, 2, 1, 2));
        assert!(t(1, 1, 2, 3, 2, 3));
        assert!(t(1, 1, 1, 1, 1, 1));
        assert!(t(1, 1, 2, 1, 2, 1));
        assert!(t(2, 1, -2, 1, -4, 1));
        assert!(t(2, 1, -1, 1, -2, 1));
        assert!(t(2, 1, -2, 3, -4, 3));
        assert!(t(2, 1, -1, 2, -1, 1));
        assert!(t(2, 1, -1, 4, -1, 2));
        assert!(t(2, 1, 0, 1, 0, 1));
        assert!(t(2, 1, 1, 4, 1, 2));
        assert!(t(2, 1, 1, 2, 1, 1));
        assert!(t(2, 1, 2, 3, 4, 3));
        assert!(t(2, 1, 1, 1, 2, 1));
        assert!(t(2, 1, 2, 1, 4, 1));
    }

    #[test]
    fn div() {
        fn t(xn: isize, xd: isize, yn: isize, yd: isize, rn: isize, rd: isize) -> bool {
            Number::compare(Number::combine(Number::ratio(xn, xd), Number::ratio(yn, yd), 0, 1, 0, 0, 0, 0, 1, 0), Number::ratio(rn, rd)) == Ordering::Equal
        }
        assert!(t(-2, 1, -2, 1, 1, 1));
        assert!(t(-2, 1, -1, 1, 2, 1));
        assert!(t(-2, 1, -2, 3, 3, 1));
        assert!(t(-2, 1, -1, 2, 4, 1));
        assert!(t(-2, 1, -1, 4, 8, 1));
        assert!(t(-2, 1, 1, 4, -8, 1));
        assert!(t(-2, 1, 1, 2, -4, 1));
        assert!(t(-2, 1, 2, 3, -3, 1));
        assert!(t(-2, 1, 1, 1, -2, 1));
        assert!(t(-2, 1, 2, 1, -1, 1));
        assert!(t(-1, 1, -2, 1, 1, 2));
        assert!(t(-1, 1, -1, 1, 1, 1));
        assert!(t(-1, 1, -2, 3, 3, 2));
        assert!(t(-1, 1, -1, 2, 2, 1));
        assert!(t(-1, 1, -1, 4, 4, 1));
        assert!(t(-1, 1, 1, 4, -4, 1));
        assert!(t(-1, 1, 1, 2, -2, 1));
        assert!(t(-1, 1, 2, 3, -3, 2));
        assert!(t(-1, 1, 1, 1, -1, 1));
        assert!(t(-1, 1, 2, 1, -1, 2));
        assert!(t(-2, 3, -2, 1, 1, 3));
        assert!(t(-2, 3, -1, 1, 2, 3));
        assert!(t(-2, 3, -2, 3, 1, 1));
        assert!(t(-2, 3, -1, 2, 4, 3));
        assert!(t(-2, 3, -1, 4, 8, 3));
        assert!(t(-2, 3, 1, 4, -8, 3));
        assert!(t(-2, 3, 1, 2, -4, 3));
        assert!(t(-2, 3, 2, 3, -1, 1));
        assert!(t(-2, 3, 1, 1, -2, 3));
        assert!(t(-2, 3, 2, 1, -1, 3));
        assert!(t(-1, 2, -2, 1, 1, 4));
        assert!(t(-1, 2, -1, 1, 1, 2));
        assert!(t(-1, 2, -2, 3, 3, 4));
        assert!(t(-1, 2, -1, 2, 1, 1));
        assert!(t(-1, 2, -1, 4, 2, 1));
        assert!(t(-1, 2, 1, 4, -2, 1));
        assert!(t(-1, 2, 1, 2, -1, 1));
        assert!(t(-1, 2, 2, 3, -3, 4));
        assert!(t(-1, 2, 1, 1, -1, 2));
        assert!(t(-1, 2, 2, 1, -1, 4));
        assert!(t(-1, 4, -2, 1, 1, 8));
        assert!(t(-1, 4, -1, 1, 1, 4));
        assert!(t(-1, 4, -2, 3, 3, 8));
        assert!(t(-1, 4, -1, 2, 1, 2));
        assert!(t(-1, 4, -1, 4, 1, 1));
        assert!(t(-1, 4, 1, 4, -1, 1));
        assert!(t(-1, 4, 1, 2, -1, 2));
        assert!(t(-1, 4, 2, 3, -3, 8));
        assert!(t(-1, 4, 1, 1, -1, 4));
        assert!(t(-1, 4, 2, 1, -1, 8));
        assert!(t(0, 1, -2, 1, 0, 1));
        assert!(t(0, 1, -1, 1, 0, 1));
        assert!(t(0, 1, -2, 3, 0, 1));
        assert!(t(0, 1, -1, 2, 0, 1));
        assert!(t(0, 1, -1, 4, 0, 1));
        assert!(t(0, 1, 1, 4, 0, 1));
        assert!(t(0, 1, 1, 2, 0, 1));
        assert!(t(0, 1, 2, 3, 0, 1));
        assert!(t(0, 1, 1, 1, 0, 1));
        assert!(t(0, 1, 2, 1, 0, 1));
        assert!(t(1, 4, -2, 1, -1, 8));
        assert!(t(1, 4, -1, 1, -1, 4));
        assert!(t(1, 4, -2, 3, -3, 8));
        assert!(t(1, 4, -1, 2, -1, 2));
        assert!(t(1, 4, -1, 4, -1, 1));
        assert!(t(1, 4, 1, 4, 1, 1));
        assert!(t(1, 4, 1, 2, 1, 2));
        assert!(t(1, 4, 2, 3, 3, 8));
        assert!(t(1, 4, 1, 1, 1, 4));
        assert!(t(1, 4, 2, 1, 1, 8));
        assert!(t(1, 2, -2, 1, -1, 4));
        assert!(t(1, 2, -1, 1, -1, 2));
        assert!(t(1, 2, -2, 3, -3, 4));
        assert!(t(1, 2, -1, 2, -1, 1));
        assert!(t(1, 2, -1, 4, -2, 1));
        assert!(t(1, 2, 1, 4, 2, 1));
        assert!(t(1, 2, 1, 2, 1, 1));
        assert!(t(1, 2, 2, 3, 3, 4));
        assert!(t(1, 2, 1, 1, 1, 2));
        assert!(t(1, 2, 2, 1, 1, 4));
        assert!(t(2, 3, -2, 1, -1, 3));
        assert!(t(2, 3, -1, 1, -2, 3));
        assert!(t(2, 3, -2, 3, -1, 1));
        assert!(t(2, 3, -1, 2, -4, 3));
        assert!(t(2, 3, -1, 4, -8, 3));
        assert!(t(2, 3, 1, 4, 8, 3));
        assert!(t(2, 3, 1, 2, 4, 3));
        assert!(t(2, 3, 2, 3, 1, 1));
        assert!(t(2, 3, 1, 1, 2, 3));
        assert!(t(2, 3, 2, 1, 1, 3));
        assert!(t(1, 1, -2, 1, -1, 2));
        assert!(t(1, 1, -1, 1, -1, 1));
        assert!(t(1, 1, -2, 3, -3, 2));
        assert!(t(1, 1, -1, 2, -2, 1));
        assert!(t(1, 1, -1, 4, -4, 1));
        assert!(t(1, 1, 1, 4, 4, 1));
        assert!(t(1, 1, 1, 2, 2, 1));
        assert!(t(1, 1, 2, 3, 3, 2));
        assert!(t(1, 1, 1, 1, 1, 1));
        assert!(t(1, 1, 2, 1, 1, 2));
        assert!(t(2, 1, -2, 1, -1, 1));
        assert!(t(2, 1, -1, 1, -2, 1));
        assert!(t(2, 1, -2, 3, -3, 1));
        assert!(t(2, 1, -1, 2, -4, 1));
        assert!(t(2, 1, -1, 4, -8, 1));
        assert!(t(2, 1, 1, 4, 8, 1));
        assert!(t(2, 1, 1, 2, 4, 1));
        assert!(t(2, 1, 2, 3, 3, 1));
        assert!(t(2, 1, 1, 1, 2, 1));
        assert!(t(2, 1, 2, 1, 1, 1));
    }

    #[test]
    fn add() {
        fn t(xn: isize, xd: isize, yn: isize, yd: isize, rn: isize, rd: isize) -> bool {
            Number::compare(Number::combine(Number::ratio(xn, xd), Number::ratio(yn, yd), 0, 1, 1, 0, 0, 0, 0, 1), Number::ratio(rn, rd)) == Ordering::Equal
        }
        assert!(t(-2, 1, -2, 1, -4, 1));
        assert!(t(-2, 1, -1, 1, -3, 1));
        assert!(t(-2, 1, -2, 3, -8, 3));
        assert!(t(-2, 1, -1, 2, -5, 2));
        assert!(t(-2, 1, -1, 4, -9, 4));
        assert!(t(-2, 1, 0, 1, -2, 1));
        assert!(t(-2, 1, 1, 4, -7, 4));
        assert!(t(-2, 1, 1, 2, -3, 2));
        assert!(t(-2, 1, 2, 3, -4, 3));
        assert!(t(-2, 1, 1, 1, -1, 1));
        assert!(t(-2, 1, 2, 1, 0, 1));
        assert!(t(-1, 1, -2, 1, -3, 1));
        assert!(t(-1, 1, -1, 1, -2, 1));
        assert!(t(-1, 1, -2, 3, -5, 3));
        assert!(t(-1, 1, -1, 2, -3, 2));
        assert!(t(-1, 1, -1, 4, -5, 4));
        assert!(t(-1, 1, 0, 1, -1, 1));
        assert!(t(-1, 1, 1, 4, -3, 4));
        assert!(t(-1, 1, 1, 2, -1, 2));
        assert!(t(-1, 1, 2, 3, -1, 3));
        assert!(t(-1, 1, 1, 1, 0, 1));
        assert!(t(-1, 1, 2, 1, 1, 1));
        assert!(t(-2, 3, -2, 1, -8, 3));
        assert!(t(-2, 3, -1, 1, -5, 3));
        assert!(t(-2, 3, -2, 3, -4, 3));
        assert!(t(-2, 3, -1, 2, -7, 6));
        assert!(t(-2, 3, -1, 4, -11, 12));
        assert!(t(-2, 3, 0, 1, -2, 3));
        assert!(t(-2, 3, 1, 4, -5, 12));
        assert!(t(-2, 3, 1, 2, -1, 6));
        assert!(t(-2, 3, 2, 3, 0, 1));
        assert!(t(-2, 3, 1, 1, 1, 3));
        assert!(t(-2, 3, 2, 1, 4, 3));
        assert!(t(-1, 2, -2, 1, -5, 2));
        assert!(t(-1, 2, -1, 1, -3, 2));
        assert!(t(-1, 2, -2, 3, -7, 6));
        assert!(t(-1, 2, -1, 2, -1, 1));
        assert!(t(-1, 2, -1, 4, -3, 4));
        assert!(t(-1, 2, 0, 1, -1, 2));
        assert!(t(-1, 2, 1, 4, -1, 4));
        assert!(t(-1, 2, 1, 2, 0, 1));
        assert!(t(-1, 2, 2, 3, 1, 6));
        assert!(t(-1, 2, 1, 1, 1, 2));
        assert!(t(-1, 2, 2, 1, 3, 2));
        assert!(t(-1, 4, -2, 1, -9, 4));
        assert!(t(-1, 4, -1, 1, -5, 4));
        assert!(t(-1, 4, -2, 3, -11, 12));
        assert!(t(-1, 4, -1, 2, -3, 4));
        assert!(t(-1, 4, -1, 4, -1, 2));
        assert!(t(-1, 4, 0, 1, -1, 4));
        assert!(t(-1, 4, 1, 4, 0, 1));
        assert!(t(-1, 4, 1, 2, 1, 4));
        assert!(t(-1, 4, 2, 3, 5, 12));
        assert!(t(-1, 4, 1, 1, 3, 4));
        assert!(t(-1, 4, 2, 1, 7, 4));
        assert!(t(0, 1, -2, 1, -2, 1));
        assert!(t(0, 1, -1, 1, -1, 1));
        assert!(t(0, 1, -2, 3, -2, 3));
        assert!(t(0, 1, -1, 2, -1, 2));
        assert!(t(0, 1, -1, 4, -1, 4));
        assert!(t(0, 1, 0, 1, 0, 1));
        assert!(t(0, 1, 1, 4, 1, 4));
        assert!(t(0, 1, 1, 2, 1, 2));
        assert!(t(0, 1, 2, 3, 2, 3));
        assert!(t(0, 1, 1, 1, 1, 1));
        assert!(t(0, 1, 2, 1, 2, 1));
        assert!(t(1, 4, -2, 1, -7, 4));
        assert!(t(1, 4, -1, 1, -3, 4));
        assert!(t(1, 4, -2, 3, -5, 12));
        assert!(t(1, 4, -1, 2, -1, 4));
        assert!(t(1, 4, -1, 4, 0, 1));
        assert!(t(1, 4, 0, 1, 1, 4));
        assert!(t(1, 4, 1, 4, 1, 2));
        assert!(t(1, 4, 1, 2, 3, 4));
        assert!(t(1, 4, 2, 3, 11, 12));
        assert!(t(1, 4, 1, 1, 5, 4));
        assert!(t(1, 4, 2, 1, 9, 4));
        assert!(t(1, 2, -2, 1, -3, 2));
        assert!(t(1, 2, -1, 1, -1, 2));
        assert!(t(1, 2, -2, 3, -1, 6));
        assert!(t(1, 2, -1, 2, 0, 1));
        assert!(t(1, 2, -1, 4, 1, 4));
        assert!(t(1, 2, 0, 1, 1, 2));
        assert!(t(1, 2, 1, 4, 3, 4));
        assert!(t(1, 2, 1, 2, 1, 1));
        assert!(t(1, 2, 2, 3, 7, 6));
        assert!(t(1, 2, 1, 1, 3, 2));
        assert!(t(1, 2, 2, 1, 5, 2));
        assert!(t(2, 3, -2, 1, -4, 3));
        assert!(t(2, 3, -1, 1, -1, 3));
        assert!(t(2, 3, -2, 3, 0, 1));
        assert!(t(2, 3, -1, 2, 1, 6));
        assert!(t(2, 3, -1, 4, 5, 12));
        assert!(t(2, 3, 0, 1, 2, 3));
        assert!(t(2, 3, 1, 4, 11, 12));
        assert!(t(2, 3, 1, 2, 7, 6));
        assert!(t(2, 3, 2, 3, 4, 3));
        assert!(t(2, 3, 1, 1, 5, 3));
        assert!(t(2, 3, 2, 1, 8, 3));
        assert!(t(1, 1, -2, 1, -1, 1));
        assert!(t(1, 1, -1, 1, 0, 1));
        assert!(t(1, 1, -2, 3, 1, 3));
        assert!(t(1, 1, -1, 2, 1, 2));
        assert!(t(1, 1, -1, 4, 3, 4));
        assert!(t(1, 1, 0, 1, 1, 1));
        assert!(t(1, 1, 1, 4, 5, 4));
        assert!(t(1, 1, 1, 2, 3, 2));
        assert!(t(1, 1, 2, 3, 5, 3));
        assert!(t(1, 1, 1, 1, 2, 1));
        assert!(t(1, 1, 2, 1, 3, 1));
        assert!(t(2, 1, -2, 1, 0, 1));
        assert!(t(2, 1, -1, 1, 1, 1));
        assert!(t(2, 1, -2, 3, 4, 3));
        assert!(t(2, 1, -1, 2, 3, 2));
        assert!(t(2, 1, -1, 4, 7, 4));
        assert!(t(2, 1, 0, 1, 2, 1));
        assert!(t(2, 1, 1, 4, 9, 4));
        assert!(t(2, 1, 1, 2, 5, 2));
        assert!(t(2, 1, 2, 3, 8, 3));
        assert!(t(2, 1, 1, 1, 3, 1));
        assert!(t(2, 1, 2, 1, 4, 1));
    }

    #[test]
    fn sub() {
        fn t(xn: isize, xd: isize, yn: isize, yd: isize, rn: isize, rd: isize) -> bool {
            Number::compare(Number::combine(Number::ratio(xn, xd), Number::ratio(yn, yd), 0, 1, -1, 0, 0, 0, 0, 1), Number::ratio(rn, rd)) == Ordering::Equal
        }
        assert!(t(-2, 1, -2, 1, 0, 1));
        assert!(t(-2, 1, -1, 1, -1, 1));
        assert!(t(-2, 1, -2, 3, -4, 3));
        assert!(t(-2, 1, -1, 2, -3, 2));
        assert!(t(-2, 1, -1, 4, -7, 4));
        assert!(t(-2, 1, 0, 1, -2, 1));
        assert!(t(-2, 1, 1, 4, -9, 4));
        assert!(t(-2, 1, 1, 2, -5, 2));
        assert!(t(-2, 1, 2, 3, -8, 3));
        assert!(t(-2, 1, 1, 1, -3, 1));
        assert!(t(-2, 1, 2, 1, -4, 1));
        assert!(t(-1, 1, -2, 1, 1, 1));
        assert!(t(-1, 1, -1, 1, 0, 1));
        assert!(t(-1, 1, -2, 3, -1, 3));
        assert!(t(-1, 1, -1, 2, -1, 2));
        assert!(t(-1, 1, -1, 4, -3, 4));
        assert!(t(-1, 1, 0, 1, -1, 1));
        assert!(t(-1, 1, 1, 4, -5, 4));
        assert!(t(-1, 1, 1, 2, -3, 2));
        assert!(t(-1, 1, 2, 3, -5, 3));
        assert!(t(-1, 1, 1, 1, -2, 1));
        assert!(t(-1, 1, 2, 1, -3, 1));
        assert!(t(-2, 3, -2, 1, 4, 3));
        assert!(t(-2, 3, -1, 1, 1, 3));
        assert!(t(-2, 3, -2, 3, 0, 1));
        assert!(t(-2, 3, -1, 2, -1, 6));
        assert!(t(-2, 3, -1, 4, -5, 12));
        assert!(t(-2, 3, 0, 1, -2, 3));
        assert!(t(-2, 3, 1, 4, -11, 12));
        assert!(t(-2, 3, 1, 2, -7, 6));
        assert!(t(-2, 3, 2, 3, -4, 3));
        assert!(t(-2, 3, 1, 1, -5, 3));
        assert!(t(-2, 3, 2, 1, -8, 3));
        assert!(t(-1, 2, -2, 1, 3, 2));
        assert!(t(-1, 2, -1, 1, 1, 2));
        assert!(t(-1, 2, -2, 3, 1, 6));
        assert!(t(-1, 2, -1, 2, 0, 1));
        assert!(t(-1, 2, -1, 4, -1, 4));
        assert!(t(-1, 2, 0, 1, -1, 2));
        assert!(t(-1, 2, 1, 4, -3, 4));
        assert!(t(-1, 2, 1, 2, -1, 1));
        assert!(t(-1, 2, 2, 3, -7, 6));
        assert!(t(-1, 2, 1, 1, -3, 2));
        assert!(t(-1, 2, 2, 1, -5, 2));
        assert!(t(-1, 4, -2, 1, 7, 4));
        assert!(t(-1, 4, -1, 1, 3, 4));
        assert!(t(-1, 4, -2, 3, 5, 12));
        assert!(t(-1, 4, -1, 2, 1, 4));
        assert!(t(-1, 4, -1, 4, 0, 1));
        assert!(t(-1, 4, 0, 1, -1, 4));
        assert!(t(-1, 4, 1, 4, -1, 2));
        assert!(t(-1, 4, 1, 2, -3, 4));
        assert!(t(-1, 4, 2, 3, -11, 12));
        assert!(t(-1, 4, 1, 1, -5, 4));
        assert!(t(-1, 4, 2, 1, -9, 4));
        assert!(t(0, 1, -2, 1, 2, 1));
        assert!(t(0, 1, -1, 1, 1, 1));
        assert!(t(0, 1, -2, 3, 2, 3));
        assert!(t(0, 1, -1, 2, 1, 2));
        assert!(t(0, 1, -1, 4, 1, 4));
        assert!(t(0, 1, 0, 1, 0, 1));
        assert!(t(0, 1, 1, 4, -1, 4));
        assert!(t(0, 1, 1, 2, -1, 2));
        assert!(t(0, 1, 2, 3, -2, 3));
        assert!(t(0, 1, 1, 1, -1, 1));
        assert!(t(0, 1, 2, 1, -2, 1));
        assert!(t(1, 4, -2, 1, 9, 4));
        assert!(t(1, 4, -1, 1, 5, 4));
        assert!(t(1, 4, -2, 3, 11, 12));
        assert!(t(1, 4, -1, 2, 3, 4));
        assert!(t(1, 4, -1, 4, 1, 2));
        assert!(t(1, 4, 0, 1, 1, 4));
        assert!(t(1, 4, 1, 4, 0, 1));
        assert!(t(1, 4, 1, 2, -1, 4));
        assert!(t(1, 4, 2, 3, -5, 12));
        assert!(t(1, 4, 1, 1, -3, 4));
        assert!(t(1, 4, 2, 1, -7, 4));
        assert!(t(1, 2, -2, 1, 5, 2));
        assert!(t(1, 2, -1, 1, 3, 2));
        assert!(t(1, 2, -2, 3, 7, 6));
        assert!(t(1, 2, -1, 2, 1, 1));
        assert!(t(1, 2, -1, 4, 3, 4));
        assert!(t(1, 2, 0, 1, 1, 2));
        assert!(t(1, 2, 1, 4, 1, 4));
        assert!(t(1, 2, 1, 2, 0, 1));
        assert!(t(1, 2, 2, 3, -1, 6));
        assert!(t(1, 2, 1, 1, -1, 2));
        assert!(t(1, 2, 2, 1, -3, 2));
        assert!(t(2, 3, -2, 1, 8, 3));
        assert!(t(2, 3, -1, 1, 5, 3));
        assert!(t(2, 3, -2, 3, 4, 3));
        assert!(t(2, 3, -1, 2, 7, 6));
        assert!(t(2, 3, -1, 4, 11, 12));
        assert!(t(2, 3, 0, 1, 2, 3));
        assert!(t(2, 3, 1, 4, 5, 12));
        assert!(t(2, 3, 1, 2, 1, 6));
        assert!(t(2, 3, 2, 3, 0, 1));
        assert!(t(2, 3, 1, 1, -1, 3));
        assert!(t(2, 3, 2, 1, -4, 3));
        assert!(t(1, 1, -2, 1, 3, 1));
        assert!(t(1, 1, -1, 1, 2, 1));
        assert!(t(1, 1, -2, 3, 5, 3));
        assert!(t(1, 1, -1, 2, 3, 2));
        assert!(t(1, 1, -1, 4, 5, 4));
        assert!(t(1, 1, 0, 1, 1, 1));
        assert!(t(1, 1, 1, 4, 3, 4));
        assert!(t(1, 1, 1, 2, 1, 2));
        assert!(t(1, 1, 2, 3, 1, 3));
        assert!(t(1, 1, 1, 1, 0, 1));
        assert!(t(1, 1, 2, 1, -1, 1));
        assert!(t(2, 1, -2, 1, 4, 1));
        assert!(t(2, 1, -1, 1, 3, 1));
        assert!(t(2, 1, -2, 3, 8, 3));
        assert!(t(2, 1, -1, 2, 5, 2));
        assert!(t(2, 1, -1, 4, 9, 4));
        assert!(t(2, 1, 0, 1, 2, 1));
        assert!(t(2, 1, 1, 4, 7, 4));
        assert!(t(2, 1, 1, 2, 3, 2));
        assert!(t(2, 1, 2, 3, 4, 3));
        assert!(t(2, 1, 1, 1, 1, 1));
        assert!(t(2, 1, 2, 1, 0, 1));
    }

}

pub struct Combine {
    x: Clog,
    y: Clog,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
    f: isize,
    g: isize,
    h: isize,
}

/*
ingestions
-x: -a -b c d -e -f g h
-y: -a b -c d -e f -g h
1/x: c d a b g h e f
1/y: b a d c f e h g
2x: a/2 b/2 c d e/2 f/2 g h
2x: a b 2c 2d e f 2g 2h
2y: a/2 b c/2 d e/2 f g/2 h
2y: a 2b c 2d e 2f g 2h
x-1: a b a+c b+d e f e+g f+h
y-1: a a+b c c+d e e+f g g+h

egestions
1/z: e f g h a b c d
-z: -a -b -c -d e f g h
2z: a b c d e/2 f/2 g/2 h/2
2z: 2a 2b 2c 2d e f g h
z-1: a-e b-f c-g d-h e f g h

special outputs
s(0,0) = d/h
s(0,1) = (c+d)/(g+h)
s(1,0) = (b+d)/(f+h)
s(1,1) = (a+b+c+d)/(e+f+g+h)
s(1/2,y) = a/2+c b/2+d e/2+g f/2+h
s(1/2,y) = a+2c b+2d e+2g f+2h
s(x,1/2) = a/2+b c/2+d e/2+f g/2+h
s(x,1/2) = a+2b c+2d e+2f g+2h
s(0, y) = c d g h
s(x, 0) = b d f h
s(1, y) = a+c b+d e+g f+h
s(x, 1) = a+b c+d e+f g+h
s(-1, y) = c-a d-b g-e h-f
s(x, -1) = b-a d-c f-e h-g

zeros in domain
signums: d, b+d, c+d, a+b+c+d

poles in domain
signums: h, f+h, g+h, e+f+g+h
 */

pub fn new(x: Number, y: Number, mut a: isize, mut b: isize, mut c: isize, mut d: isize, mut e: isize, mut f: isize, mut g: isize, mut h: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {

    fn as_homographic(x: Number, nx: isize, n: isize, dx:isize, d: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {
        let (special, primer, ratio, homographic) = homographic::new(x, nx, n, dx, d);
        (special, primer, ratio, homographic, None)
    }

    macro_rules! turn_x {
        () => {
            swap(&mut a, &mut c);
            swap(&mut b, &mut d);
            swap(&mut e, &mut g);
            swap(&mut f, &mut h);
        };
    }

    macro_rules! turn_y {
        () => {
            swap(&mut a, &mut b);
            swap(&mut c, &mut d);
            swap(&mut e, &mut f);
            swap(&mut g, &mut h);
        };
    }

    macro_rules! reflect_x {
        () => {
            a *= -1;
            b *= -1;
            e *= -1;
            f *= -1;
        };
    }

    macro_rules! reflect_y {
        () => {
            a *= -1;
            c *= -1;
            e *= -1;
            g *= -1;
        };
    }

    macro_rules! ground_x {
        () => {
            turn_x!();
            reflect_x!();
        };
    }

    macro_rules! ground_y {
        () => {
            turn_y!();
            reflect_y!();
        };
    }

    if let Number::Special(special) = x {
        match special {
            protocol::Special::NegOne => {
                as_homographic(y, c.checked_sub(a).unwrap(), d.checked_sub(b).unwrap(), g.checked_sub(e).unwrap(), h.checked_sub(f).unwrap())
            },
            protocol::Special::Zero => {
                as_homographic(y, c, d, g, h)
            },
            protocol::Special::PosOne => {
                as_homographic(y, c.checked_add(a).unwrap(), d.checked_add(b).unwrap(), g.checked_add(e).unwrap(), h.checked_add(f).unwrap())
            },
        }
    }
    else if let Number::Special(special) = y {
        match special {
            protocol::Special::NegOne => {
                as_homographic(x, b.checked_sub(a).unwrap(), d.checked_sub(c).unwrap(), f.checked_sub(e).unwrap(), h.checked_sub(g).unwrap())
            },
            protocol::Special::Zero => {
                as_homographic(x, b, d, f, h)
            },
            protocol::Special::PosOne => {
                as_homographic(x, b.checked_add(a).unwrap(), d.checked_add(c).unwrap(), f.checked_add(e).unwrap(), h.checked_add(g).unwrap())
            },
        }
    }
    else {

        let (x_primer, x_clog) = x.unwrap_other();
        match x_primer {
            Some(protocol::Primer::Turn) => {
                turn_x!();
            },
            Some(protocol::Primer::Reflect) => {
                reflect_x!();
            },
            Some(protocol::Primer::Ground) => {
                ground_x!();
            },
            None => {},
        }

        let (y_primer, y_clog) = y.unwrap_other();
        match y_primer {
            Some(protocol::Primer::Turn) => {
                turn_y!();
            },
            Some(protocol::Primer::Reflect) => {
                reflect_y!();
            },
            Some(protocol::Primer::Ground) => {
                ground_y!();
            },
            None => {},
        }

        Combine::new(x_clog, y_clog, a, b, c, d, e, f, g, h)
    }
}

impl Combine {

    fn new(x: Clog, y: Clog, a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, g: isize, h: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {
        (Combine { x, y, a, b, c, d, e, f, g, h }).prime()
    }

    fn prime(mut self) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {
        if self.are_singularities_outside_domain() {
            if let Ok(primer) = self.primer_egest() {
                return (None, primer, None, None, Some(self));
            }
        }
        match self.prime_ingest() {
            Some((special, primer, ratio, homographic)) => (special, primer, ratio, homographic, None),
            None => self.prime(),
        }
    }

    fn are_singularities_outside_domain(&self) -> bool {
        self.is_zero_outside_domain() && self.is_pole_outside_domain()
    }

    fn is_zero_outside_domain(&self) -> bool {
        Combine::is_domain_amenable(self.a, self.b, self.c, self.d)
    }

    fn is_pole_outside_domain(&self) -> bool {
        Combine::is_domain_amenable(self.e, self.f, self.g, self.h)
    }

    fn is_domain_amenable(mxy: isize, mx: isize, my: isize, m: isize) -> bool {
        let s = m.signum();
        s != 0
            && s == m.checked_add(mx).unwrap().signum()
            && s == m.checked_add(my).unwrap().signum()
            && s == m.checked_add(mxy.checked_add(mx.checked_add(my).unwrap()).unwrap()).unwrap().signum()
    }

    fn primer_egest(&mut self) -> Result<Option<protocol::Primer>, isize> {
        // FIXME: implement me
        Ok(None)
    }

    fn prime_ingest(&mut self) -> Option<(Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>)> {
        // FIXME: implement me
        Some((Some(protocol::Special::Zero), None, None, None))
    }
}

impl Strategy for Combine {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        // FIXME: implement me
        Ok(None)
    }

}
