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

pub fn greater_than_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n > d
    } else if d < 0 {
        n < d
    } else {
        n > 0
    }
}

pub fn less_than_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n < d
    } else if d < 0 {
        n > d
    } else {
        n < 0
    }
}

pub fn greater_than_zero(n: isize, d: isize) -> bool {
    if d >= 0 {
        n > 0
    } else {
        n < 0
    }
}

pub fn less_than_zero(n: isize, d: isize) -> bool {
    if d >= 0 {
        n < 0
    } else {
        n > 0
    }
}

pub fn greater_than_minus_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n > -d
    } else if d < 0 {
        n < -d
    } else {
        n > 0
    }
}

pub fn less_than_minus_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n < -d
    } else if d < 0 {
        n > -d
    } else {
        n < 0
    }
}

pub fn greater_than_one_half(mut n: isize, mut d: isize) -> bool {
    if d % 2 == 0 {
        d /= 2;
    } else {
        n = n.checked_mul(2).unwrap();
    }
    if d > 0 {
        n > d
    } else if d < 0 {
        n < d
    } else {
        n > 0
    }
}

pub fn less_than_one_half(mut n: isize, mut d: isize) -> bool {
    if d % 2 == 0 {
        d /= 2;
    } else {
        n = n.checked_mul(2).unwrap();
    }
    if d > 0 {
        n < d
    } else if d < 0 {
        n > d
    } else {
        n < 0
    }
}

pub fn not_less_than_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n >= d
    } else if d < 0 {
        n <= d
    } else {
        n > 0
    }
}

pub fn not_greater_than_zero(n: isize, d: isize) -> bool {
    if d > 0 {
        n <= 0
    } else if d < 0 {
        n >= 0
    } else {
        n < 0
    }
}

pub fn updated_range(
    nmin: isize,
    dmin: isize,
    nmax: isize,
    dmax: isize,
    nnew: isize,
    dnew: isize,
) -> (isize, isize, isize, isize) {
    assert!(dmin != 0 && dmax != 0 && dnew != 0, "division by zero");
    if less_than(nnew, dnew, nmin, dmin) {
        (nnew, dnew, nmax, dmax)
    } else if greater_than(nnew, dnew, nmax, dmax) {
        (nmin, dmin, nnew, dnew)
    } else {
        (nmin, dmin, nmax, dmax)
    }
}

pub fn less_than(n0: isize, d0: isize, n1: isize, d1: isize) -> bool {
    let (ad, bc) = compare_reduce(n0, d0, n1, d1);
    ad < bc
}

pub fn greater_than(n0: isize, d0: isize, n1: isize, d1: isize) -> bool {
    let (ad, bc) = compare_reduce(n0, d0, n1, d1);
    ad > bc
}

pub fn compare_reduce(n0: isize, d0: isize, n1: isize, d1: isize) -> (isize, isize) {
    (
        (n0 * d0.signum()).checked_mul(d1.abs()).unwrap(),
        (n1 * d1.signum()).checked_mul(d0.abs()).unwrap(),
    )
}

pub fn is_even(n: isize) -> bool {
    n % 2 == 0
}
