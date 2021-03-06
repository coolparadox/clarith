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
use crate::strategy::ratio;
use crate::strategy::ratio::Ratio;
use crate::strategy::support;
use crate::strategy::Strategy;
use crate::Clog;
use crate::Number;
use std::mem::swap;

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    // known well defined values from ratio unit tests

    fn neg_two() -> Number {
        Number::ratio(-2, 1)
    }

    fn neg_one() -> Number {
        Number::ratio(-1, 1)
    }

    fn neg_two_thirds() -> Number {
        Number::ratio(-2, 3)
    }

    fn neg_one_half() -> Number {
        Number::ratio(-1, 2)
    }

    fn neg_one_fourth() -> Number {
        Number::ratio(-1, 4)
    }

    fn zero() -> Number {
        Number::ratio(0, 1)
    }

    fn one_fourth() -> Number {
        Number::ratio(1, 4)
    }

    fn one_half() -> Number {
        Number::ratio(1, 2)
    }

    fn two_thirds() -> Number {
        Number::ratio(2, 3)
    }

    fn one() -> Number {
        Number::ratio(1, 1)
    }

    fn two() -> Number {
        Number::ratio(2, 1)
    }

    // Support

    fn assert_eq(n1: Number, n2: Number) {
        assert_eq!(Number::compare(n1, n2), Ordering::Equal);
    }

    // Tests

    #[test]
    fn t_1111() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 1, 1, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), one());
        // t(neg_one(), one());
        t(neg_two_thirds(), one());
        t(neg_one_half(), one());
        t(neg_one_fourth(), one());
        t(zero(), one());
        t(one_fourth(), one());
        t(one_half(), one());
        t(two_thirds(), one());
        t(one(), one());
        t(two(), one());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_1111() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 1, 1, 1)
        }
        h(neg_one());
    }

    #[test]
    fn t_1110() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 1, 1, 0)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), one_half());
        t(neg_one(), zero());
        t(neg_two_thirds(), neg_one_half());
        t(neg_one_half(), neg_one());
        t(neg_one_fourth(), Number::ratio(-3, 1));
        // t(zero(), inf());
        t(one_fourth(), Number::ratio(5, 1));
        t(one_half(), Number::ratio(3, 1));
        t(two_thirds(), Number::ratio(5, 2));
        t(one(), two());
        t(two(), Number::ratio(3, 2));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_1110() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 1, 1, 0)
        }
        h(zero());
    }

    #[test]
    fn t_1101() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 1, 0, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), neg_one());
        t(neg_one(), zero());
        t(neg_two_thirds(), Number::ratio(1, 3));
        t(neg_one_half(), one_half());
        t(neg_one_fourth(), Number::ratio(3, 4));
        t(zero(), one());
        t(one_fourth(), Number::ratio(5, 4));
        t(one_half(), Number::ratio(3, 2));
        t(two_thirds(), Number::ratio(5, 3));
        t(one(), two());
        t(two(), Number::ratio(3, 1));
    }

    #[test]
    fn t_1011() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 0, 1, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), two());
        // t(neg_one(), neg_inf());
        t(neg_two_thirds(), neg_two());
        t(neg_one_half(), neg_one());
        t(neg_one_fourth(), Number::ratio(-1, 3));
        t(zero(), zero());
        t(one_fourth(), Number::ratio(1, 5));
        t(one_half(), Number::ratio(1, 3));
        t(two_thirds(), Number::ratio(2, 5));
        t(one(), one_half());
        t(two(), two_thirds());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_1011() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 1, 0, 1, 1)
        }
        h(neg_one());
    }

    #[test]
    fn t_0111() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 1, 1, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), neg_one());
        // t(neg_one(), inf());
        t(neg_two_thirds(), Number::ratio(3, 1));
        t(neg_one_half(), two());
        t(neg_one_fourth(), Number::ratio(4, 3));
        t(zero(), one());
        t(one_fourth(), Number::ratio(4, 5));
        t(one_half(), Number::ratio(2, 3));
        t(two_thirds(), Number::ratio(3, 5));
        t(one(), one_half());
        t(two(), Number::ratio(1, 3));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_0111() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 1, 1, 1)
        }
        h(neg_one());
    }

    #[test]
    fn t_0110() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 1, 1, 0)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), neg_one_half());
        t(neg_one(), neg_one());
        t(neg_two_thirds(), Number::ratio(-3, 2));
        t(neg_one_half(), neg_two());
        t(neg_one_fourth(), Number::ratio(-4, 1));
        // t(zero(), inf());
        t(one_fourth(), Number::ratio(4, 1));
        t(one_half(), two());
        t(two_thirds(), Number::ratio(3, 2));
        t(one(), one());
        t(two(), one_half());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_0110() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 1, 1, 0)
        }
        h(zero());
    }

    #[test]
    fn t_0101() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 1, 0, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), one());
        t(neg_one(), one());
        t(neg_two_thirds(), one());
        t(neg_one_half(), one());
        t(neg_one_fourth(), one());
        t(zero(), one());
        t(one_fourth(), one());
        t(one_half(), one());
        t(two_thirds(), one());
        t(one(), one());
        t(two(), one());
    }

    #[test]
    fn t_0011() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 1, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), zero());
        // t(neg_one(), zero());
        t(neg_two_thirds(), zero());
        t(neg_one_half(), zero());
        t(neg_one_fourth(), zero());
        t(zero(), zero());
        t(one_fourth(), zero());
        t(one_half(), zero());
        t(two_thirds(), zero());
        t(one(), zero());
        t(two(), zero());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_0011() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 1, 1)
        }
        h(neg_one());
    }

    #[test]
    fn t_0010() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 1, 0)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), zero());
        t(neg_one(), zero());
        t(neg_two_thirds(), zero());
        t(neg_one_half(), zero());
        t(neg_one_fourth(), zero());
        // t(zero(), zero());
        t(one_fourth(), zero());
        t(one_half(), zero());
        t(two_thirds(), zero());
        t(one(), zero());
        t(two(), zero());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_0010() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 1, 0)
        }
        h(zero());
    }

    #[test]
    fn t_0001() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 0, 1)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), zero());
        t(neg_one(), zero());
        t(neg_two_thirds(), zero());
        t(neg_one_half(), zero());
        t(neg_one_fourth(), zero());
        t(zero(), zero());
        t(one_fourth(), zero());
        t(one_half(), zero());
        t(two_thirds(), zero());
        t(one(), zero());
        t(two(), zero());
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn t_0000() {
        Number::homographic(one(), 0, 0, 0, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_0021() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 0, 0, 2, 1)
        }
        h(neg_one_half());
    }

    #[test]
    fn t_pppp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 2, 3, 4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(1, 3));
        t(neg_one(), Number::ratio(1, 1));
        t(neg_two_thirds(), Number::ratio(5, 7));
        t(neg_one_half(), Number::ratio(2, 3));
        t(neg_one_fourth(), Number::ratio(5, 8));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(7, 12));
        t(one_half(), Number::ratio(4, 7));
        t(two_thirds(), Number::ratio(13, 23));
        t(one(), Number::ratio(5, 9));
        t(two(), Number::ratio(7, 13));
    }

    #[test]
    fn t_pppn() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 2, 3, 4, -5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(1, 13));
        t(neg_one(), Number::ratio(-1, 9));
        t(neg_two_thirds(), Number::ratio(-5, 23));
        t(neg_one_half(), Number::ratio(-2, 7));
        t(neg_one_fourth(), Number::ratio(-5, 12));
        t(zero(), Number::ratio(-3, 5));
        t(one_fourth(), Number::ratio(-7, 8));
        t(one_half(), Number::ratio(-4, 3));
        t(two_thirds(), Number::ratio(-13, 7));
        t(one(), Number::ratio(-5, 1));
        t(two(), Number::ratio(7, 3));
    }

    #[test]
    fn t_ppnp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 2, 3, -4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(-1, 13));
        t(neg_one(), Number::ratio(1, 9));
        t(neg_two_thirds(), Number::ratio(5, 23));
        t(neg_one_half(), Number::ratio(2, 7));
        t(neg_one_fourth(), Number::ratio(5, 12));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(7, 8));
        t(one_half(), Number::ratio(4, 3));
        t(two_thirds(), Number::ratio(13, 7));
        t(one(), Number::ratio(5, 1));
        t(two(), Number::ratio(-7, 3));
    }

    #[test]
    fn t_pnpp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 2, -3, 4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(7, 3));
        t(neg_one(), Number::ratio(-5, 1));
        t(neg_two_thirds(), Number::ratio(-13, 7));
        t(neg_one_half(), Number::ratio(-4, 3));
        t(neg_one_fourth(), Number::ratio(-7, 8));
        t(zero(), Number::ratio(-3, 5));
        t(one_fourth(), Number::ratio(-5, 12));
        t(one_half(), Number::ratio(-2, 7));
        t(two_thirds(), Number::ratio(-5, 23));
        t(one(), Number::ratio(-1, 9));
        t(two(), Number::ratio(1, 13));
    }

    #[test]
    fn t_nppp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, 3, 4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(-7, 3));
        t(neg_one(), Number::ratio(5, 1));
        t(neg_two_thirds(), Number::ratio(13, 7));
        t(neg_one_half(), Number::ratio(4, 3));
        t(neg_one_fourth(), Number::ratio(7, 8));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(5, 12));
        t(one_half(), Number::ratio(2, 7));
        t(two_thirds(), Number::ratio(5, 23));
        t(one(), Number::ratio(1, 9));
        t(two(), Number::ratio(-1, 13));
    }

    #[test]
    fn t_nppn() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, 3, 4, -5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(-7, 13));
        t(neg_one(), Number::ratio(-5, 9));
        t(neg_two_thirds(), Number::ratio(-13, 23));
        t(neg_one_half(), Number::ratio(-4, 7));
        t(neg_one_fourth(), Number::ratio(-7, 12));
        t(zero(), Number::ratio(-3, 5));
        t(one_fourth(), Number::ratio(-5, 8));
        t(one_half(), Number::ratio(-2, 3));
        t(two_thirds(), Number::ratio(-5, 7));
        t(one(), Number::ratio(-1, 1));
        t(two(), Number::ratio(-1, 3));
    }

    #[test]
    fn t_npnp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, 3, -4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(7, 13));
        t(neg_one(), Number::ratio(5, 9));
        t(neg_two_thirds(), Number::ratio(13, 23));
        t(neg_one_half(), Number::ratio(4, 7));
        t(neg_one_fourth(), Number::ratio(7, 12));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(5, 8));
        t(one_half(), Number::ratio(2, 3));
        t(two_thirds(), Number::ratio(5, 7));
        t(one(), Number::ratio(1, 1));
        t(two(), Number::ratio(1, 3));
    }

    #[test]
    fn t_nnpp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, -3, 4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(-1, 3));
        t(neg_one(), Number::ratio(-1, 1));
        t(neg_two_thirds(), Number::ratio(-5, 7));
        t(neg_one_half(), Number::ratio(-2, 3));
        t(neg_one_fourth(), Number::ratio(-5, 8));
        t(zero(), Number::ratio(-3, 5));
        t(one_fourth(), Number::ratio(-7, 12));
        t(one_half(), Number::ratio(-4, 7));
        t(two_thirds(), Number::ratio(-13, 23));
        t(one(), Number::ratio(-5, 9));
        t(two(), Number::ratio(-7, 13));
    }

    #[test]
    fn t_nnpn() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, -3, 4, -5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(-1, 13));
        t(neg_one(), Number::ratio(1, 9));
        t(neg_two_thirds(), Number::ratio(5, 23));
        t(neg_one_half(), Number::ratio(2, 7));
        t(neg_one_fourth(), Number::ratio(5, 12));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(7, 8));
        t(one_half(), Number::ratio(4, 3));
        t(two_thirds(), Number::ratio(13, 7));
        t(one(), Number::ratio(5, 1));
        t(two(), Number::ratio(-7, 3));
    }

    #[test]
    fn t_nnnp() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, -3, -4, 5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(1, 13));
        t(neg_one(), Number::ratio(-1, 9));
        t(neg_two_thirds(), Number::ratio(-5, 23));
        t(neg_one_half(), Number::ratio(-2, 7));
        t(neg_one_fourth(), Number::ratio(-5, 12));
        t(zero(), Number::ratio(-3, 5));
        t(one_fourth(), Number::ratio(-7, 8));
        t(one_half(), Number::ratio(-4, 3));
        t(two_thirds(), Number::ratio(-13, 7));
        t(one(), Number::ratio(-5, 1));
        t(two(), Number::ratio(7, 3));
    }

    #[test]
    fn t_nnnn() {
        fn h(x: Number) -> Number {
            Number::homographic(x, -2, -3, -4, -5)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(1, 3));
        t(neg_one(), Number::ratio(1, 1));
        t(neg_two_thirds(), Number::ratio(5, 7));
        t(neg_one_half(), Number::ratio(2, 3));
        t(neg_one_fourth(), Number::ratio(5, 8));
        t(zero(), Number::ratio(3, 5));
        t(one_fourth(), Number::ratio(7, 12));
        t(one_half(), Number::ratio(4, 7));
        t(two_thirds(), Number::ratio(13, 23));
        t(one(), Number::ratio(5, 9));
        t(two(), Number::ratio(7, 13));
    }

    #[test]
    fn t_6342() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 6, 3, 4, 2)
        }
        fn t(n1: Number, n2: Number) {
            assert_eq(h(n1), n2)
        }
        t(neg_two(), Number::ratio(3, 2));
        t(neg_one(), Number::ratio(3, 2));
        t(neg_two_thirds(), Number::ratio(3, 2));
        // t(neg_one_half(), Number::ratio(3, 2));
        t(neg_one_fourth(), Number::ratio(3, 2));
        t(zero(), Number::ratio(3, 2));
        t(one_fourth(), Number::ratio(3, 2));
        t(one_half(), Number::ratio(3, 2));
        t(two_thirds(), Number::ratio(3, 2));
        t(one(), Number::ratio(3, 2));
        t(two(), Number::ratio(3, 2));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn p_6342() {
        fn h(x: Number) -> Number {
            Number::homographic(x, 6, 3, 4, 2)
        }
        h(neg_one_half());
    }

    fn consume(x: Number) {
        let (_, mut c) = x.unwrap_other();
        loop {
            match c.egest() {
                None => {
                    break;
                }
                _ => {}
            }
        }
    }

    #[test]
    fn unity_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            1,
            0,
            0,
            1,
        ));
    }

    #[test]
    fn unity_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            1,
            0,
            0,
            1,
        ));
    }

    #[test]
    fn mul_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            2,
            0,
            0,
            1,
        ));
    }

    #[test]
    fn mul_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            2,
            0,
            0,
            1,
        ));
    }

    #[test]
    fn div_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            1,
            0,
            0,
            2,
        ));
    }

    #[test]
    fn div_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            1,
            0,
            0,
            2,
        ));
    }

    #[test]
    fn rec_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            0,
            1,
            1,
            0,
        ));
    }

    #[test]
    fn rec_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            0,
            1,
            1,
            0,
        ));
    }

    #[test]
    #[ignore] // FIXME: issue #2
    fn add_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            1,
            1,
            0,
            1,
        ));
    }

    #[test]
    fn add_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            1,
            1,
            0,
            1,
        ));
    }

    #[test]
    #[ignore] // FIXME: issue #2
    fn rec_add_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            0,
            1,
            1,
            1,
        ));
    }

    #[test]
    fn rec_add_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            0,
            1,
            1,
            1,
        ));
    }

    #[test]
    fn incr_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            1,
            1,
            1,
            0,
        ));
    }

    #[test]
    #[ignore] // FIXME: issue #2
    fn incr_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            1,
            1,
            1,
            0,
        ));
    }

    #[test]
    fn rec_incr_does_not_overflow1() {
        consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            1,
            0,
            1,
            1,
        ));
    }

    #[test]
    #[ignore] // FIXME: issue #2
    fn rec_incr_does_not_overflow2() {
        consume(Number::homographic(
            Number::ratio(1, isize::max_value()),
            1,
            0,
            1,
            1,
        ));
    }
}

