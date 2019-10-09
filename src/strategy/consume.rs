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
// use crate::strategy::ratio;
// use crate::strategy::ratio::Ratio;
// use crate::strategy::support;
use crate::strategy::Strategy;
use crate::Clog;
use crate::Number;
use std::mem::swap;

#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn t_none() {
        let (xo, nx, n, dx, d) = Number::consume(Number::ratio(2953, 7829));
        assert!(xo.is_none());
        assert_eq!(
            Number::compare(
                Number::ratio(nx + 2 * n, dx + 2 * d),
                Number::ratio(2953, 7829)
            ),
            Ordering::Equal
        );
    }

    #[test]
    #[ignore] // FIXME: issue #2
    fn t_some() {
        let (xo, nx, n, dx, d) = Number::consume(Number::homographic(
            Number::ratio(isize::max_value(), 1),
            isize::max_value(),
            isize::max_value(),
            0,
            1,
        ));
        assert!(xo.is_some());
        assert_eq!(
            Number::compare(
                Number::homographic(xo.unwrap(), nx, n, dx, d),
                Number::homographic(
                    Number::ratio(isize::max_value(), 1),
                    isize::max_value(),
                    isize::max_value(),
                    0,
                    1
                )
            ),
            Ordering::Equal
        );
    }
}

pub struct Consume {
    x: Clog,
    nx: isize,
    n: isize,
    dx: isize,
    d: isize,
}

pub fn new(x: Number) -> (Option<Clog>, isize, isize, isize, isize) {
    if let Number::Special(s) = x {
        return match s {
            protocol::Special::NegOne => (None, 0, -1, 0, 1),
            protocol::Special::Zero => (None, 0, 0, 0, 1),
            protocol::Special::PosOne => (None, 0, 1, 0, 1),
        };
    }
    let mut nx = 1;
    let mut n = 0;
    let mut dx = 0;
    let mut d = 1;
    let (po, c) = x.unwrap_other();
    match po {
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
    Consume::new(c, nx, n, dx, d)
}

impl Consume {
    fn new(
        x: Clog,
        nx: isize,
        n: isize,
        dx: isize,
        d: isize,
    ) -> (Option<Clog>, isize, isize, isize, isize) {
        (Consume { x, nx, n, dx, d }).consume()
    }

    fn consume(mut self) -> (Option<Clog>, isize, isize, isize, isize) {
        loop {
            match self.x.egest() {
                Some(protocol::Reduction::Amplify) => {
                    self.amplify_x();
                }
                Some(protocol::Reduction::Uncover) => {
                    self.uncover_x();
                }
                None => {
                    return (None, self.nx, self.n, self.dx, self.d);
                }
            }
            if self.is_state_unsafe() {
                return (Some(self.x), self.nx, self.n, self.dx, self.d);
            }
        }
    }

    fn amplify_x(&mut self) {
        if self.nx % 2 != 0 || self.dx % 2 != 0 {
            self.n *= 2;
            self.d *= 2;
        } else {
            self.nx /= 2;
            self.dx /= 2;
        }
    }

    fn uncover_x(&mut self) {
        self.nx += self.n;
        self.dx += self.d;
        swap(&mut self.nx, &mut self.n);
        swap(&mut self.dx, &mut self.d);
    }

    fn is_state_unsafe(&self) -> bool {
        let lim = isize::max_value() / 4;
        self.nx.abs() >= lim || self.n.abs() >= lim || self.dx.abs() >= lim || self.d.abs() >= lim
    }

    /*

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
            if support::less_than_minus_one(nmax, dmax) {
                Ok(Some(self.ground()))
            } else if support::greater_than_minus_one(nmin, dmin) && support::less_than_zero(nmax, dmax)
            {
                Ok(Some(self.reflect()))
            } else if support::greater_than_zero(nmin, dmin) && support::less_than_one(nmax, dmax) {
                Ok(None)
            } else if support::greater_than_one(nmin, dmin) {
                Ok(Some(self.turn()))
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
            let (n1, d1) = self.value_at_one();
            support::updated_range(n0, d0, n0, d0, n1, d1)
        }

        fn are_singularities_outside_domain(&self) -> bool {
            self.is_zero_outside_domain() && self.is_pole_outside_domain()
        }

        fn is_domain_amenable(mx: isize, m: isize) -> bool {
            let s = m.signum();
            s != 0 && s == mx.checked_add(m).unwrap().signum()
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
            if self.are_singularities_outside_domain() {
                if let Ok(primer) = self.primer_egest() {
                    return (None, primer, None, Some(self));
                }
            }
            match self.prime_ingest() {
                Some((special, primer, ratio)) => (special, primer, ratio, None),
                None => self.prime(),
            }
        }

        fn reduction_egest(&mut self) -> Result<Option<protocol::Reduction>, isize> {
            let (nmin, dmin, nmax, dmax) = self.image_extremes();
            // FIXME: remove sanity checks?
            if support::not_greater_than_zero(nmin, dmin) {
                panic!("logic error");
            }
            if support::not_less_than_one(nmax, dmax) {
                panic!("logic error");
            }
            if support::less_than_one_half(nmax, dmax) {
                Ok(Some(self.amplify()))
            } else if support::greater_than_one_half(nmin, dmin) {
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

    */
}

impl Strategy for Consume {
    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }
}
