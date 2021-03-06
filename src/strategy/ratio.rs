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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic(expected = "division by zero")]
    fn forbids_undefined_ratio() {
        new(true, 0, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn forbids_negative_infinity() {
        new(false, 1, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn forbids_positive_infinity() {
        new(true, 1, 0);
    }

    #[test]
    fn supports_negative_two() {
        if let (None, Some(protocol::Primer::Ground), Some(mut ratio)) = new(false, 2, 1) {
            if let Ok(None) = ratio.egest() {
                return;
            }
        }
        panic!();
    }

    #[test]
    fn supports_negative_one() {
        if let (Some(protocol::Special::NegOne), None, None) = new(false, 1, 1) {
            return;
        }
        panic!();
    }

    #[test]
    fn supports_negative_two_thirds() {
        if let (None, Some(protocol::Primer::Reflect), Some(mut ratio)) = new(false, 2, 3) {
            if let Ok(Some(protocol::Reduction::Uncover)) = ratio.egest() {
                if let Ok(None) = ratio.egest() {
                    return;
                }
            }
        }
        panic!();
    }

    #[test]
    fn supports_negative_one_half() {
        if let (None, Some(protocol::Primer::Reflect), Some(mut ratio)) = new(false, 1, 2) {
            if let Ok(None) = ratio.egest() {
                return;
            }
        }
        panic!();
    }

    #[test]
    fn supports_negative_one_fourth() {
        if let (None, Some(protocol::Primer::Reflect), Some(mut ratio)) = new(false, 1, 4) {
            if let Ok(Some(protocol::Reduction::Amplify)) = ratio.egest() {
                if let Ok(None) = ratio.egest() {
                    return;
                }
            }
        }
        panic!();
    }

    #[test]
    fn supports_zero() {
        if let (Some(protocol::Special::Zero), None, None) = new(true, 0, 1) {
            return;
        }
        panic!();
    }

    #[test]
    fn supports_one_fourth() {
        if let (None, None, Some(mut ratio)) = new(true, 1, 4) {
            if let Ok(Some(protocol::Reduction::Amplify)) = ratio.egest() {
                if let Ok(None) = ratio.egest() {
                    return;
                }
            }
        }
        panic!();
    }

    #[test]
    fn supports_one_half() {
        if let (None, None, Some(mut ratio)) = new(true, 1, 2) {
            if let Ok(None) = ratio.egest() {
                return;
            }
        }
        panic!();
    }

    #[test]
    fn supports_two_thirds() {
        if let (None, None, Some(mut ratio)) = new(true, 2, 3) {
            if let Ok(Some(protocol::Reduction::Uncover)) = ratio.egest() {
                if let Ok(None) = ratio.egest() {
                    return;
                }
            }
        }
        panic!();
    }

    #[test]
    fn supports_one() {
        if let (Some(protocol::Special::PosOne), None, None) = new(true, 1, 1) {
            return;
        }
        panic!();
    }

    #[test]
    fn supports_two() {
        if let (None, Some(protocol::Primer::Turn), Some(mut ratio)) = new(true, 2, 1) {
            if let Ok(None) = ratio.egest() {
                return;
            }
        }
        panic!();
    }

    #[test]
    fn one_half_is_final() {
        if let (None, None, Some(mut ratio)) = new(true, 3, 6) {
            for _ in 1..10 {
                if let Ok(None) = ratio.egest() {
                    continue;
                }
                panic!();
            }
            return;
        }
        panic!();
    }

    #[test]
    fn does_not_overflow() {
        if let (None, None, Some(mut ratio)) = new(true, usize::max_value() - 1, usize::max_value())
        {
            if let Ok(Some(protocol::Reduction::Uncover)) = ratio.egest() {
                return;
            }
        }
        panic!();
    }
}

pub struct Ratio {
    num: usize,
    den: usize,
}

pub fn new(
    positive: bool,
    num: usize,
    den: usize,
) -> (
    Option<protocol::Special>,
    Option<protocol::Primer>,
    Option<Ratio>,
) {
    if den == 0 {
        panic!("division by zero");
    }
    if num == 0 {
        return (Some(protocol::Special::Zero), None, None);
    }
    if num == den {
        if positive {
            return (Some(protocol::Special::PosOne), None, None);
        }
        return (Some(protocol::Special::NegOne), None, None);
    }
    if num > den {
        return (
            None,
            if positive {
                Some(protocol::Primer::Turn)
            } else {
                Some(protocol::Primer::Ground)
            },
            Some(Ratio { num: den, den: num }),
        );
    }
    return (
        None,
        if positive {
            None
        } else {
            Some(protocol::Primer::Reflect)
        },
        Some(Ratio { num: num, den: den }),
    );
}

pub fn new_i(
    num: isize,
    den: isize,
) -> (
    Option<protocol::Special>,
    Option<protocol::Primer>,
    Option<Ratio>,
) {
    new(
        (num >= 0 && den >= 0) || (num < 0 && den < 0),
        if num >= 0 { num } else { -num } as usize,
        if den >= 0 { den } else { -den } as usize,
    )
}

impl Strategy for Ratio {
    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        if self.num > self.den / 2 {
            std::mem::swap(&mut self.num, &mut self.den);
            self.num -= self.den;
            return Ok(Some(protocol::Reduction::Uncover));
        }
        if self.den % 2 == 0 {
            self.den /= 2;
            if self.num == self.den {
                self.den *= 2;
                return Ok(None);
            }
        } else {
            self.num *= 2;
        }
        return Ok(Some(protocol::Reduction::Amplify));
    }
}