pub struct Homographic {
    x: Clog,
    nx: isize,
    n: isize,
    dx: isize,
    d: isize,
}

pub fn new(
    x: Number,
    mut nx: isize,
    mut n: isize,
    mut dx: isize,
    mut d: isize,
) -> (
    Option<protocol::Special>,
    Option<protocol::Primer>,
    Option<Ratio>,
    Option<Homographic>,
) {
    fn as_ratio(
        n: isize,
        d: isize,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
    ) {
        let (special, primer, ratio) = ratio::new_i(n, d);
        (special, primer, ratio, None)
    }

    if nx == 0 && dx == 0 {
        return as_ratio(n, d);
    }

    if let Number::Special(special) = x {
        return match special {
            protocol::Special::NegOne => {
                as_ratio(n.checked_sub(nx).unwrap(), d.checked_sub(dx).unwrap())
            }
            protocol::Special::Zero => as_ratio(n, d),
            protocol::Special::PosOne => {
                as_ratio(n.checked_add(nx).unwrap(), d.checked_add(dx).unwrap())
            }
        };
    }

    let (x_primer, x_clog) = x.unwrap_other();
    match x_primer {
        Some(protocol::Primer::Turn) => {
            swap(&mut nx, &mut n);
            swap(&mut dx, &mut d);
        }
        Some(protocol::Primer::Reflect) => {
            nx = -nx;
            dx = -dx;
        }
        Some(protocol::Primer::Ground) => {
            nx = -nx;
            dx = -dx;
            swap(&mut nx, &mut n);
            swap(&mut dx, &mut d);
        }
        None => {}
    }
    Homographic::new(x_clog, nx, n, dx, d)
}

