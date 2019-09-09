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

fn result_ratio(a: isize, b: isize, c: isize, d: isize, mut xn: isize, mut xd: isize) -> (isize, isize) {
    assert!(xn != 0 || xd != 0);
    if xn == 0 {
        xd = 1;
    }
    if xd < 0 {
        xn *= -1;
        xd *= -1;
    }
    (a * xn + b * xd, c * xn + d * xd)
}

#[test]
fn compare_homographics() {
    let range = 6;
    for a in -range..range {
        for b in -range..range {
            for c in -range..range {
                for d in -range..range {
                    if a == 0 && b == 0 && c == 0 && d == 0 {
                        continue;
                    }
                    for xn in -range..range {
                        for xd in -range..range {
                            if xn == 0 && xd == 0 {
                                continue;
                            }
                            let (rn, rd) = result_ratio(a, b, c, d, xn, xd);
                            if rn == 0 && rd == 0 {
                                continue;
                            }
                            // println!("{} {} {} {} ({} {}) = {} {}", a, b, c, d, xn, xd, rn, rd);
                            assert_eq!(Number::compare(Number::homographic(Number::ratio(xn, xd), a, b, c, d), Number::ratio(rn, rd)), Ordering::Equal);
                        }
                    }
                }
            }
        }
    }
}
