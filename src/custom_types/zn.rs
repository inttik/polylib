//! Defines type `Zn` is remains of n.

use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{One, Zero};

/// Struct, that hold remain of n.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Zn<const N: u32>(u32);

impl<const N: u32> Zn<N> {
    /// Creates Zn. If value is equal to or more than N, takes only remain.
    ///
    /// Example:
    /// ```
    /// # use polylib::custom_types::Zn;
    /// let val = Zn::<5>::new(7); // 7 > 5, so val is 2
    /// ```
    pub fn new(value: u32) -> Zn<N> {
        Zn::<N>(value % N)
    }

    /// Returns holding value.
    /// 
    /// Example:
    /// ```
    /// # use polylib::custom_types::Zn;
    /// let val = Zn::<5>::new(7);
    /// assert_eq!(val.value(), 2);
    /// ``` 
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl<const N: u32> Zero for Zn<N> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

impl<const N: u32> One for Zn<N> {
    fn one() -> Self {
        if N == 0 {
            panic!("can't create one for Z0");
        }
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        self.value() == 1
    }
}

impl<const N: u32> Add for Zn<N> {
    type Output = Zn<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            0: (self.0 + rhs.0) % N,
        }
    }
}

impl<const N: u32> AddAssign for Zn<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % N;
    }
}

impl<const N: u32> Sub for Zn<N> {
    type Output = Zn<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            0: (self.0 + N - rhs.0) % N,
        }
    }
}

impl<const N: u32> SubAssign for Zn<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + N - rhs.0) % N;
    }
}

impl<const N: u32> Mul for Zn<N> {
    type Output = Zn<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            0: (self.0 * rhs.0) % N,
        }
    }
}

impl<const N: u32> MulAssign for Zn<N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 * rhs.0) % N;
    }
}

impl<const N: u32> Display for Zn<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Z{} {}>", N, self.value())
    }
}

#[cfg(test)]
mod zn_tests {
    use super::Zn;
    use crate::{One, Zero};

