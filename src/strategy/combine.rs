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

use crate::protocol;
use crate::strategy::homographic;
use crate::strategy::homographic::Homographic;
use crate::strategy::ratio;
use crate::strategy::ratio::Ratio;
use crate::strategy::support;
use crate::strategy::Strategy;
use crate::Clog;
use crate::Number;
use std::mem::swap;
use std::rc::Rc;

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn mul() {
        fn t(xn: isize, xd: isize, yn: isize, yd: isize, rn: isize, rd: isize) -> bool {
            Number::compare(
                Number::combine(
                    Number::ratio(xn, xd),
                    Number::ratio(yn, yd),
                    1,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                ),
                Number::ratio(rn, rd),
            ) == Ordering::Equal
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
            Number::compare(
                Number::combine(
                    Number::ratio(xn, xd),
                    Number::ratio(yn, yd),
                    0,
                    1,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0,
                ),
                Number::ratio(rn, rd),
            ) == Ordering::Equal
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
            Number::compare(
                Number::combine(
                    Number::ratio(xn, xd),
                    Number::ratio(yn, yd),
                    0,
                    1,
                    1,
                    0,
                    0,
                    0,
                    0,
                    1,
                ),
                Number::ratio(rn, rd),
            ) == Ordering::Equal
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
            Number::compare(
                Number::combine(
                    Number::ratio(xn, xd),
                    Number::ratio(yn, yd),
                    0,
                    1,
                    -1,
                    0,
                    0,
                    0,
                    0,
                    1,
                ),
                Number::ratio(rn, rd),
            ) == Ordering::Equal
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

    #[test]
    fn egest0() {
        assert_eq!(
            Number::compare(
                Number::combine(
                    Number::ratio(-2, 3),
                    Number::ratio(2, 3),
                    0,
                    0,
                    0,
                    1,
                    0,
                    0,
                    1,
                    1
                ),
                Number::ratio(3, 5)
            ),
            Ordering::Equal
        );
    }
}

pub struct Combine {
    x: Rc<Clog>,
    y: Rc<Clog>,
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

pub fn new(
    x: Number,
    y: Number,
    mut a: isize,
    mut b: isize,
    mut c: isize,
    mut d: isize,
    mut e: isize,
    mut f: isize,
    mut g: isize,
    mut h: isize,
) -> (
    Option<protocol::Special>,
    Option<protocol::Primer>,
    Option<Ratio>,
    Option<Homographic>,
    Option<Combine>,
) {
    fn as_homographic(
        x: Number,
        nx: isize,
        n: isize,
        dx: isize,
        d: isize,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
        Option<Combine>,
    ) {
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
            protocol::Special::NegOne => as_homographic(
                y,
                c.checked_sub(a).unwrap(),
                d.checked_sub(b).unwrap(),
                g.checked_sub(e).unwrap(),
                h.checked_sub(f).unwrap(),
            ),
            protocol::Special::Zero => as_homographic(y, c, d, g, h),
            protocol::Special::PosOne => as_homographic(
                y,
                c.checked_add(a).unwrap(),
                d.checked_add(b).unwrap(),
                g.checked_add(e).unwrap(),
                h.checked_add(f).unwrap(),
            ),
        }
    } else if let Number::Special(special) = y {
        match special {
            protocol::Special::NegOne => as_homographic(
                x,
                b.checked_sub(a).unwrap(),
                d.checked_sub(c).unwrap(),
                f.checked_sub(e).unwrap(),
                h.checked_sub(g).unwrap(),
            ),
            protocol::Special::Zero => as_homographic(x, b, d, f, h),
            protocol::Special::PosOne => as_homographic(
                x,
                b.checked_add(a).unwrap(),
                d.checked_add(c).unwrap(),
                f.checked_add(e).unwrap(),
                h.checked_add(g).unwrap(),
            ),
        }
    } else {
        let (x_primer, x_clog) = x.unwrap_other();
        match x_primer {
            Some(protocol::Primer::Turn) => {
                turn_x!();
            }
            Some(protocol::Primer::Reflect) => {
                reflect_x!();
            }
            Some(protocol::Primer::Ground) => {
                ground_x!();
            }
            None => {}
        }

        let (y_primer, y_clog) = y.unwrap_other();
        match y_primer {
            Some(protocol::Primer::Turn) => {
                turn_y!();
            }
            Some(protocol::Primer::Reflect) => {
                reflect_y!();
            }
            Some(protocol::Primer::Ground) => {
                ground_y!();
            }
            None => {}
        }

        Combine::new(Rc::new(x_clog), Rc::new(y_clog), a, b, c, d, e, f, g, h)
    }
}

impl Combine {
    fn new(
        x: Rc<Clog>,
        y: Rc<Clog>,
        a: isize,
        b: isize,
        c: isize,
        d: isize,
        e: isize,
        f: isize,
        g: isize,
        h: isize,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
        Option<Combine>,
    ) {
        (Combine {
            x,
            y,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        })
        .prime()
    }

    fn prime(
        mut self,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
        Option<Combine>,
    ) {
        if self.are_singularities_outside_domain() {
            if let Ok(primer) = self.primer_egest() {
                return (None, primer, None, None, Some(self));
            }
        }
        match self.prime_ingest() {
            Ok((special, primer, ratio, homographic)) => {
                (special, primer, ratio, homographic, None)
            }
            Err(myself) => myself.prime(),
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
            && s == m
                .checked_add(mxy.checked_add(mx.checked_add(my).unwrap()).unwrap())
                .unwrap()
                .signum()
    }

    fn primer_egest(&mut self) -> Result<Option<protocol::Primer>, isize> {
        let (nmin, dmin, nmax, dmax) = self.image_extremes();
        if support::less_than_minus_one(nmax, dmax) {
            Ok(Some(self.ground()))
        } else if support::greater_than_minus_one(nmin, dmin) && support::less_than_zero(nmax, dmax)
        {
            Ok(Some(self.reflect()))
        } else if support::greater_than_zero(nmin, dmin) && support::less_than_one(nmax, dmax) {
            Ok(None)
        } else if support::greater_than_one(nmin, dmin) {
            Ok(Some(self.turn()))
        } else {
            Err(0)
        }
    }

    fn prime_ingest(
        mut self,
    ) -> Result<
        (
            Option<protocol::Special>,
            Option<protocol::Primer>,
            Option<Ratio>,
            Option<Homographic>,
        ),
        Combine,
    > {
        match Rc::get_mut(&mut self.x).unwrap().egest() {
            None => {
                let (ny, n, dy, d) = self.value_at_end_of_x();
                return Ok(homographic::new(
                    Number::Other(None, Rc::try_unwrap(self.y).ok().unwrap()),
                    ny,
                    n,
                    dy,
                    d,
                ));
            }
            Some(protocol::Reduction::Amplify) => {
                self.amplify_x();
            }
            Some(protocol::Reduction::Uncover) => {
                self.uncover_x();
            }
        }
        match Rc::get_mut(&mut self.y).unwrap().egest() {
            None => {
                let (nx, n, dx, d) = self.value_at_end_of_y();
                return Ok(homographic::new(
                    Number::Other(None, Rc::try_unwrap(self.x).ok().unwrap()),
                    nx,
                    n,
                    dx,
                    d,
                ));
            }
            Some(protocol::Reduction::Amplify) => {
                self.amplify_y();
            }
            Some(protocol::Reduction::Uncover) => {
                self.uncover_y();
            }
        }
        Err(self)
    }

    fn value_at_end_of_x(&self) -> (isize, isize, isize, isize) {
        if support::is_even(self.a)
            && support::is_even(self.b)
            && support::is_even(self.e)
            && support::is_even(self.f)
        {
            (
                self.c.checked_add(self.a / 2).unwrap(),
                self.d.checked_add(self.b / 2).unwrap(),
                self.g.checked_add(self.e / 2).unwrap(),
                self.h.checked_add(self.f / 2).unwrap(),
            )
        } else {
            (
                self.a.checked_add(self.c.checked_mul(2).unwrap()).unwrap(),
                self.b.checked_add(self.d.checked_mul(2).unwrap()).unwrap(),
                self.e.checked_add(self.g.checked_mul(2).unwrap()).unwrap(),
                self.f.checked_add(self.h.checked_mul(2).unwrap()).unwrap(),
            )
        }
    }

    fn value_at_end_of_y(&self) -> (isize, isize, isize, isize) {
        if support::is_even(self.a)
            && support::is_even(self.c)
            && support::is_even(self.e)
            && support::is_even(self.g)
        {
            (
                self.b.checked_add(self.a / 2).unwrap(),
                self.d.checked_add(self.c / 2).unwrap(),
                self.f.checked_add(self.e / 2).unwrap(),
                self.h.checked_add(self.g / 2).unwrap(),
            )
        } else {
            (
                self.a.checked_add(self.b.checked_mul(2).unwrap()).unwrap(),
                self.c.checked_add(self.d.checked_mul(2).unwrap()).unwrap(),
                self.e.checked_add(self.f.checked_mul(2).unwrap()).unwrap(),
                self.g.checked_add(self.h.checked_mul(2).unwrap()).unwrap(),
            )
        }
    }

    fn amplify_x(&mut self) {
        if support::is_even(self.a)
            && support::is_even(self.b)
            && support::is_even(self.e)
            && support::is_even(self.f)
        {
            self.a /= 2;
            self.b /= 2;
            self.e /= 2;
            self.f /= 2;
        } else {
            self.c = self.c.checked_mul(2).unwrap();
            self.d = self.d.checked_mul(2).unwrap();
            self.g = self.g.checked_mul(2).unwrap();
            self.h = self.h.checked_mul(2).unwrap();
        }
    }

    fn amplify_y(&mut self) {
        if support::is_even(self.a)
            && support::is_even(self.c)
            && support::is_even(self.e)
            && support::is_even(self.g)
        {
            self.a /= 2;
            self.c /= 2;
            self.e /= 2;
            self.g /= 2;
        } else {
            self.b = self.b.checked_mul(2).unwrap();
            self.d = self.d.checked_mul(2).unwrap();
            self.f = self.f.checked_mul(2).unwrap();
            self.h = self.h.checked_mul(2).unwrap();
        }
    }

    fn uncover_x(&mut self) {
        self.turn_x();
        self.shift_x();
    }

    fn uncover_y(&mut self) {
        self.turn_y();
        self.shift_y();
    }

    fn turn_x(&mut self) {
        swap(&mut self.a, &mut self.c);
        swap(&mut self.b, &mut self.d);
        swap(&mut self.e, &mut self.g);
        swap(&mut self.f, &mut self.h);
    }

    fn turn_y(&mut self) {
        swap(&mut self.a, &mut self.b);
        swap(&mut self.c, &mut self.d);
        swap(&mut self.e, &mut self.f);
        swap(&mut self.g, &mut self.h);
    }

    fn shift_x(&mut self) {
        self.c = self.c.checked_add(self.a).unwrap();
        self.d = self.d.checked_add(self.b).unwrap();
        self.g = self.g.checked_add(self.e).unwrap();
        self.h = self.h.checked_add(self.f).unwrap();
    }

    fn shift_y(&mut self) {
        self.b = self.b.checked_add(self.a).unwrap();
        self.d = self.d.checked_add(self.c).unwrap();
        self.f = self.f.checked_add(self.e).unwrap();
        self.h = self.h.checked_add(self.g).unwrap();
    }

    fn image_extremes(&self) -> (isize, isize, isize, isize) {
        let (nmin, dmin) = self.value_at_0_0();
        let (n, d) = self.value_at_0_1();
        let (nmin, dmin, nmax, dmax) = support::updated_range(nmin, dmin, nmin, dmin, n, d);
        let (n, d) = self.value_at_1_0();
        let (nmin, dmin, nmax, dmax) = support::updated_range(nmin, dmin, nmax, dmax, n, d);
        let (n, d) = self.value_at_1_1();
        support::updated_range(nmin, dmin, nmax, dmax, n, d)
    }

    fn ground(&mut self) -> protocol::Primer {
        self.turn();
        self.reflect();
        protocol::Primer::Ground
    }

    fn reflect(&mut self) -> protocol::Primer {
        self.a = -self.a;
        self.b = -self.b;
        self.c = -self.c;
        self.d = -self.d;
        protocol::Primer::Reflect
    }

    fn turn(&mut self) -> protocol::Primer {
        swap(&mut self.a, &mut self.e);
        swap(&mut self.b, &mut self.f);
        swap(&mut self.c, &mut self.g);
        swap(&mut self.d, &mut self.h);
        protocol::Primer::Turn
    }

    fn value_at_0_0(&self) -> (isize, isize) {
        (self.d, self.h)
    }

    fn value_at_0_1(&self) -> (isize, isize) {
        (
            self.c.checked_add(self.d).unwrap(),
            self.g.checked_add(self.h).unwrap(),
        )
    }

    fn value_at_1_0(&self) -> (isize, isize) {
        (
            self.b.checked_add(self.d).unwrap(),
            self.f.checked_add(self.h).unwrap(),
        )
    }

    fn value_at_1_1(&self) -> (isize, isize) {
        (
            self.a
                .checked_add(
                    self.b
                        .checked_add(self.c.checked_add(self.d).unwrap())
                        .unwrap(),
                )
                .unwrap(),
            self.e
                .checked_add(
                    self.f
                        .checked_add(self.g.checked_add(self.h).unwrap())
                        .unwrap(),
                )
                .unwrap(),
        )
    }

    fn reduction_egest(&mut self) -> Result<Option<protocol::Reduction>, isize> {
        let (nmin, dmin, nmax, dmax) = self.image_extremes();
        // FIXME: remove sanity checks?
        if support::not_greater_than_zero(nmin, dmin) {
            panic!("logic error");
        }
        if support::not_less_than_one(nmax, dmax) {
            panic!("logic error");
        }
        if support::less_than_one_half(nmax, dmax) {
            Ok(Some(self.amplify()))
        } else if support::greater_than_one_half(nmin, dmin) {
            Ok(Some(self.uncover()))
        } else {
            Err(0)
        }
    }

    fn amplify(&mut self) -> protocol::Reduction {
        if support::is_even(self.e)
            && support::is_even(self.f)
            && support::is_even(self.g)
            && support::is_even(self.h)
        {
            self.e /= 2;
            self.f /= 2;
            self.g /= 2;
            self.h /= 2;
        } else {
            self.a = self.a.checked_mul(2).unwrap();
            self.b = self.b.checked_mul(2).unwrap();
            self.c = self.c.checked_mul(2).unwrap();
            self.d = self.d.checked_mul(2).unwrap();
        }
        protocol::Reduction::Amplify
    }

    fn uncover(&mut self) -> protocol::Reduction {
        self.turn();
        self.shift();
        protocol::Reduction::Uncover
    }

    fn shift(&mut self) {
        self.a = self.a.checked_sub(self.e).unwrap();
        self.b = self.b.checked_sub(self.f).unwrap();
        self.c = self.c.checked_sub(self.g).unwrap();
        self.d = self.d.checked_sub(self.h).unwrap();
    }

    fn simple_clog() -> Rc<Clog> {
        let (_, _, ratio) = ratio::new_i(1, 2);
        Rc::new(Clog {
            strategy: Box::new(ratio.unwrap()),
        })
    }

    fn promote_clog(clog: Rc<Clog>) -> Number {
        Number::Other(None, Rc::try_unwrap(clog).ok().unwrap())
    }

    fn shutdown_x(&mut self) -> Number {
        let moved = Rc::clone(&self.x);
        self.x = Combine::simple_clog();
        Combine::promote_clog(moved)
    }

    fn shutdown_y(&mut self) -> Number {
        let moved = Rc::clone(&self.y);
        self.y = Combine::simple_clog();
        Combine::promote_clog(moved)
    }

    fn reduction_ingest(&mut self) -> (Option<Ratio>, Option<Homographic>) {
        match Rc::get_mut(&mut self.x).unwrap().egest() {
            None => {
                let (ny, n, dy, d) = self.value_at_end_of_x();
                match homographic::new(self.shutdown_y(), ny, n, dy, d) {
                    (None, None, ratio, homographic) => {
                        return (ratio, homographic);
                    }
                    _ => {
                        panic!("logic error");
                    }
                }
            }
            Some(protocol::Reduction::Amplify) => {
                self.amplify_x();
            }
            Some(protocol::Reduction::Uncover) => {
                self.uncover_x();
            }
        }
        match Rc::get_mut(&mut self.y).unwrap().egest() {
            None => {
                let (nx, n, dx, d) = self.value_at_end_of_y();
                match homographic::new(self.shutdown_x(), nx, n, dx, d) {
                    (None, None, ratio, homographic) => {
                        return (ratio, homographic);
                    }
                    _ => {
                        panic!("logic error");
                    }
                }
            }
            Some(protocol::Reduction::Amplify) => {
                self.amplify_y();
            }
            Some(protocol::Reduction::Uncover) => {
                self.uncover_y();
            }
        }
        (None, None)
    }
}

impl Strategy for Combine {
    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        if self.are_singularities_outside_domain() {
            if let Ok(reduction) = self.reduction_egest() {
                return Ok(reduction);
            }
        }
        match self.reduction_ingest() {
            (_, Some(homographic)) => Err(Box::new(homographic)),
            (Some(ratio), _) => Err(Box::new(ratio)),
            _ => self.egest(),
        }
    }
}