impl Homographic {
    fn new(
        x: Clog,
        nx: isize,
        n: isize,
        dx: isize,
        d: isize,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
    ) {
        (Homographic { x, nx, n, dx, d }).prime()
    }

    fn prime_ingest(
        &mut self,
    ) -> Option<(
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
    )> {
        match self.x.egest() {
            None => {
                let (num, den) = self.value_at_one_half();
                Some(ratio::new_i(num, den))
            }
            Some(protocol::Reduction::Amplify) => {
                self.ingest_amplify();
                None
            }
            Some(protocol::Reduction::Uncover) => {
                self.ingest_uncover();
                None
            }
        }
    }

    fn primer_egest(&mut self) -> Result<Option<protocol::Primer>, isize> {
        let (nmin, dmin, nmax, dmax) = self.image_extremes();
        if nmin == 0 && dmin == 0 {
            Err(0)
        } else if support::are_same(nmin, dmin, nmax, dmax) && support::is_special(nmin, dmin) {
            Err(0)
        } else if !support::greater_than_minus_one(nmax, dmax) {
            // max <= -1
            Ok(Some(self.ground()))
        } else if !support::less_than_one(nmin, dmin) {
            // min >= 1
            Ok(Some(self.turn()))
        } else if !support::less_than_minus_one(nmin, dmin)
            && !support::greater_than_zero(nmax, dmax)
        {
            // min >= -1 && max <= 0
            Ok(Some(self.reflect()))
        } else if !support::less_than_zero(nmin, dmin) && !support::greater_than_one(nmax, dmax) {
            // min >= 0 && max <= 1
            Ok(None)
        } else {
            Err(0)
        }
    }

