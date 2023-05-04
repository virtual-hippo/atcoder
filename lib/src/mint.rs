use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct Mint {
    v: i64,
}

const MOD: i64 = 998244353;

impl Mint {
    pub fn new(v: i64) -> Mint {
        Mint {
            v: (v % MOD + MOD) % MOD,
        }
    }
}

impl Neg for Mint {
    type Output = Mint;
    fn neg(self) -> Self::Output {
        Self::new(-self.v)
    }
}

impl Add for Mint {
    type Output = Mint;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.v + rhs.v)
    }
}

impl Sub for Mint {
    type Output = Mint;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.v - rhs.v)
    }
}

impl Mul for Mint {
    type Output = Mint;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.v * rhs.v)
    }
}

impl AddAssign<&Mint> for Mint {
    fn add_assign(&mut self, rhs: &Self) {
        if self.v + rhs.v >= MOD {
            self.v -= MOD
        }
    }
}

impl SubAssign<&Mint> for Mint {
    fn sub_assign(&mut self, rhs: &Self) {
        if self.v + (MOD - rhs.v) >= MOD {
            self.v -= MOD
        }
    }
}

impl MulAssign<&Mint> for Mint {
    fn mul_assign(&mut self, rhs: &Self) {
        self.v = (self.v * rhs.v) % MOD;
    }
}
