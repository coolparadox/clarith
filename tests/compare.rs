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

#[test]
fn compare_ratios() {
    let range = 25;
    for n1 in -range..range {
        for d1 in -range..range {
            if n1 == 0 && d1 == 0 {
                continue;
            }
            for n2 in -range..range {
                for d2 in -range..range {
                    if n2 == 0 && d2 == 0 {
                        continue;
                    }
                    assert_eq!(
                        Number::compare(
                            Number::ratio(n1, d1),
                            Number::ratio(n2, d2)),
                        reference_compare(n1, d1, n2, d2));
                }
            }
        }
    }
}

#[test]
fn compare_homographics() {
    let range = 6;
    for nx in -range..range {
        for n in -range..range {
            for dx in -range..range {
                for d in -range..range {
                    if nx == 0 && n == 0 && dx == 0 && d == 0 {
                        continue;
                    }
                    for xn in -range..range {
                        for xd in -range..range {
                            if xn == 0 && xd == 0 {
                                continue;
                            }
                            let (rn, rd) = result_ratio(nx, n, dx, d, xn, xd);
                            if rn == 0 && rd == 0 {
                                continue;
                            }
                            // println!("{} {} {} {} ({} {}) = {} {}", nx, n, dx, d, xn, xd, rn, rd);
                            assert_eq!(Number::compare(Number::homographic(Number::ratio(xn, xd), nx, n, dx, d), Number::ratio(rn, rd)), Ordering::Equal);
                        }
                    }
                }
            }
        }
    }
}

fn reference_compare(mut n1: isize, mut d1: isize, mut n2: isize, mut d2: isize) -> Ordering {
    assert!(n1 != 0 || d1 != 0);
    assert!(n2 != 0 || d2 != 0);
    if d1 == 0 { n1 /= n1.abs(); }
    if d2 == 0 { n2 /= n2.abs(); }
    if d1 < 0 { n1 *= -1; d1 *= -1; }
    if d2 < 0 { n2 *= -1; d2 *= -1; }
    let dx = if d1 != 0 || d2 != 0 { n1 * d2 - n2 * d1 } else { n1 - n2 };
    if dx > 0 {
        Ordering::Greater
    }
    else if dx < 0 {
        Ordering::Less
    }
    else {
        Ordering::Equal
    }
}

fn result_ratio(nx: isize, n: isize, dx: isize, d: isize, mut xn: isize, mut xd: isize) -> (isize, isize) {
    assert!(xn != 0 || xd != 0);
    if xn == 0 {
        xd = 1;
    }
    if xd < 0 {
        xn *= -1;
        xd *= -1;
    }
    (nx * xn + n * xd, dx * xn + d * xd)
}
