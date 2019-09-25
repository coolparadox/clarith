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

use clarith::Number;
use std::cmp::Ordering;

#[test]
fn test_compare() {
    let range = 25;
    for n1 in -range..range {
        for d1 in -range..range {
            if d1 == 0 {
                continue;
            }
            for n2 in -range..range {
                for d2 in -range..range {
                    if d2 == 0 {
                        continue;
                    }
                    assert_eq!(
                        Number::compare(Number::ratio(n1, d1), Number::ratio(n2, d2)),
                        reference_compare(n1, d1, n2, d2)
                    );
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
                            if xd == 0 {
                                continue;
                            }
                            let (rn, rd) = expected_homographic(nx, n, dx, d, xn, xd);
                            if rd == 0 {
                                continue;
                            }
                            println!(
                                "{} {} {} {} ({} {}) = ({} {})",
                                nx, n, dx, d, xn, xd, rn, rd
                            );
                            assert_eq!(
                                Number::compare(
                                    Number::homographic(Number::ratio(xn, xd), nx, n, dx, d),
                                    Number::ratio(rn, rd)
                                ),
                                Ordering::Equal
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn compare_combines() {
    for nxy in 0..2 {
        for dxy in 0..2 {
            for nx in 0..2 {
                for dx in 0..2 {
                    for ny in 0..2 {
                        for dy in 0..2 {
                            for n in 0..2 {
                                for d in 0..2 {
                                    compare_combine(nxy, nx, ny, n, dxy, dx, dy, d);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn reference_compare(mut n1: isize, mut d1: isize, mut n2: isize, mut d2: isize) -> Ordering {
    assert!(d1 != 0);
    assert!(d2 != 0);
    if d1 < 0 {
        n1 *= -1;
        d1 *= -1;
    }
    if d2 < 0 {
        n2 *= -1;
        d2 *= -1;
    }
    let dx = n1 * d2 - n2 * d1;
    if dx > 0 {
        Ordering::Greater
    } else if dx < 0 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn compare_combine(
    nxy: isize,
    nx: isize,
    ny: isize,
    n: isize,
    dxy: isize,
    dx: isize,
    dy: isize,
    d: isize,
) {
    let inputs = vec![
        (-2, 1),
        (-1, 1),
        (-2, 3),
        (-1, 2),
        (-1, 4),
        (0, 1),
        (1, 4),
        (1, 2),
        (2, 3),
        (1, 1),
        (2, 1),
    ];

    for (xn, xd) in &inputs {
        for (yn, yd) in &inputs {
            let (rn, rd) = expected_combine(nxy, nx, ny, n, dxy, dx, dy, d, *xn, *xd, *yn, *yd);
            if rd == 0 {
                continue;
            }
            println!(
                "{} {} {} {}  {} {} {} {}  ({} {})  ({} {})  =  ({} {})",
                nxy, nx, ny, n, dxy, dx, dy, d, *xn, *xd, *yn, *yd, rn, rd
            );
            assert_eq!(
                Number::compare(
                    Number::combine(
                        Number::ratio(*xn, *xd),
                        Number::ratio(*yn, *yd),
                        nxy,
                        nx,
                        ny,
                        n,
                        dxy,
                        dx,
                        dy,
                        d
                    ),
                    Number::ratio(rn, rd)
                ),
                Ordering::Equal
            );
        }
    }
}

fn expected_homographic(
    nx: isize,
    n: isize,
    dx: isize,
    d: isize,
    xn: isize,
    xd: isize,
) -> (isize, isize) {
    let x = (xn, xd);
    div(add(mul((nx, 1), x), (n, 1)), add(mul((dx, 1), x), (d, 1)))
}

fn expected_combine(
    nxy: isize,
    nx: isize,
    ny: isize,
    n: isize,
    dxy: isize,
    dx: isize,
    dy: isize,
    d: isize,
    xn: isize,
    xd: isize,
    yn: isize,
    yd: isize,
) -> (isize, isize) {
    let x = (xn, xd);
    let y = (yn, yd);
    div(
        add(
            add(
                add(mul((nxy, 1), mul(x, y)), mul((nx, 1), x)),
                mul((ny, 1), y),
            ),
            (n, 1),
        ),
        add(
            add(
                add(mul((dxy, 1), mul(x, y)), mul((dx, 1), x)),
                mul((dy, 1), y),
            ),
            (d, 1),
        ),
    )
}

fn add(f1: (isize, isize), f2: (isize, isize)) -> (isize, isize) {
    let (n1, d1) = fix(f1);
    let (n2, d2) = fix(f2);
    fix((n1 * d2 + n2 * d1, d1 * d2))
}

fn div(f1: (isize, isize), f2: (isize, isize)) -> (isize, isize) {
    let (n1, d1) = fix(f1);
    let (n2, d2) = fix(f2);
    fix((n1 * d2, d1 * n2))
}

fn mul(f1: (isize, isize), f2: (isize, isize)) -> (isize, isize) {
    let (n1, d1) = fix(f1);
    let (n2, d2) = fix(f2);
    fix((n1 * n2, d1 * d2))
}

fn fix(f: (isize, isize)) -> (isize, isize) {
    let (n, d) = f;
    if d == 0 {
        (1, 0)
    } else if n == 0 {
        (0, 1)
    } else {
        let g = gcd(n, d);
        let ng = n / g;
        let dg = d / g;
        (ng * dg.signum(), dg * dg.signum())
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
