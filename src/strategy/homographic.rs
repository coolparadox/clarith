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
use std::mem::swap;
use crate::protocol;
use crate::strategy::Strategy;
use crate::strategy::ratio;
use crate::strategy::ratio::Ratio;
use crate::Clog;
use crate::Number;

#[cfg(test)]
mod tests {

    use super::*;

    // known well defined values from ratio unit tests

    fn neg_inf() -> Number {
        Number::ratio(-1, 0)
    }

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

    fn inf() -> Number {
        Number::ratio(1, 0)
    }

    // Support

    fn assert_eq(n1: Number, n2: Number) {
        assert_eq!(Number::compare(n1, n2), Ordering::Equal);
    }

     // Tests

    #[test]
    fn t_1111() {
        fn h(x: Number) -> Number { Number::homographic(x, 1, 1, 1, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), one());
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
        t(inf(), one());
    }

    #[test]
    fn t_1110() {
        fn h(x: Number) -> Number { Number::homographic(x, 1, 1, 1, 0) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), one());
        t(neg_two(), one_half());
        t(neg_one(), zero());
        t(neg_two_thirds(), neg_one_half());
        t(neg_one_half(), neg_one());
        t(neg_one_fourth(), Number::ratio(-3, 1));
        t(zero(), inf());
        t(one_fourth(), Number::ratio(5, 1));
        t(one_half(), Number::ratio(3, 1));
        t(two_thirds(), Number::ratio(5, 2));
        t(one(), two());
        t(two(), Number::ratio(3, 2));
        t(inf(), one());
    }

    #[test]
    fn t_1101() {
        fn h(x: Number) -> Number { Number::homographic(x, 1, 1, 0, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), neg_inf());
        t(neg_two(), neg_one());
        t(neg_one(), zero());
        t(neg_two_thirds(), Number::ratio(1, 3));
        t(neg_one_half(), one_half());
        t(neg_one_fourth(), Number::ratio(-3, 4));
        t(zero(), one());
        t(one_fourth(), Number::ratio(5, 4));
        t(one_half(), Number::ratio(3, 2));
        t(two_thirds(), Number::ratio(5, 3));
        t(one(), two());
        t(two(), Number::ratio(3, 1));
        t(inf(), inf());
    }

    #[test]
    fn t_1011() {
        fn h(x: Number) -> Number { Number::homographic(x, 1, 0, 1, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), one());
        t(neg_two(), two());
        t(neg_one(), neg_inf());
        t(neg_two_thirds(), neg_two());
        t(neg_one_half(), neg_one());
        t(neg_one_fourth(), Number::ratio(-1, 3));
        t(zero(), zero());
        t(one_fourth(), Number::ratio(1, 5));
        t(one_half(), Number::ratio(1, 3));
        t(two_thirds(), Number::ratio(2, 5));
        t(one(), one_half());
        t(two(), two_thirds());
        t(inf(), one());
    }

    #[test]
    fn t_0111() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 1, 1, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), zero());
        t(neg_two(), neg_one());
        t(neg_one(), inf());
        t(neg_two_thirds(), Number::ratio(3, 1));
        t(neg_one_half(), two());
        t(neg_one_fourth(), Number::ratio(4, 3));
        t(zero(), one());
        t(one_fourth(), Number::ratio(4, 5));
        t(one_half(), Number::ratio(2, 3));
        t(two_thirds(), Number::ratio(3, 5));
        t(one(), one_half());
        t(two(), Number::ratio(1, 3));
        t(inf(), zero());
    }

    #[test]
    fn t_0110() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 1, 1, 0) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), zero());
        t(neg_two(), neg_one_half());
        t(neg_one(), neg_one());
        t(neg_two_thirds(), Number::ratio(-3, 2));
        t(neg_one_half(), neg_two());
        t(neg_one_fourth(), Number::ratio(-4, 1));
        t(zero(), inf());
        t(one_fourth(), Number::ratio(4, 1));
        t(one_half(), two());
        t(two_thirds(), Number::ratio(3, 2));
        t(one(), one());
        t(two(), one_half());
        t(inf(), zero());
    }

    #[test]
    fn t_0101() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 1, 0, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), one());
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
        t(inf(), one());
    }

    #[test]
    fn t_0011() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 1, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), zero());
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
        t(inf(), zero());
    }

    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn t_0011_panic() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 1, 1) }
        h(neg_one());
    }

    #[test]
    fn t_0010() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 1, 0) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), zero());
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
        t(inf(), zero());
    }

    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn t_0010_panic() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 1, 0) }
        h(zero());
    }

    #[test]
    fn t_0001() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 0, 1) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), zero());
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
        t(inf(), zero());
    }

    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn t_0000() {
        Number::homographic(one(), 0, 0, 0, 0);
    }

    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn t_0021_panic() {
        fn h(x: Number) -> Number { Number::homographic(x, 0, 0, 2, 1) }
        h(neg_one_half());
    }

    #[test]
    fn t_pppp() {
        fn h(x: Number) -> Number { Number::homographic(x, 2, 3, 4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(1, 2));
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
        t(inf(), Number::ratio(1, 2));
    }

    #[test]
    fn t_pppn() {
        fn h(x: Number) -> Number { Number::homographic(x, 2, 3, 4, -5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(1, 2));
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
        t(inf(), Number::ratio(1, 2));
    }

    #[test]
    fn t_ppnp() {
        fn h(x: Number) -> Number { Number::homographic(x, 2, 3, -4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_pnpp() {
        fn h(x: Number) -> Number { Number::homographic(x, 2, -3, 4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(1, 2));
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
        t(inf(), Number::ratio(1, 2));
    }

    #[test]
    fn t_nppp() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, 3, 4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_nppn() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, 3, 4, -5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_npnp() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, 3, -4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_nnpp() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, -3, 4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_nnpn() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, -3, 4, -5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(-1, 2));
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
        t(inf(), Number::ratio(-1, 2));
    }

    #[test]
    fn t_nnnp() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, -3, -4, 5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(1, 2));
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
        t(inf(), Number::ratio(1, 2));
    }

    #[test]
    fn t_nnnn() {
        fn h(x: Number) -> Number { Number::homographic(x, -2, -3, -4, -5) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(1, 2));
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
        t(inf(), Number::ratio(1, 2));
    }

    #[test]
    fn t_6342() {
        fn h(x: Number) -> Number { Number::homographic(x, 6, 3, 4, 2) }
        fn t(n1: Number, n2: Number) { assert_eq(h(n1), n2) }
        t(neg_inf(), Number::ratio(3, 2));
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
        t(inf(), Number::ratio(3, 2));
    }

    #[test]
    #[should_panic(expected = "undefined ratio")]
    fn t_6342_panic() {
        fn h(x: Number) -> Number { Number::homographic(x, 6, 3, 4, 2) }
        h(neg_one_half());
    }

    // FIXME: overflow tests

}

pub struct Homographic {
    x: Clog,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
}

pub fn new(x: Number, mut a: isize, mut b: isize, mut c: isize, mut d: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>) {

    fn as_ratio(n: isize, d: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>) {
        let (special, primer, ratio) = ratio::new_i(n, d);
        (special, primer, ratio, None)
    }

    if a == 0 && c == 0 {
        return as_ratio(b, d);
    }

    if let Number::Special(special) = x {
        return match special {
            protocol::Special::NegInf => {
                if a == 0 {
                    (Some(protocol::Special::Zero), None, None, None)
                }
                else if c == 0 {
                    if a > 0 {
                        (Some(protocol::Special::NegInf), None, None, None)
                    }
                    else {
                        (Some(protocol::Special::PosInf), None, None, None)
                    }
                }
                else {
                    as_ratio(a, c)
                }
            },
            protocol::Special::NegOne => {
                as_ratio(b.checked_sub(a).unwrap(), d.checked_sub(c).unwrap())
            },
            protocol::Special::Zero => {
                as_ratio(b, d)
            },
            protocol::Special::PosOne => {
                as_ratio(b.checked_add(a).unwrap(), d.checked_add(c).unwrap())
            },
            protocol::Special::PosInf => {
                if a == 0 {
                    (Some(protocol::Special::Zero), None, None, None)
                }
                else if c == 0 {
                    if a > 0 {
                        (Some(protocol::Special::PosInf), None, None, None)
                    }
                    else {
                        (Some(protocol::Special::NegInf), None, None, None)
                    }
                }
                else {
                    as_ratio(a, c)
                }
            },
        }
    }

    if b == 0 && d == 0 {
        return as_ratio(a, c);
    }

    let (primer_opt, clog) = x.as_other();
    if let Some(primer) = primer_opt {
        match primer {
            protocol::Primer::Turn => {
                swap(&mut a, &mut b);
                swap(&mut c, &mut d);
            },
            protocol::Primer::Reflect => {
                a = -a;
                c = -c;
            },
            protocol::Primer::Ground => {
                a = -a;
                c = -c;
                swap(&mut a, &mut b);
                swap(&mut c, &mut d);
            },
        }
    }

    fn compare(n1: isize, d1: isize, n2: isize, d2: isize) -> Ordering {
        Number::compare(Number::ratio(n1, d1), Number::ratio(n2, d2))
    }

    fn lt(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        compare(n1, d1, n2, d2) == Ordering::Less
    }

    fn gt(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        compare(n1, d1, n2, d2) == Ordering::Greater
    }

    fn inside_domain(n: isize, d:isize) -> bool {
        gt(n, d, 0, 1) && lt(n, d, 1, 1)
    }

    fn sort(n1: isize, d1: isize, n2: isize, d2: isize) -> (isize, isize, isize, isize) {
        if lt(n1, d1, n2, d2) { (n1, d1, n2, d2) } else { (n2, d2, n1, n1) }
    }

    fn is_primeable(a: isize, b: isize, c: isize, d: isize) -> Result<Option<protocol::Primer>, i32>
    {
        // zero
        if inside_domain(-b, a) {
            return Err(0);
        }
        // pole
        if inside_domain(-d, c) {
            return Err(0);
        }
        // image extremes
        let (nmin, dmin, nmax, dmax) = sort(b, d, a.checked_add(b).unwrap(), c.checked_add(d).unwrap());
        // try to classify image range
        // FIXME: optimize comparisons
        if lt(nmax, dmax, -1, 1) {
            Ok(Some(protocol::Primer::Ground))
        }
        else if gt(nmin, dmin, -1, 1) && lt(nmax, dmax, 0, 1) {
            Ok(Some(protocol::Primer::Reflect))
        }
        else if gt(nmin, dmin, 0, 1) && lt(nmax, dmax, 1, 1) {
            Ok(None)
        }
        else if gt(nmin, dmin, 1, 1) {
            Ok(Some(protocol::Primer::Turn))
        }
        else {
            Err(0)
        }
    }

    (Some(protocol::Special::Zero), None, None, None)
}

impl Homographic {

    fn compare(n1: isize, d1: isize, n2: isize, d2: isize) -> Ordering {
        Number::compare(Number::ratio(n1, d1), Number::ratio(n2, d2))
    }

    fn lt(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        Homographic::compare(n1, d1, n2, d2) == Ordering::Less
    }

    fn gt(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        Homographic::compare(n1, d1, n2, d2) == Ordering::Greater
    }

    fn le(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        Homographic::compare(n1, d1, n2, d2) != Ordering::Greater
    }

    fn ge(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
        Homographic::compare(n1, d1, n2, d2) != Ordering::Less
    }

    fn inside_domain(n: isize, d:isize) -> bool {
        Homographic::gt(n, d, 0, 1) && Homographic::lt(n, d, 1, 1)
    }

    fn sort(n1: isize, d1: isize, n2: isize, d2: isize) -> (isize, isize, isize, isize) {
        if Homographic::lt(n1, d1, n2, d2) { (n1, d1, n2, d2) } else { (n2, d2, n1, n1) }
    }

    fn ingest(&mut self) -> Option<Ratio> {
        match self.x.egest() {
            None => {
                let (num, den) = if self.a % 2 != 0 || self.c % 2 != 0 {
                    (self.a.checked_add(self.b.checked_mul(2).unwrap()).unwrap(), self.c.checked_add(self.d.checked_mul(2).unwrap()).unwrap())
                }
                else {
                    (self.b.checked_add(self.a / 2).unwrap(), self.d.checked_add(self.c / 2).unwrap())
                };
                match ratio::new_i(num, den) {
                    (None, None, Some(ratio)) => Some(ratio),
                    _ => panic!("logic error"),
                }
            },
            Some(protocol::Reduction::Amplify) => {
                if self.a % 2 != 0 || self.c % 2 != 0 {
                    self.b = self.b.checked_mul(2).unwrap();
                    self.d = self.d.checked_mul(2).unwrap();
                }
                else {
                    self.a /= 2;
                    self.c /= 2;
                }
                None
            },
            Some(protocol::Reduction::Uncover) => {
                self.a = self.a.checked_add(self.b).unwrap();
                self.c = self.c.checked_add(self.d).unwrap();
                swap(&mut self.a, &mut self.b);
                swap(&mut self.c, &mut self.d);
                None
            },
        }
    }

    fn prime(&mut self) -> Result<Option<protocol::Primer>, Box<dyn Strategy>> {
        // zero, pole
        if Homographic::inside_domain(-self.b, self.a) || Homographic::inside_domain(-self.d, self.c) {
            match self.ingest() {
                Some(ratio) => Err(Box::new(ratio)),
                None => self.prime(),
            }
        }
        else {
            // image extremes
            let (nmin, dmin, nmax, dmax) = Homographic::sort(self.b, self.d, self.a.checked_add(self.b).unwrap(), self.c.checked_add(self.d).unwrap());
            // classify output
            // FIXME: optimize comparisons?
            if Homographic::lt(nmax, dmax, -1, 1) {
                Ok(Some(protocol::Primer::Ground))
            }
            else if Homographic::gt(nmin, dmin, -1, 1) && Homographic::lt(nmax, dmax, 0, 1) {
                Ok(Some(protocol::Primer::Reflect))
            }
            else if Homographic::gt(nmin, dmin, 0, 1) && Homographic::lt(nmax, dmax, 1, 1) {
                Ok(None)
            }
            else if Homographic::gt(nmin, dmin, 1, 1) {
                Ok(Some(protocol::Primer::Turn))
            }
            else {
                match self.ingest() {
                    Some(ratio) => Err(Box::new(ratio)),
                    None => self.prime(),
                }
            }
        }
    }

}

impl Strategy for Homographic {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        // zero, pole
        if Homographic::inside_domain(-self.b, self.a) || Homographic::inside_domain(-self.d, self.c) {
            match self.ingest() {
                Some(ratio) => Err(Box::new(ratio)),
                None => self.egest(),
            }
        }
        else {
            // image extremes
            let (nmin, dmin, nmax, dmax) = Homographic::sort(self.b, self.d, self.a.checked_add(self.b).unwrap(), self.c.checked_add(self.d).unwrap());
            // FIXME: remove these sanity checks?
            // FIXME: optimize comparison?
            if Homographic::le(nmin, dmin, 0, 1) {
                panic!("logic error");
            }
            // FIXME: optimize comparison?
            if Homographic::ge(nmax, dmax, 1, 1) {
                panic!("logic error");
            }
            // classify output
            // FIXME: optimize comparison?
            if Homographic::lt(nmax, dmax, 1, 2) {
                if self.c % 2 != 0 || self.d % 2 != 0 {
                    self.a = self.a.checked_mul(2).unwrap();
                    self.b = self.b.checked_mul(2).unwrap();
                }
                else {
                    self.c /= 2;
                    self.d /= 2;
                }
                Ok(Some(protocol::Reduction::Amplify))
            }
            // FIXME: optimize comparison?
            else if Homographic::gt(nmin, dmin, 1, 2) {
                self.c = self.c.checked_sub(self.a).unwrap();
                self.d = self.d.checked_sub(self.b).unwrap();
                swap(&mut self.a, &mut self.c);
                swap(&mut self.b, &mut self.d);
                Ok(Some(protocol::Reduction::Uncover))
            }
            else {
                match self.ingest() {
                    Some(ratio) => Err(Box::new(ratio)),
                    None => self.egest(),
                }
            }
        }
    }

}
