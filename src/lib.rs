//! Polylib - simple lib about polynomials.
//! 
//! # Features
//! * Any of yours custom types, that implement common traits
//! * Any type substitution (even polynomial to polynomial substitution)
//! * Some common types, like `zn` and `matrix`
//! * No dependencies at all. No need to monitor deep library modifications.
//! 

pub mod custom_types;
pub mod polynom;

/**
 * Means, that numeric type has `zero`
 * (Add neutral element).
 *
*/
pub trait Zero {
    /// Return `zero` item of numeric group.
    fn zero() -> Self;
    /// Check if self is `zero`.
    fn is_zero(&self) -> bool;
}

/**
 * Means, that numeric type has `one`
 * (Mul neutral element).
 *
*/
pub trait One {
    /// Return `one` item of numeric group.
    fn one() -> Self;
    /// Check if self is `one`.
    fn is_one(&self) -> bool;
}

impl<T> Zero for T
where
    T: From<u8> + PartialEq,
{
    fn zero() -> Self {
        T::from(0)
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl<T> One for T
where
    T: From<u8> + PartialEq,
{
    fn one() -> Self {
        T::from(1)
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }
}
