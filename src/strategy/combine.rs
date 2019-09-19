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
use crate::Number;
use crate::strategy::ratio::Ratio;
use crate::strategy::homographic::Homographic;

#[cfg(test)]
mod tests {

    use super::*;

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
}

pub fn new(x: Number, y: Number, nxy: isize, nx: isize, ny: isize, n: isize, dxy: isize, dx: isize, dy: isize, d: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {
    (Some(protocol::Special::Zero), None, None, None, None)
}

impl Strategy for Combine {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}
