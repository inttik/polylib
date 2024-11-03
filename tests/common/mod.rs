use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

use polylib::polynom::Polynomial;
use polylib::{One, Zero};

#[allow(dead_code)]
pub fn same_coef<T, U>(poly: Polynomial<T, U>, coef: Vec<T>)
where
    T: Clone + Zero + PartialEq + Eq + Debug,
    T: Add<T, Output = T>,
    Polynomial<T, U>: Display,
{
    let poly = poly.reduce();

    let mut need_coef = 0;
    for i in 0..coef.len() {
        if coef[i].is_zero() {
            continue;
        }
        need_coef += 1;
        let val = poly.get(i as u32);
        match val {
            None => {
                panic!(
                    "poly '{}' is expected to have coef {} equal to {:?}",
                    poly, i, coef[i]
                )
            }
            Some(val) => {
                assert_eq!(val.clone(), coef[i], "poly '{}' has bad coef {}", poly, i)
            }
        }
    }
    if need_coef != poly.len() {
        panic!(
            "poly '{}' is expected to have {} non zero coefs",
            poly, need_coef
        )
    }
}

pub fn substitude_check<A, B, T, U>(poly: Polynomial<T, U>, x: Vec<A>, ans: Vec<B>)
where
    T: Clone,
    A: One + Clone + Debug,
    B: Zero + Clone + Eq + Debug,
    T: Mul<A, Output = B>,
    A: Mul<A, Output = A>,
    B: Add<B, Output = B>,
    Polynomial<T, U>: Display,
{
    assert_eq!(x.len(), ans.len(), "bad test: different sizes of x, ans");

    for i in 0..x.len() {
        let expect = ans[i].clone();
        let actual = poly.substitude(x[i].clone());
        assert_eq!(
            expect, actual,
            "incorrect substitution for polynom '{}'. x = {:?}. Expected = {:?}. Actual = {:?}",
            poly, x[i], expect, actual
        );
    }
}