    fn check_add(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let a = Z5::new(lhs);
        let b = Z5::new(rhs);
        assert_eq!(
            (a + b).value(),
            ans5,
            "{} + {} != {} for Z5",
            lhs,
            rhs,
            ans5
        );

        let a = Z10::new(lhs);
        let b = Z10::new(rhs);
        assert_eq!(
            (a + b).value(),
            ans10,
            "{} + {} != {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let a = Z100::new(lhs);
        let b = Z100::new(rhs);
        assert_eq!(
            (a + b).value(),
            ans100,
            "{} + {} != {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    fn check_add_assign(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let mut a = Z5::new(lhs);
        a += Z5::new(rhs);
        assert_eq!(a.value(), ans5, "{} += {} is not {} for Z5", lhs, rhs, ans5);

        let mut a = Z10::new(lhs);
        a += Z10::new(rhs);
        assert_eq!(
            a.value(),
            ans10,
            "{} += {} is not {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let mut a = Z100::new(lhs);
        a += Z100::new(rhs);
        assert_eq!(
            a.value(),
            ans100,
            "{} += {} is not {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    fn check_rem(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let a = Z5::new(lhs);
        let b = Z5::new(rhs);
        assert_eq!(
            (a - b).value(),
            ans5,
            "{} - {} != {} for Z5",
            lhs,
            rhs,
            ans5
        );

        let a = Z10::new(lhs);
        let b = Z10::new(rhs);
        assert_eq!(
            (a - b).value(),
            ans10,
            "{} - {} != {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let a = Z100::new(lhs);
        let b = Z100::new(rhs);
        assert_eq!(
            (a - b).value(),
            ans100,
            "{} - {} != {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    fn check_rem_assign(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let mut a = Z5::new(lhs);
        a -= Z5::new(rhs);
        assert_eq!(a.value(), ans5, "{} -= {} is not {} for Z5", lhs, rhs, ans5);

        let mut a = Z10::new(lhs);
        a -= Z10::new(rhs);
        assert_eq!(
            a.value(),
            ans10,
            "{} -= {} is not {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let mut a = Z100::new(lhs);
        a -= Z100::new(rhs);
        assert_eq!(
            a.value(),
            ans100,
            "{} -= {} is not {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    fn check_mul(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let a = Z5::new(lhs);
        let b = Z5::new(rhs);
        assert_eq!(
            (a * b).value(),
            ans5,
            "{} * {} != {} for Z5",
            lhs,
            rhs,
            ans5
        );

        let a = Z10::new(lhs);
        let b = Z10::new(rhs);
        assert_eq!(
            (a * b).value(),
            ans10,
            "{} * {} != {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let a = Z100::new(lhs);
        let b = Z100::new(rhs);
        assert_eq!(
            (a * b).value(),
            ans100,
            "{} * {} != {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    fn check_mul_assign(lhs: u32, rhs: u32, ans5: u32, ans10: u32, ans100: u32) {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        let mut a = Z5::new(lhs);
        a *= Z5::new(rhs);
        assert_eq!(a.value(), ans5, "{} *= {} is not {} for Z5", lhs, rhs, ans5);

        let mut a = Z10::new(lhs);
        a *= Z10::new(rhs);
        assert_eq!(
            a.value(),
            ans10,
            "{} *= {} is not {} for Z10",
            lhs,
            rhs,
            ans10
        );

        let mut a = Z100::new(lhs);
        a *= Z100::new(rhs);
        assert_eq!(
            a.value(),
            ans100,
            "{} *= {} is not {} for Z100",
            lhs,
            rhs,
            ans100
        );
    }

    #[test]
    fn test_create() {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;

        let a = Z5::new(0);
        assert_eq!(a.value(), 0);

        let a = Z5::new(3);
        assert_eq!(a.value(), 3);

        let a = Z5::new(7);
        assert_eq!(a.value(), 2);

        let a = Z10::new(7);
        assert_eq!(a.value(), 7);
    }

    #[test]
    fn test_add() {
        check_add(1, 1, 2, 2, 2);
        check_add(3, 4, 2, 7, 7);
        check_add(32, 99, 1, 1, 31);
    }

    #[test]
    fn test_add_assign() {
        check_add_assign(1, 1, 2, 2, 2);
        check_add_assign(3, 4, 2, 7, 7);
        check_add_assign(32, 99, 1, 1, 31);
    }

    #[test]
    fn test_rem() {
        check_rem(1, 0, 1, 1, 1);
        check_rem(1, 1, 0, 0, 0);
        check_rem(4, 2, 2, 2, 2);
        check_rem(2, 4, 3, 8, 98);
        check_rem(32, 99, 3, 3, 33);
    }

    #[test]
    fn test_rem_assign() {
        check_rem_assign(1, 0, 1, 1, 1);
        check_rem_assign(1, 1, 0, 0, 0);
        check_rem_assign(4, 2, 2, 2, 2);
        check_rem_assign(2, 4, 3, 8, 98);
        check_rem_assign(32, 99, 3, 3, 33);
    }

    #[test]
    fn test_mul() {
        check_mul(1, 0, 0, 0, 0);
        check_mul(0, 1, 0, 0, 0);
        check_mul(1, 1, 1, 1, 1);
        check_mul(2, 3, 1, 6, 6);
        check_mul(32, 99, 3, 8, 68);
    }

    #[test]
    fn test_mul_assign() {
        check_mul_assign(1, 0, 0, 0, 0);
        check_mul_assign(0, 1, 0, 0, 0);
        check_mul_assign(1, 1, 1, 1, 1);
        check_mul_assign(2, 3, 1, 6, 6);
        check_mul_assign(32, 99, 3, 8, 68);
    }

    #[test]
    fn test_one() {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        assert_eq!(Z5::new(1) * Z5::one(), Z5::new(1));
        assert_eq!(Z5::new(0) * Z5::one(), Z5::new(0));
        assert_eq!(Z5::new(3) * Z5::one(), Z5::new(3));

        assert_eq!(Z10::new(1) * Z10::one(), Z10::new(1));
        assert_eq!(Z10::new(0) * Z10::one(), Z10::new(0));
        assert_eq!(Z10::new(8) * Z10::one(), Z10::new(8));

        assert_eq!(Z100::new(1) * Z100::one(), Z100::new(1));
        assert_eq!(Z100::new(0) * Z100::one(), Z100::new(0));
        assert_eq!(Z100::new(37) * Z100::one(), Z100::new(37));

        assert_eq!(Z5::one() * Z5::one(), Z5::one());
        assert_eq!(Z10::one() * Z10::one(), Z10::one());
        assert_eq!(Z100::one() * Z100::one(), Z100::one());
    }

    #[test]
    fn test_zero() {
        type Z5 = Zn<5>;
        type Z10 = Zn<10>;
        type Z100 = Zn<100>;

        assert_eq!(Z5::new(1) + Z5::zero(), Z5::new(1));
        assert_eq!(Z5::new(0) + Z5::zero(), Z5::new(0));
        assert_eq!(Z5::new(3) - Z5::zero(), Z5::new(3));

        assert_eq!(Z10::new(1) + Z10::zero(), Z10::new(1));
        assert_eq!(Z10::new(0) + Z10::zero(), Z10::new(0));
        assert_eq!(Z10::new(8) - Z10::zero(), Z10::new(8));

        assert_eq!(Z100::new(1) + Z100::zero(), Z100::new(1));
        assert_eq!(Z100::new(0) + Z100::zero(), Z100::new(0));
        assert_eq!(Z100::new(37) - Z100::zero(), Z100::new(37));

        assert_eq!(Z5::zero() + Z5::zero(), Z5::zero());
        assert_eq!(Z10::zero() + Z10::zero(), Z10::zero());
        assert_eq!(Z100::zero() + Z100::zero(), Z100::zero());
    }

    #[test]
    #[should_panic]
    fn test_z0_one() {
        type Z0 = Zn<0>;

        Z0::one();
    }
}
