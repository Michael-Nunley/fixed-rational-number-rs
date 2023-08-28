use cordic::*;
use fixed::{types::extra::U28, types::U0F64, FixedI128};
use std::any;

type Fix = FixedI128<U28>;
#[derive(Debug, Eq, Copy, PartialEq, PartialOrd, Clone, Default)]
pub struct Num {
    i: Fix,
}

#[allow(dead_code)]
impl Num {
    pub fn num_bits() -> u8 {
        128_u8
    }
    pub fn num_fract_bits() -> u8 {
        28_u8
    }
    pub fn from_num<Src: fixed::traits::ToFixed>(src: Src) -> Self {
        Self {
            i: Fix::from_num(src),
        }
    }
    pub fn to_num<Dst: fixed::traits::FromFixed>(self) -> Dst {
        self.i.to_num()
    }
    pub fn get_type() -> fn() -> &'static str {
        any::type_name::<Num>
    }
    pub fn floor(self) -> Self {
        Self { i: self.i.floor() }
    }
    pub fn zero() -> Self {
        Self { i: Fix::ZERO }
    }
    pub fn one() -> Self {
        Self { i: Fix::ONE }
    }
    pub fn frac_pi_2() -> Self {
        Self { i: Fix::FRAC_PI_2 }
    }
    pub fn pi() -> Self {
        Self { i: Fix::PI }
    }
    pub fn e() -> Self {
        Self { i: Fix::E }
    }
    pub fn from_u0f64(val: U0F64) -> Self {
        Self { i: val.to_num() }
    }
}
use std::ops::{Add, AddAssign, Div, Mul, Neg, Shl, Shr, Sub, SubAssign};

impl AddAssign for Num {
    fn add_assign(&mut self, rhs: Self) {
        self.i = self.i + rhs.i;
    }
}

impl SubAssign for Num {
    fn sub_assign(&mut self, rhs: Self) {
        self.i = self.i - rhs.i;
    }
}

impl Div for Num {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { i: self.i / rhs.i }
    }
}

impl Mul for Num {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { i: self.i * rhs.i }
    }
}

impl Neg for Num {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { i: self.i.neg() }
    }
}

impl Sub for Num {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { i: self.i - rhs.i }
    }
}

impl Add for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { i: self.i + rhs.i }
    }
}

impl Shr<u8> for Num {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self { i: self.i >> rhs }
    }
}

impl Shl<u8> for Num {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self { i: self.i << rhs }
    }
}

impl CordicNumber for Num {
    fn floor(self) -> Self {
        self.floor()
    }
    fn zero() -> Self {
        Num::zero()
    }
    fn one() -> Self {
        Num::one()
    }
    fn frac_pi_2() -> Self {
        Num::frac_pi_2()
    }
    fn pi() -> Self {
        Num::pi()
    }
    fn e() -> Self {
        Num::e()
    }
    fn from_u0f64(val: U0F64) -> Self {
        Num::from_u0f64(val)
    }
    fn num_fract_bits() -> u8 {
        Num::num_fract_bits()
    }
    fn num_bits() -> u8 {
        Num::num_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq<T: core::fmt::Display>(
        input: T,
        computed: f64,
        expected: f64,
        max_err: f64,
    ) {
        let err = (computed - expected).abs();
        if err > max_err {
            panic!(
                "mismatch for input {}: computed {}, expected {}",
                input, computed, expected
            );
        }
    }
    pub const ERRMAX: f64 = 0.00000001_f64;

    #[test]
    fn num_new() {
        let result = Num::one().neg();
        assert_approx_eq(0, result.to_num(), Num::from_num(-1).to_num(), ERRMAX);
    }
    #[test]
    fn zero() {
        let result = Num::one().neg() + Num::one();
        let expected = Num::from_num(0);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn add() {
        let result = Num::one() + Num::one();
        let expected = Num::from_num(2);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn pos_floor() {
        let result = Num::e().floor();
        let expected = Num::from_num(2);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn neg_floor() {
        let result = Num::e().neg().floor();
        let expected = Num::from_num(-3);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn sqrt2() {
        let result = sqrt(Num::one() + Num::one());
        let expected = Num::from_num(fixed::consts::SQRT_2);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn cos_pi() {
        let result = cordic::cos(Num::pi());
        let expected = Num::from_num(-1);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
    #[test]
    fn cos0() {
        let result = cordic::cos(Num::zero());
        let expected = Num::from_num(1);
        assert_approx_eq(0, result.to_num(), expected.to_num(), ERRMAX);
    }
}
