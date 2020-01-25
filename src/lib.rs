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

//! # Continued Logarithm Arithmetic
//!
//! `clarith` provides primitives for basic arithmetic
//! with numbers expressed in a continued logarithm representation.
//!
//! Continued logarithm is a way of representing rational numbers with unbounded precision.
//! Reference: <https://perl.plover.com/classes/cftalk/INFO/gosper.txt>

mod compare;
mod strategy;

pub mod protocol;

use crate::strategy::Strategy;
use std::cmp::Ordering;

/// A number greater than zero and lesser than one with unbounded precision.
pub struct Clog {
    strategy: Box<dyn Strategy>,
}

impl Clog {
    /**
     * Destructively extract the next Reduction symbol from self.
     * If self is one half, no symbol is returned.
     */
    pub fn egest(&mut self) -> Option<protocol::Reduction> {
        match self.strategy.egest() {
            Ok(reduction) => reduction,
            Err(new_strategy) => {
                self.strategy = new_strategy;
                self.egest()
            }
        }
    }
}

/// A finite unbounded number with unbounded precision.
pub enum Number {
    Special(protocol::Special),
    Other(Option<protocol::Primer>, Clog),
}

impl Number {
    /// Unwraps the Special content from self, or panic.
    pub fn unwrap_special(self) -> protocol::Special {
        match self {
            Number::Special(special) => special,
            _ => panic!("Number is not Special"),
        }
    }

    /// Unwraps the Other content from self, or panic.
    pub fn unwrap_other(self) -> (Option<protocol::Primer>, Clog) {
        match self {
            Number::Other(primer, clog) => (primer, clog),
            _ => panic!("Number is not Other"),
        }
    }

    /// Destructively compare two Numbers.
    pub fn compare(n1: Number, n2: Number) -> Ordering {
        compare::compare(n1, n2)
    }

    /// Construct a Number from the ratio of two signed machine integers.
    pub fn ratio(num: isize, den: isize) -> Number {
        Number::ratio_u(
            (num >= 0 && den >= 0) || (num < 0 && den < 0),
            if num >= 0 { num } else { -num } as usize,
            if den >= 0 { den } else { -den } as usize,
        )
    }

    /// Construct a Number from the ratio of two unsigned machine integers.
    pub fn ratio_u(positive: bool, num: usize, den: usize) -> Number {
        let (special, primer, ratio) = strategy::ratio::new(positive, num, den);
        if let Some(fixed) = special {
            return Number::Special(fixed);
        }
        Number::Other(
            primer,
            Clog {
                strategy: Box::new(ratio.unwrap()),
            },
        )
    }

    /**
     * Construct the Number _(nx * x + n) / (dx * x + d)_
     */
    pub fn homographic(x: Number, nx: isize, n: isize, dx: isize, d: isize) -> Number {
        let (special, primer, ratio, homographic) = strategy::homographic::new(x, nx, n, dx, d);
        if let Some(fixed) = special {
            Number::Special(fixed)
        } else if let Some(ratio) = ratio {
            Number::Other(
                primer,
                Clog {
                    strategy: Box::new(ratio),
                },
            )
        } else {
            Number::Other(
                primer,
                Clog {
                    strategy: Box::new(homographic.unwrap()),
                },
            )
        }
    }

    /**
     * Construct the Number _(nxy * x * y + nx * x + ny * y + n) / (dxy * x * y + dx * x + dy * y + d)_
     */
    pub fn combine(
        x: Number,
        y: Number,
        nxy: isize,
        nx: isize,
        ny: isize,
        n: isize,
        dxy: isize,
        dx: isize,
        dy: isize,
        d: isize,
    ) -> Number {
        let (special, primer, ratio, homographic, combine) =
            strategy::combine::new(x, y, nxy, nx, ny, n, dxy, dx, dy, d);
        if let Some(fixed) = special {
            Number::Special(fixed)
        } else if let Some(ratio) = ratio {
            Number::Other(
                primer,
                Clog {
                    strategy: Box::new(ratio),
                },
            )
        } else if let Some(homographic) = homographic {
            Number::Other(
                primer,
                Clog {
                    strategy: Box::new(homographic),
                },
            )
        } else {
            Number::Other(
                primer,
                Clog {
                    strategy: Box::new(combine.unwrap()),
                },
            )
        }
    }

    /**
     * Transfer information from a Number to a homograhic transformation.
     *
     * This funtion returns:
     *  - A Number _y_
     *  - Coefficients _ny_, _n_, _dy_, _d_
     *
     *  such that _(ny * y + n) / (dy * y + d)_ equals to the input _x_.
     *  
     *  The information extraction from the input _x_ proceeds until one of the following conditions is met:
     *   - The input reaches one half. (In this case _y_ is returned None.)
     *   - The returned coefficients are about to overflow.
     */
    pub fn consume(x: Number) -> (Option<Number>, isize, isize, isize, isize) {
        let (co, nx, n, dx, d) = strategy::consume::new(x);
        (co.map(|c| Number::Other(None, c)), nx, n, dx, d)
    }
}
