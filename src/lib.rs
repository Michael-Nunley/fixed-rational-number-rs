//use serde::{Deserialize, Serialize};
use cordic::*;



pub trait FloatingPoint: Into<f64> {}
impl FloatingPoint for f32  {}
impl FloatingPoint for f64  {}

pub trait Integer {
    fn to_i128(self) -> i128;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(impl Integer for $t {
            #[inline]
            fn to_i128(self) -> i128 {
                self.into()
            }
        })*
    };
}

impl_integer!(i128, i64, u64, i32, u32, i16, u16, i8, u8);

impl Integer for u128 {
    fn to_i128(self) -> i128 {
        if self > i128::MAX as u128 {
            panic!("Value too large to fit into i128");
        }
        self as i128
    }
}




#[derive(Debug, Eq, Copy, PartialEq, /*Serialize, Deserialize,*/ Clone, Default)]
pub struct Num {
    big: i128,
}

#[allow(dead_code)]
impl Num {
    pub fn new(l: i128, r: u8) -> Self {
        if l.abs() > 664613997892457936451903530140172286_i128 {
            panic!("too big num!");
        }
        Self {
            //big: ((l >> 127) << 127) | (l << 8) | (r as i128),
            big: (l << 8) | (r as i128),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            big : self.big & (0b_1111_1111), //right-most 8 bits.
        }
    }

    pub fn zero() -> Self {
        Self { big: 0, }
    }

    pub fn one() -> Self {
        Self { big: 1 << 8, }
    }

    pub fn frac_pi_2() -> Self {
        Self { big: (3 << 8) | (36), }
    }

    pub fn pi() -> Self {
        Self { big: (3 << 8) | (36), }
    }

    pub fn e() -> Self {
        Self { big: (2 << 8) | (183),}
    }

    pub fn from_u0f64(val: u64) -> Self {
        Self {
            big: ((val as i128) << 8),
        }
    }

    pub fn num_bits() -> u8 { 128_u8 }
    pub fn num_fract_bits() -> u8 { 8_u8}
}


use std::ops::Add;
impl Add<Num> for Num {
    type Output = Self;
    fn add(self, other: Num) -> Num {
        Self {
            big: self.big + other.big,
        }
    }
}
impl<T: Integer>  Add<T> for Num {
    type Output = Self;
    fn add(self, other: T) -> Num {
        Self {
            big: self.big + (other.to_i128() << 8),
        }
    }
}
impl Add<f64> for Num {
    type Output = Self;
    fn add(self, other: f64) -> Num {
        if !other.is_finite() {
            panic!("Tried to add a non-finite float to Num!");
        }
        if other.signum() == 1.0 {
            Self {
                big: self.big
                    + (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        } else {
            Self {
                big: self.big
                	- (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        }
    }
}
impl Add<f32> for Num {
    type Output = Self;
    fn add(self, other: f32) -> Num {
        if !other.is_finite() {
            panic!("Tried to add a non-finite float to Num!");
        }
        if other.signum() == 1.0 {
            Self {
                big: self.big
                    + (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        } else {
            Self {
                big: self.big
                	- (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        }
    }
}

use std::ops::Sub;
impl Sub<Num> for Num {
    type Output = Self;
    fn sub(self, other: Num) -> Num {
        Self {
            big: self.big - other.big,
        }
    }
}
impl<T: Integer>  Sub<T> for Num {
    type Output = Self;
    fn sub(self, other: T) -> Num {
        Self {
            big: self.big - (other.to_i128() << 8),
        }
    }
}
impl Sub<f64> for Num {
    type Output = Self;
    fn sub(self, other: f64) -> Num {
        if !other.is_finite() {
            panic!("Tried to sub a non-finite float to Num!");
        }
        if other.signum() == 1.0 {
            Self {
                big: self.big
                	- (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        } else {
            Self {
                big: self.big
                    + (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        }
    }
}
impl Sub<f32> for Num {
    type Output = Self;
    fn sub(self, other: f32) -> Num {
        if !other.is_finite() {
            panic!("Tried to sub a non-finite float to Num!");
        }
        if other.signum() == 1.0 {
            Self {
                big: self.big
                	- (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        } else {
            Self {
                big: self.big
                    + (((other.fract()+0.00195) * 256.0).floor() as i128)
                    // the +0.002 is so that the error is +- 0.002, otherwise it would be upto -0.004,+0 . I felt the symmetric error is preferable.
                    + ((other.trunc() as i128) << 8),
            }
        }
    }
}

use std::ops::Mul;
impl Mul<Num> for Num {
    type Output = Self;
    fn mul(self, other: Num) -> Num {
        Self {
            big: (self.big * other.big) >> 8,
        }
    }
}
impl<T: Integer>  Mul<T> for Num {
    type Output = Self;
    fn mul(self, other: T) -> Num {
        Self {
            big: (self.big * other.to_i128() ) >> 8,
        }
    }
}

impl Mul<f64> for Num {
    type Output = Self;
    fn mul(self, other: f64) -> Num {
        Self {
            big: (self.big
                * (((other.fract() * 256.0).floor() as i128) + ((other.trunc() as i128) << 8)))
                >> 8,
        }
    }
}
impl Mul<f32> for Num {
    type Output = Self;
    fn mul(self, other: f32) -> Num {
        Self {
            big: (self.big
                * (((other.fract() * 256.0).floor() as i128) + ((other.trunc() as i128) << 8)))
                >> 8,
        }
    }
}




fn get_decimal_numbers(num: f64) -> String {
    let decimal_part = num.fract().abs().to_string();
    let start_index = decimal_part.find('.').map_or(0, |idx| idx + 1);
    decimal_part[start_index..].to_string()
}
use std::fmt;
impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0}.{1}",
            ((self.big >> 127) << 127) | ((self.big & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFF00) >> 8),
            get_decimal_numbers(((self.big & 0xFF) as f64) * 0.00390625_f64)
        )
    }
}
impl fmt::Binary for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.big, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = Num::one() + Num::one();
        assert_eq!(result, Num::new(2,0));
    }
}
