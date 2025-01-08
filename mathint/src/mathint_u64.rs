#[derive(Eq, Debug, Copy, Clone)]
pub struct MathIntU64(u64);

mod rt_decls {
    type BoolU64 = u64;

    extern "C" {
        pub fn CVT_mathint_u64_eq(_: u64, _: u64) -> BoolU64;
        pub fn CVT_mathint_u64_lt(_: u64, _: u64) -> BoolU64;
        pub fn CVT_mathint_u64_le(_: u64, _: u64) -> BoolU64;

        pub fn CVT_mathint_u64_add(_: u64, _: u64) -> u64;
        pub fn CVT_mathint_u64_mul(_: u64, _: u64) -> u64;
        pub fn CVT_mathint_u64_div(_: u64, _: u64) -> u64;
        pub fn CVT_mathint_u64_div_ceil(_: u64, _: u64) -> u64;

        pub fn CVT_mathint_u64_nondet() -> u64;
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_eq(a: u64, b: u64) -> u64 {
        (a == b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_lt(a: u64, b: u64) -> u64 {
        (a < b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_le(a: u64, b: u64) -> u64 {
        (a <= b).into()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_add(a: u64, b: u64) -> u64 {
        a.checked_add(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_mul(a: u64, b: u64) -> u64 {
        a.checked_mul(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_div(a: u64, b: u64) -> u64 {
        a.checked_div(b).unwrap()
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_div_ceil(a: u64, b: u64) -> u64 {
        a.div_ceil(b)
    }

    #[no_mangle]
    pub extern "C" fn CVT_mathint_u64_nondet() -> u64 {
        /* nondet::nondet() */
        0
    }
}

use rt_decls::*;

impl MathIntU64 {
    pub fn div_ceil(self, rhs: Self) -> Self {
        unsafe { Self(CVT_mathint_u64_div_ceil(self.0, rhs.0)) }
    }
}

impl PartialEq for MathIntU64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CVT_mathint_u64_eq(self.0, other.0) != 0 }
    }
}

impl PartialOrd for MathIntU64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = if self.0 == other.0 {
            std::cmp::Ordering::Equal
        } else if self.0 < other.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        };
        Some(ord)
    }
    fn lt(&self, other: &MathIntU64) -> bool {
        unsafe { CVT_mathint_u64_lt(self.0, other.0) != 0 }
    }
    fn le(&self, other: &MathIntU64) -> bool {
        unsafe { CVT_mathint_u64_le(self.0, other.0) != 0 }
    }
    fn gt(&self, other: &MathIntU64) -> bool {
        other.lt(self)
    }
    fn ge(&self, other: &MathIntU64) -> bool {
        other.le(self)
    }
}

impl Ord for MathIntU64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.lt(&other) {
            std::cmp::Ordering::Less
        } else if self.gt(&other) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }

    fn max(self, other: Self) -> Self {
        if self.gt(&other) {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self {
        if self.gt(&other) {
            other
        } else {
            self
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self.gt(&max) {
            max
        } else if self.lt(&min) {
            min
        } else {
            self
        }
    }
}

impl std::ops::Add<MathIntU64> for MathIntU64 {
    type Output = Self;

    fn add(self, rhs: MathIntU64) -> Self::Output {
        unsafe { Self(CVT_mathint_u64_add(self.0, rhs.0)) }
    }
}

impl std::ops::Mul<MathIntU64> for MathIntU64 {
    type Output = Self;

    fn mul(self, rhs: MathIntU64) -> Self::Output {
        unsafe { Self(CVT_mathint_u64_mul(self.0, rhs.0)) }
    }
}

impl std::ops::Div<MathIntU64> for MathIntU64 {
    type Output = Self;

    fn div(self, rhs: MathIntU64) -> Self::Output {
        unsafe { Self(CVT_mathint_u64_div(self.0, rhs.0)) }
    }
}

impl std::ops::Add<u64> for MathIntU64 {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self + Self(rhs)
    }
}

impl std::ops::Mul<u64> for MathIntU64 {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self * Self(rhs)
    }
}

impl std::ops::Div<u64> for MathIntU64 {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        self / Self(rhs)
    }
}

impl From<u64> for MathIntU64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl nondet::Nondet for MathIntU64 {
    fn nondet() -> MathIntU64 {
        unsafe { Self(CVT_mathint_u64_nondet()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = MathIntU64(2);
        let y = MathIntU64(4);
        assert_eq!(x + y, MathIntU64(6));
        assert_eq!(x + y, 6.into());
        assert!(x < 6.into());
    }
}