    fn ground(&mut self) -> protocol::Primer {
        self.dx = -self.dx;
        self.d = -self.d;
        swap(&mut self.nx, &mut self.dx);
        swap(&mut self.n, &mut self.d);
        protocol::Primer::Ground
    }

    fn reflect(&mut self) -> protocol::Primer {
        self.nx = -self.nx;
        self.n = -self.n;
        protocol::Primer::Reflect
    }

    fn turn(&mut self) -> protocol::Primer {
        swap(&mut self.nx, &mut self.dx);
        swap(&mut self.n, &mut self.d);
        protocol::Primer::Turn
    }

    fn image_extremes(&self) -> (isize, isize, isize, isize) {
        let (n0, d0) = self.value_at_zero();
        if n0 == 0 && d0 == 0 {
            return (0, 0, 0, 0);
        }
        let (n1, d1) = self.value_at_one();
        if n1 == 0 && d1 == 0 {
            return (0, 0, 0, 0);
        }
        support::updated_range(n0, d0, n0, d0, n1, d1)
    }

    fn are_singularities_outside_domain(&self) -> bool {
        self.is_zero_outside_domain() && self.is_pole_outside_domain()
    }

    fn is_domain_amenable(mx: isize, m: isize) -> bool {
        let s = m.signum();
        s == 0
            || s == mx.checked_add(m).unwrap().signum()
            || mx.checked_add(m).unwrap().signum() == 0
    }

