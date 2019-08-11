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
use clarith::Number;

fn reference_compare(mut n1: isize, mut d1: isize, mut n2: isize, mut d2: isize) -> Ordering {
    assert!(n1 != 0 || d1 != 0);
    assert!(n2 != 0 || d2 != 0);
    if d1 == 0 { n1 /= n1.abs(); }
    if d2 == 0 { n2 /= n2.abs(); }
    if d1 < 0 { n1 *= -1; d1 *= -1; }
    if d2 < 0 { n2 *= -1; d2 *= -1; }
    let c = if d1 != 0 || d2 != 0 { n1 * d2 - n2 * d1 } else { n1 - n2 };
    if c > 0 {
        Ordering::Greater
    }
    else if c < 0 {
        Ordering::Less
    }
    else {
        Ordering::Equal
    }
}

#[test]
fn compare_ratios() {
    let range = 25;
    let mut n1 = -range;
    while n1 <= range {
        let mut d1 = -range;
        while d1 <= range {
            let mut n2 = -range;
            while n2 <= range {
                let mut d2 = -range;
                while d2 <= range {
                    assert_eq!(
                        Number::compare(
                            Number::from_ratio(n1, d1),
                            Number::from_ratio(n2, d2)),
                        reference_compare(n1, d1, n2, d2));
                    d2 += 1;
                    if n2 == 0 && d2 == 0 {
                        d2 += 1;
                    }
                }
                n2 += 1;
            }
            d1 += 1;
            if n1 == 0 && d1 == 0 {
                d1 += 1;
            }
        }
        n1 += 1;
    }
}
