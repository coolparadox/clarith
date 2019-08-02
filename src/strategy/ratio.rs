use crate::protocol;
use crate::strategy::Strategy;

pub struct Ratio {
    num: u64,
    den: u64,
}

impl Ratio {

    fn new(positive: bool, num: u64, den:u64) -> (Option<protocol::Fixed>, Option<protocol::Primer>, Option<Ratio>) {
        if num == 0 && den == 0 {
            panic!("undefined ratio");
        }
        else if den == 0 {
            if positive {
                (Some(protocol::Fixed::PosInf), None, None)
            }
            else {
                (Some(protocol::Fixed::NegInf), None, None)
            }
        }
        else if num == 0 {
            (Some(protocol::Fixed::Zero), None, None)
        }
        else if !positive {
            if num > den {
                (None, Some(protocol::Primer::Ground), Some(Ratio {num: den, den: num}))
            }
            else {
                (None, Some(protocol::Primer::Reflect), Some(Ratio {num: num, den: den}))
            }
        }
        else if num > den {
            (None, Some(protocol::Primer::Turn), Some(Ratio {num: den, den: num}))
        }
        else {
            (None, None, Some(Ratio {num: num, den: den}))
        }
    }

}

impl Strategy for Ratio {

    fn egest(&mut self) -> Result<Option<protocol::Reduction>, Box<dyn Strategy>> {
        if self.num == self.den {
            Ok(None)
        }
        else if self.num > self.den / 2 {
            std::mem::swap(&mut self.num, &mut self.den);
            self.num -= self.den;
            Ok(Some(protocol::Reduction::Uncover))
        }
        else {
            if self.den % 2 == 0 {
                self.den /= 2;
            }
            else {
                self.num *= 2;
            }
            Ok(Some(protocol::Reduction::Amplify))
        }
    }

}