    fn is_pole_outside_domain(&self) -> bool {
        Homographic::is_domain_amenable(self.dx, self.d)
    }

    fn is_zero_outside_domain(&self) -> bool {
        Homographic::is_domain_amenable(self.nx, self.n)
    }

    fn value_at_one(&self) -> (isize, isize) {
        (
            self.n.checked_add(self.nx).unwrap(),
            self.d.checked_add(self.dx).unwrap(),
        )
    }

    fn value_at_zero(&self) -> (isize, isize) {
        (self.n, self.d)
    }

    fn value_at_one_half(&self) -> (isize, isize) {
        if self.nx % 2 != 0 || self.dx % 2 != 0 {
            (
                self.nx.checked_add(self.n.checked_mul(2).unwrap()).unwrap(),
                self.dx.checked_add(self.d.checked_mul(2).unwrap()).unwrap(),
            )
        } else {
            (
                self.n.checked_add(self.nx / 2).unwrap(),
                self.d.checked_add(self.dx / 2).unwrap(),
            )
        }
    }

    fn ingest_amplify(&mut self) {
        if self.nx % 2 != 0 || self.dx % 2 != 0 {
            self.n = self.n.checked_mul(2).unwrap();
            self.d = self.d.checked_mul(2).unwrap();
        } else {
            self.nx /= 2;
            self.dx /= 2;
        }
    }

