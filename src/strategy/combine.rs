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
use crate::strategy::Strategy;
use crate::Number;
use crate::strategy::ratio::Ratio;
use crate::strategy::homographic::Homographic;

pub struct Combine {
}

pub fn new(x: Number, y: Number, nxy: isize, nx: isize, ny: isize, n: isize, dxy: isize, dx: isize, dy: isize, d: isize) -> (Option<protocol::Special>, Option<protocol::Primer>, Option<Ratio>, Option<Homographic>, Option<Combine>) {
    (Some(protocol::Special::Zero), None, None, None, None)
}

impl Strategy for Combine {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        Ok(None)
    }

}
