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

use crate::Clog;
use crate::Number;
use crate::protocol;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {

    use crate::ratio;
    use super::*;

    // known well defined values from ratio unit tests

    fn neg_inf() -> Number {
        ratio(-1, 0)
    }

    fn neg_two() -> Number {
        ratio(-2, 1)
    }

    fn neg_one() -> Number {
        ratio(-1, 1)
    }

    fn neg_two_thirds() -> Number {
        ratio(-2, 3)
    }

    fn neg_one_half() -> Number {
        ratio(-1, 2)
    }

    fn neg_one_fourth() -> Number {
        ratio(-1, 4)
    }

    fn zero() -> Number {
        ratio(0, 1)
    }

    fn one_fourth() -> Number {
        ratio(1, 4)
    }

    fn one_half() -> Number {
        ratio(1, 2)
    }

    fn two_thirds() -> Number {
        ratio(2, 3)
    }

    fn one() -> Number {
        ratio(1, 1)
    }

    fn two() -> Number {
        ratio(2, 1)
    }

    fn inf() -> Number {
        ratio(1, 0)
    }

    #[test]
    fn compare_negative_infinity() {
        assert_eq!(compare(neg_inf(), neg_inf()), Ordering::Equal);
        assert_eq!(compare(neg_inf(), neg_two()), Ordering::Less);
        assert_eq!(compare(neg_inf(), neg_one()), Ordering::Less);
        assert_eq!(compare(neg_inf(), neg_two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_inf(), neg_one_half()), Ordering::Less);
        assert_eq!(compare(neg_inf(), neg_one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_inf(), zero()), Ordering::Less);
        assert_eq!(compare(neg_inf(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_inf(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_inf(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_inf(), one()), Ordering::Less);
        assert_eq!(compare(neg_inf(), two()), Ordering::Less);
        assert_eq!(compare(neg_inf(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_negative_two() {
        assert_eq!(compare(neg_two(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(neg_two(), neg_two()), Ordering::Equal);
        assert_eq!(compare(neg_two(), neg_one()), Ordering::Less);
        assert_eq!(compare(neg_two(), neg_two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_two(), neg_one_half()), Ordering::Less);
        assert_eq!(compare(neg_two(), neg_one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_two(), zero()), Ordering::Less);
        assert_eq!(compare(neg_two(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_two(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_two(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_two(), one()), Ordering::Less);
        assert_eq!(compare(neg_two(), two()), Ordering::Less);
        assert_eq!(compare(neg_two(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_negative_one() {
        assert_eq!(compare(neg_one(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(neg_one(), neg_two()), Ordering::Greater);
        assert_eq!(compare(neg_one(), neg_one()), Ordering::Equal);
        assert_eq!(compare(neg_one(), neg_two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_one(), neg_one_half()), Ordering::Less);
        assert_eq!(compare(neg_one(), neg_one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_one(), zero()), Ordering::Less);
        assert_eq!(compare(neg_one(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_one(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_one(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_one(), one()), Ordering::Less);
        assert_eq!(compare(neg_one(), two()), Ordering::Less);
        assert_eq!(compare(neg_one(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_negative_two_thirds() {
        assert_eq!(compare(neg_two_thirds(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(neg_two_thirds(), neg_two()), Ordering::Greater);
        assert_eq!(compare(neg_two_thirds(), neg_one()), Ordering::Greater);
        assert_eq!(compare(neg_two_thirds(), neg_two_thirds()), Ordering::Equal);
        assert_eq!(compare(neg_two_thirds(), neg_one_half()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), neg_one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), zero()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), one()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), two()), Ordering::Less);
        assert_eq!(compare(neg_two_thirds(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_negative_one_half() {
        assert_eq!(compare(neg_one_half(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(neg_one_half(), neg_two()), Ordering::Greater);
        assert_eq!(compare(neg_one_half(), neg_one()), Ordering::Greater);
        assert_eq!(compare(neg_one_half(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(neg_one_half(), neg_one_half()), Ordering::Equal);
        assert_eq!(compare(neg_one_half(), neg_one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), zero()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), one()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), two()), Ordering::Less);
        assert_eq!(compare(neg_one_half(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_negative_one_fourth() {
        assert_eq!(compare(neg_one_fourth(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(neg_one_fourth(), neg_two()), Ordering::Greater);
        assert_eq!(compare(neg_one_fourth(), neg_one()), Ordering::Greater);
        assert_eq!(compare(neg_one_fourth(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(neg_one_fourth(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(neg_one_fourth(), neg_one_fourth()), Ordering::Equal);
        assert_eq!(compare(neg_one_fourth(), zero()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), one_fourth()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), one_half()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), two_thirds()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), one()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), two()), Ordering::Less);
        assert_eq!(compare(neg_one_fourth(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_zero() {
        assert_eq!(compare(zero(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(zero(), neg_two()), Ordering::Greater);
        assert_eq!(compare(zero(), neg_one()), Ordering::Greater);
        assert_eq!(compare(zero(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(zero(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(zero(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(zero(), zero()), Ordering::Equal);
        assert_eq!(compare(zero(), one_fourth()), Ordering::Less);
        assert_eq!(compare(zero(), one_half()), Ordering::Less);
        assert_eq!(compare(zero(), two_thirds()), Ordering::Less);
        assert_eq!(compare(zero(), one()), Ordering::Less);
        assert_eq!(compare(zero(), two()), Ordering::Less);
        assert_eq!(compare(zero(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_one_fourth() {
        assert_eq!(compare(one_fourth(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), neg_two()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), neg_one()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), zero()), Ordering::Greater);
        assert_eq!(compare(one_fourth(), one_fourth()), Ordering::Equal);
        assert_eq!(compare(one_fourth(), one_half()), Ordering::Less);
        assert_eq!(compare(one_fourth(), two_thirds()), Ordering::Less);
        assert_eq!(compare(one_fourth(), one()), Ordering::Less);
        assert_eq!(compare(one_fourth(), two()), Ordering::Less);
        assert_eq!(compare(one_fourth(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_one_half() {
        assert_eq!(compare(one_half(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(one_half(), neg_two()), Ordering::Greater);
        assert_eq!(compare(one_half(), neg_one()), Ordering::Greater);
        assert_eq!(compare(one_half(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(one_half(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(one_half(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(one_half(), zero()), Ordering::Greater);
        assert_eq!(compare(one_half(), one_fourth()), Ordering::Greater);
        assert_eq!(compare(one_half(), one_half()), Ordering::Equal);
        assert_eq!(compare(one_half(), two_thirds()), Ordering::Less);
        assert_eq!(compare(one_half(), one()), Ordering::Less);
        assert_eq!(compare(one_half(), two()), Ordering::Less);
        assert_eq!(compare(one_half(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_two_thirds() {
        assert_eq!(compare(two_thirds(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), neg_two()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), neg_one()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), zero()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), one_fourth()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), one_half()), Ordering::Greater);
        assert_eq!(compare(two_thirds(), two_thirds()), Ordering::Equal);
        assert_eq!(compare(two_thirds(), one()), Ordering::Less);
        assert_eq!(compare(two_thirds(), two()), Ordering::Less);
        assert_eq!(compare(two_thirds(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_one() {
        assert_eq!(compare(one(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(one(), neg_two()), Ordering::Greater);
        assert_eq!(compare(one(), neg_one()), Ordering::Greater);
        assert_eq!(compare(one(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(one(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(one(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(one(), zero()), Ordering::Greater);
        assert_eq!(compare(one(), one_fourth()), Ordering::Greater);
        assert_eq!(compare(one(), one_half()), Ordering::Greater);
        assert_eq!(compare(one(), two_thirds()), Ordering::Greater);
        assert_eq!(compare(one(), one()), Ordering::Equal);
        assert_eq!(compare(one(), two()), Ordering::Less);
        assert_eq!(compare(one(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_two() {
        assert_eq!(compare(two(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(two(), neg_two()), Ordering::Greater);
        assert_eq!(compare(two(), neg_one()), Ordering::Greater);
        assert_eq!(compare(two(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(two(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(two(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(two(), zero()), Ordering::Greater);
        assert_eq!(compare(two(), one_fourth()), Ordering::Greater);
        assert_eq!(compare(two(), one_half()), Ordering::Greater);
        assert_eq!(compare(two(), two_thirds()), Ordering::Greater);
        assert_eq!(compare(two(), one()), Ordering::Greater);
        assert_eq!(compare(two(), two()), Ordering::Equal);
        assert_eq!(compare(two(), inf()), Ordering::Less);
    }

    #[test]
    fn compare_infinity() {
        assert_eq!(compare(inf(), neg_inf()), Ordering::Greater);
        assert_eq!(compare(inf(), neg_two()), Ordering::Greater);
        assert_eq!(compare(inf(), neg_one()), Ordering::Greater);
        assert_eq!(compare(inf(), neg_two_thirds()), Ordering::Greater);
        assert_eq!(compare(inf(), neg_one_half()), Ordering::Greater);
        assert_eq!(compare(inf(), neg_one_fourth()), Ordering::Greater);
        assert_eq!(compare(inf(), zero()), Ordering::Greater);
        assert_eq!(compare(inf(), one_fourth()), Ordering::Greater);
        assert_eq!(compare(inf(), one_half()), Ordering::Greater);
        assert_eq!(compare(inf(), two_thirds()), Ordering::Greater);
        assert_eq!(compare(inf(), one()), Ordering::Greater);
        assert_eq!(compare(inf(), two()), Ordering::Greater);
        assert_eq!(compare(inf(), inf()), Ordering::Equal);
    }

}

pub fn compare(n1: Number, n2: Number) -> Ordering {
    match n1 {
        Number::Special(s1) => match n2 {
            Number::Special(s2) => compare_specials(s1, s2),
            Number::Other(p2, _) => compare_hybrid(s1, p2),
        },
        Number::Other(p1, c1) => match n2 {
            Number::Special(s2) => compare_hybrid(s2, p1).reverse(),
            Number::Other(p2, c2) => compare_others(p1, c1, p2, c2),
        },
    }
}

fn compare_specials(s1: protocol::Special, s2: protocol::Special) -> Ordering {
    if s1 == s2 {
        return Ordering::Equal;
    }
    if s1 == protocol::Special::NegInf {
        return Ordering::Less;
    }
    if s2 == protocol::Special::NegInf {
        return Ordering::Greater;
    }
    if s1 == protocol::Special::NegOne {
        return Ordering::Less;
    }
    if s2 == protocol::Special::NegOne {
        return Ordering::Greater;
    }
    if s1 == protocol::Special::Zero {
        return Ordering::Less;
    }
    if s2 == protocol::Special::Zero {
        return Ordering::Greater;
    }
    if s1 == protocol::Special::PosOne {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

fn compare_hybrid(s: protocol::Special, p: Option<protocol::Primer>) -> Ordering {
    if s == protocol::Special::NegInf {
        return Ordering::Less;
    }
    if let Some(protocol::Primer::Ground) = p {
        return Ordering::Greater;
    }
    if s == protocol::Special::NegOne {
        return Ordering::Less;
    }
    if let Some(protocol::Primer::Reflect) = p {
        return Ordering::Greater;
    }
    if s == protocol::Special::Zero {
        return Ordering::Less;
    }
    if let None = p {
        return Ordering::Greater;
    }
    if s == protocol::Special::PosOne {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

fn compare_others(p1: Option<protocol::Primer>, c1: Clog, p2: Option<protocol::Primer>, c2: Clog) -> Ordering {
    compare_primers(&p1, &p2)
        .then(compare_clogs(c1, c2, p1 != Some(protocol::Primer::Reflect) && p1 != Some(protocol::Primer::Turn)))
}

fn compare_primers(p1: &Option<protocol::Primer>, p2: &Option<protocol::Primer>) -> Ordering {
    if p1 == p2 {
        return Ordering::Equal;
    }
    if let Some(protocol::Primer::Ground) = p1 {
        return Ordering::Less;
    }
    if let Some(protocol::Primer::Ground) = p2 {
        return Ordering::Greater;
    }
    if let Some(protocol::Primer::Reflect) = p1 {
        return Ordering::Less;
    }
    if let Some(protocol::Primer::Reflect) = p2 {
        return Ordering::Greater;
    }
    if let None = p1 {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

fn compare_clogs(mut c1: Clog, mut c2: Clog, mut polarity: bool) -> Ordering {
    let mut e1: Option<protocol::Reduction>;
    let mut e2: Option<protocol::Reduction>;
    loop {
        e1 = c1.egest();
        e2 = c2.egest();
        if e1 != e2 {
            break;
        }
        match e1 {
            None => {
                return Ordering::Equal;
            },
            Some(protocol::Reduction::Uncover) => {
                polarity = !polarity;
            },
            _ => {},
        }
    }
    let answer = match e1 {
        None => match e2 {
            Some(protocol::Reduction::Amplify) => Ordering::Greater,
            _ => Ordering::Less,
        },
        Some(protocol::Reduction::Amplify) => Ordering::Less,
        Some(protocol::Reduction::Uncover) => Ordering::Greater,
    };
    if polarity {
        answer
    }
    else {
        answer.reverse()
    }
}