    fn ingest_uncover(&mut self) {
        self.nx = self.nx.checked_add(self.n).unwrap();
        self.dx = self.dx.checked_add(self.d).unwrap();
        swap(&mut self.nx, &mut self.n);
        swap(&mut self.dx, &mut self.d);
    }

    fn reduction_ingest(&mut self) -> Option<Ratio> {
        match self.x.egest() {
            None => {
                let (num, den) = self.value_at_one_half();
                match ratio::new_i(num, den) {
                    (None, None, Some(ratio)) => Some(ratio),
                    _ => panic!("logic error"),
                }
            }
            Some(protocol::Reduction::Amplify) => {
                self.ingest_amplify();
                None
            }
            Some(protocol::Reduction::Uncover) => {
                self.ingest_uncover();
                None
            }
        }
    }

    fn prime(
        mut self,
    ) -> (
        Option<protocol::Special>,
        Option<protocol::Primer>,
        Option<Ratio>,
        Option<Homographic>,
    ) {
        // FIXME: debug
        println!("{} {} {} {}", self.nx, self.n, self.dx, self.d);
        if self.are_singularities_outside_domain() {
            if let Ok(primer) = self.primer_egest() {
                return (None, primer, None, Some(self));
            }
        }
        match self.prime_ingest() {
            Some((special, primer, ratio)) => (special, primer, ratio, None),
            // FIXME: recursion
            None => self.prime(),
        }
    }

    fn reduction_egest(&mut self) -> Result<Option<protocol::Reduction>, isize> {
        let (nmin, dmin, nmax, dmax) = self.image_extremes();
        if support::less_than_zero(nmin, dmin) {
            panic!("logic error");
        } else if support::greater_than_one(nmax, dmax) {
            panic!("logic error");
        } else if support::equal_to_one_half(nmin, dmin) && support::equal_to_one_half(nmax, dmax) {
            Ok(None)
        } else if !support::greater_than_one_half(nmax, dmax) {
            Ok(Some(self.amplify()))
        } else if !support::less_than_one_half(nmin, dmin) {
            Ok(Some(self.uncover()))
        } else {
            Err(0)
        }
    }

    fn uncover(&mut self) -> protocol::Reduction {
        self.dx = self.dx.checked_sub(self.nx).unwrap();
        self.d = self.d.checked_sub(self.n).unwrap();
        swap(&mut self.nx, &mut self.dx);
        swap(&mut self.n, &mut self.d);
        protocol::Reduction::Uncover
    }

    fn amplify(&mut self) -> protocol::Reduction {
        if self.dx % 2 != 0 || self.d % 2 != 0 {
            self.nx = self.nx.checked_mul(2).unwrap();
            self.n = self.n.checked_mul(2).unwrap();
        } else {
            self.dx /= 2;
            self.d /= 2;
        }
        protocol::Reduction::Amplify
    }
}

impl Strategy for Homographic {
    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        // FIXME: debug
        println!("{} {} {} {}", self.nx, self.n, self.dx, self.d);
        if self.are_singularities_outside_domain() {
            if let Ok(reduction) = self.reduction_egest() {
                return Ok(reduction);
            }
        }
        match self.reduction_ingest() {
            Some(ratio) => Err(Box::new(ratio)),
            // FIXME: recursion
            None => self.egest(),
        }
    }
}
