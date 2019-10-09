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

pub fn equal_to_one_half(mut n: isize, mut d: isize) -> bool {
    if d % 2 == 0 {
        d /= 2;
    } else {
        n = n.checked_mul(2).unwrap();
    }
    n == d
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

// FIXME: superfluous
pub fn not_less_than_one(n: isize, d: isize) -> bool {
    if d > 0 {
        n >= d
    } else if d < 0 {
        n <= d
    } else {
        n > 0
    }
}

// FIXME: superfluous
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
    mut nmin: isize,
    mut dmin: isize,
    mut nmax: isize,
    mut dmax: isize,
    mut nnew: isize,
    mut dnew: isize,
) -> (isize, isize, isize, isize) {
    // FIXME: emerge ratio fixes
    if dmin == 0 {
        nmin = nmin.signum();
    }
    if dmax == 0 {
        nmax = nmax.signum();
    }
    if dnew == 0 {
        nnew = nnew.signum();
    }

    if dmin < 0 {
        nmin *= -1;
        dmin *= -1;
    }
    if dmax < 0 {
        nmax *= -1;
        dmax *= -1;
    }
    if dnew < 0 {
        nnew *= -1;
        dnew *= -1;
    }

    if nmin == 0 {
        dmin = 1;
    }
    if nmax == 0 {
        dmax = 1;
    }
    if nnew == 0 {
        dnew = 1;
    }

    // assert!(dmin != 0 && dmax != 0 && dnew != 0, "division by zero");
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
        // (n0 * d0.signum()).checked_mul(d1.abs()).unwrap(),
        // (n1 * d1.signum()).checked_mul(d0.abs()).unwrap(),
        n0.checked_mul(d1).unwrap(),
        n1.checked_mul(d0).unwrap(),
    )
}

pub fn is_even(n: isize) -> bool {
    n % 2 == 0
}

pub fn are_same(n1: isize, d1: isize, n2: isize, d2: isize) -> bool {
    n1 == n2 && d1 == d2
}

pub fn is_special(n: isize, d: isize) -> bool {
    equal_to_zero(n, d) || equal_to_minus_one(n, d) || equal_to_one(n, d)
}

pub fn equal_to_zero(n: isize, d: isize) -> bool {
    d != 0 && n == 0
}

pub fn equal_to_one(n: isize, d: isize) -> bool {
    d != 0 && n == d
}

pub fn equal_to_minus_one(n: isize, d: isize) -> bool {
    d != 0 && n == -d
}
