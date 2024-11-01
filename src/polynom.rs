//! Module, where entire logic of polynomials is presented.

use std::{
    fmt::{Debug, Display},
    ops::{Add, BitXor, Mul, Neg, Sub},
};

use std::marker::PhantomData;

use super::{One, Zero};

/// One of polynomial variable.
///
/// This one represents letter 'x' in polynomial expressions.
///
/// T is type of polynomial coefficients.
/// It is not stored anywhere and used
/// for function `pow` to create
/// polynomial with correct type.
///
#[derive(Default, Debug, Clone, Copy)]
pub struct X<T: One>(PhantomData<T>);

impl<T: One> X<T> {
    /// Returns polynomial with only one argument.
    ///
    /// Example:
    /// ```rust
    /// # use polylib::polynom::X;
    /// let x = X::<i32>::default();
    /// x.pow(3);                // is polynomial(x^3)
    /// x.pow(2) + x.pow(5) * 3; // is polynomial(x^2 + 3x^5)
    /// ```
    pub fn pow(&self, power: u32) -> Polynomial<T, X<T>> {
        let mut ans = Polynomial::<T, X<T>>::new();
        ans.push(T::one(), Powered::<X<T>>::new(power));
        ans
    }
}

impl<T: One> BitXor<u32> for X<T> {
    type Output = Polynomial<T, X<T>>;

    fn bitxor(self, rhs: u32) -> Self::Output {
        let mut ans = Polynomial::<T, X<T>>::new();
        ans.push(T::one(), Powered::<X<T>>::new(rhs));
        ans
    }
}

impl<T: One> Display for X<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x")
    }
}

/// One of polynomial variable.
///
/// This one represents letter 'y' in polynomial expressions.
///
/// Same as at X, T is type of polynomial coefficients.
/// It is not stored anywhere and used
/// for function `pow` to create
/// polynomial with correct type.
///
/// Different letters are used for type safety of polynomials.
/// For example, you can't add x^2 to y^3
///
#[derive(Default, Debug, Clone, Copy)]
pub struct Y<T: One>(PhantomData<T>);

impl<T: One> Y<T> {
    /// Returns polynomial with only one argument.
    ///
    /// Example:
    /// ```rust
    /// # use polylib::polynom::X;
    /// # use polylib::polynom::Y;
    /// let y = Y::<i32>::default();
    /// y.pow(3);                // is polynomial(y^3)
    /// y.pow(2) + y.pow(5) * 3; // is polynomial(y^2 + 3y^5)
    /// ```
    ///
    /// ```compile_fail
    /// # use polylib::polynom::X;
    /// # use polylib::polynom::Y;
    /// # let y = Y::<i32>::default();
    /// let x = X::<i32>::default();
    /// y.pow(3) + x.pow(2);     // not allowed
    /// ```
    pub fn pow(&self, power: u32) -> Polynomial<T, Y<T>> {
        let mut ans = Polynomial::<T, Y<T>>::new();
        ans.push(T::one(), Powered::<Y<T>>::new(power));
        ans
    }
}

impl<T: One> BitXor<u32> for Y<T> {
    type Output = Polynomial<T, Y<T>>;

    fn bitxor(self, rhs: u32) -> Self::Output {
        let mut ans = Polynomial::<T, Y<T>>::new();
        ans.push(T::one(), Powered::<Y<T>>::new(rhs));
        ans
    }
}

impl<T: One> Display for Y<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y")
    }
}

// private structure represents polynomial variable T (wich is X<i32> for example)
// that is powered to power.
struct Powered<T> {
    power: u32,
    value: PhantomData<T>,
}

impl<T> Powered<T> {
    fn new(power: u32) -> Powered<T> {
        Powered::<T> {
            power,
            value: PhantomData,
        }
    }
    // returns value to the power of self.power
    fn substitude<U>(&self, value: U) -> U
    where
        U: One + Clone,
        U: Mul<U, Output = U>,
    {
        if self.power == 0 {
            return U::one();
        }
        let mut ans = U::one();
        let mut to_mul = value;
        let mut pow = self.power;

        while pow > 0 {
            if pow & 1 == 1 {
                ans = ans * to_mul.clone();
            }
            to_mul = to_mul.clone() * to_mul;
            pow >>= 1;
        }

        ans
    }
}

impl<T> Add for Powered<T> {
    type Output = Powered<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.power + rhs.power)
    }
}

impl<T> Default for Powered<T> {
    fn default() -> Self {
        Powered::<T> {
            power: 0,
            value: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for Powered<T> {
    fn clone(&self) -> Self {
        Self {
            power: self.power,
            value: std::marker::PhantomData,
        }
    }
}

impl<T> Debug for Powered<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Powered")
            .field("power", &self.power)
            .finish()
    }
}

impl<T> Display for Powered<T> 
where
    T: Default + Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.power == 0 {
            Result::Ok(())
        } else if self.power == 1 {
            write!(f, "{}", T::default())
        } else {
            write!(f, "{}^{}", T::default(), self.power)
        }
    }
}

/// Polynomial of one variable
///
/// # Type parameters
///  
/// #### `T`: type of coeffs. 
/// To use all features of polynomial it should
/// implement traits: `Clone`, `One`, `Zero`, and corresponded math operators.
/// Also it might implement trait `Display`, so it was possible to display polynomial.
///
/// #### `U`: type of variable
/// There is no special requirements to it since it is used to differenciate different polynomials.
/// But if you would like to use `Display` trait, `U` should implement `Defaul` and `Display`.
///
#[derive(Debug, Default)]
pub struct Polynomial<T, U = X<T>> {
    members: Vec<(T, Powered<U>)>,
}

impl<T, U> Polynomial<T, U> {
    fn new() -> Polynomial<T, U> {
        let v = Vec::<(T, Powered<U>)>::new();
        Polynomial::<T, U> { members: v }
    }
    fn push(&mut self, coef: T, var: Powered<U>) {
        self.members.push((coef, var));
    }

    /// Returns polynomial with coefs.
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// Polynomial::<i32>::from_coefs(vec![1, 0, 0, 1]); // is x^3 + 1
    /// ```
    pub fn from_coefs(coefs: Vec<T>) -> Polynomial<T, U>
    where
        T: Zero,
    {
        let mut v = Polynomial::<T, U>::new();
        let mut power = 0;
        for c in coefs {
            if c.is_zero() {
                power += 1;
                continue;
            }
            v.push(c, Powered::<U>::new(power));
            power += 1;
        }
        v
    }

    /// Returns const polynomial.
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// let p = Polynomial::<i32>::new_const(23); // is polynomial 23
    /// assert_eq!(p.substitude(1), 23);
    /// ```
    pub fn new_const(value: T) -> Polynomial<T, U> {
        let mut ans = Polynomial::<T, U>::new();
        ans.push(value, Powered::<U>::default());
        ans
    }

    /// Raises polynomial to power.
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(1) + 1; // p is x + 1
    /// p.pow(2);             // is (x + 1)^2 or x^2 + 2x + 1
    /// ```
    pub fn pow(self, power: u32) -> Polynomial<T, U>
    where
        T: Clone,
        T: Mul<T, Output = T>,
        T: One,
    {
        let powered = Powered::<U>::new(power);
        powered.substitude(self)
    }

    /// Calculate value of polynom at point 
    /// 
    /// Represent's polynomial like:
    /// 
    /// a0 + a1 * x + a2 * x^2 + ...
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(2) + 1;           // p is x^2 + 1
    /// assert_eq!(p.substitude(4), 17) // 4^2 + 1 = 17
    /// ```
    pub fn substitude<X, Y>(&self, point: X) -> Y
    where
        X: Clone + One,
        Y: Zero,
        T: Clone,
        X: Mul<X, Output = X>,
        Y: Add<Y, Output = Y>,
        T: Mul<X, Output = Y>,
    {
        let mut ans = Y::zero();
        for (coef, var) in self.members.iter() {
            let rhs = var.substitude(point.clone());
            ans = ans + coef.clone() * rhs;
        }
        ans
    }

    /// Same as substitude: calculate value of polynom at point.
    /// 
    /// But represent's polynomial like:
    /// 
    /// a0 + x * a1 + x^2 * a2 + ...
    /// 
    /// currently - the only way to substitude other polynomial
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(2) + 1;           // p is x^2 + 1
    /// assert_eq!(p.rsubstitude(4), 17) // 4^2 + 1 = 17
    /// ```
    pub fn rsubstitude<X, Y>(&self, value: X) -> Y
    where
        X: Clone + One,
        Y: Zero,
        T: Clone,
        X: Mul<X, Output = X>,
        Y: Add<Y, Output = Y>,
        X: Mul<T, Output = Y>,
    {
        let mut ans = Y::zero();
        for (coef, var) in self.members.iter() {
            let rhs = var.substitude(value.clone());
            ans = ans + rhs * coef.clone();
        }
        ans
    }

    /// Return polynomial in shortest form possible
    /// 
    /// For exmaple, we make this polynomial:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(2) * 2 + 1 -1 - x.pow(2) - x.pow(2) + 1; // 2x^2 + 1 - 1 - x^2 - x^2 + 1  
    /// ```
    /// By default, any + and - operations are just push value to the back of data vector,
    /// so the len of polynomial is huge:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// # let p = x.pow(2) * 2 + 1 -1 - x.pow(2) - x.pow(2) + 1;
    /// assert_eq!(p.len(), 6);
    /// ```
    /// But if we reduce the polynomial, we can see, that it is equal to 1.
    /// So we can use reduce to safe our memory and operations time.
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// # let p = x.pow(2) * 2 + 1 -1 - x.pow(2) - x.pow(2) + 1;
    /// # assert_eq!(p.len(), 6);
    /// let p = p.reduce();
    /// assert_eq!(p.len(), 1);
    /// ```
    pub fn reduce(mut self) -> Polynomial<T, U>
    where
        T: Clone + Zero,
        T: Add<T, Output = T>,
    {
        if self.members.is_empty() {
            return self;
        }
        self.members.sort_by_key(|(_, power)| power.power);
        let mut ans = Polynomial::new();
        let (mut coef, mut pow) = self.members[0].clone();
        for i in 1..self.members.len() {
            if self.members[i].1.power == pow.power {
                coef = coef + self.members[i].0.clone();
                continue;
            }
            if !coef.is_zero() {
                ans.push(coef, pow);
            }
            coef = self.members[i].0.clone();
            pow = self.members[i].1.clone();
        }
        if !coef.is_zero() {
            ans.push(coef, pow);
        }
        ans
    }

    /// Returns coeff of x^index
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(3) * 2 + 1;                   // is 2x^3 + 1
    /// assert_eq!(p.get(3).expect("").clone(), 2); // coef of x^3 is 2
    /// assert!(p.get(2).is_none());                // there is no x^2, so get(2) returns none
    /// ```
    pub fn get(&self, index: u32) -> Option<&T> {
        for memb in &self.members {
            if memb.1.power != index {
                continue;
            }
            return Some(&memb.0);
        }
        return None;
    }

    /// Returns len of data vector
    /// 
    /// Example:
    /// ```
    /// # use polylib::polynom::Polynomial;
    /// # use polylib::polynom::X;
    /// # let x = X::<i32>::default();
    /// let p = x.pow(3) * 2 + 1; // is 2x^3 + 1
    /// assert_eq!(p.len(), 2);   // len() is 2
    /// ```
    pub fn len(&self) -> usize {
        self.members.len()
    }
}

impl<T, U> Add for Polynomial<T, U> {
    type Output = Polynomial<T, U>;

    fn add(mut self, rhs: Self) -> Self::Output {
        for memb in rhs.members {
            self.push(memb.0, memb.1);
        }
        self
    }
}

impl<T, U> Add<T> for Polynomial<T, U> {
    type Output = Polynomial<T, U>;

    fn add(mut self, rhs: T) -> Self::Output {
        self.push(rhs, Powered::<U>::default());
        self
    }
}

impl<T, U> Neg for Polynomial<T, U>
where
    T: Neg<Output = T>,
{
    type Output = Polynomial<T, U>;

    fn neg(self) -> Self::Output {
        let mut ans = Self::Output::new();
        ans.members.reserve(self.members.len());
        for memb in self.members {
            ans.push(-memb.0, memb.1);
        }
        ans
    }
}

impl<A, T, U> Sub<A> for Polynomial<T, U>
where
    A: Neg<Output = T>,
{
    type Output = Polynomial<T, U>;

    fn sub(mut self, rhs: A) -> Self::Output {
        self.push(-rhs, Powered::<U>::default());
        self
    }
}

impl<T, U> Sub for Polynomial<T, U>
where
    T: Neg<Output = T>,
{
    type Output = Polynomial<T, U>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T, U> Mul<T> for Polynomial<T, U>
where
    T: Mul<T, Output = T>,
    T: Clone,
{
    type Output = Polynomial<T, U>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ans = Self::Output::new();
        ans.members.reserve(self.members.len());
        for memb in self.members {
            ans.push(memb.0 * rhs.clone(), memb.1);
        }
        ans
    }
}

impl<T, U> Mul for Polynomial<T, U>
where
    T: Clone,
    T: Mul,
{
    type Output = Polynomial<<T as Mul>::Output, U>;

    fn mul(self, rhs: Polynomial<T, U>) -> Self::Output {
        let mut ans = Self::Output::new();
        ans.members.reserve(self.members.len() * rhs.members.len());
        for memb1 in self.members {
            for memb2 in &rhs.members {
                ans.push(
                    memb1.0.clone() * memb2.0.clone(),
                    memb1.1.clone() + memb2.1.clone(),
                );
            }
        }
        ans
    }
}

impl<T, U> One for Polynomial<T, U>
where
    T: One,
{
    fn one() -> Self {
        Self::new_const(T::one())
    }

    fn is_one(&self) -> bool {
        panic!("is_one - hard operation for polynom");
    }
}

impl<T, U> Zero for Polynomial<T, U>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new_const(T::zero())
    }

    fn is_zero(&self) -> bool {
        panic!("is_zero - hard operation for polynom");
    }
}

impl<T, U> Clone for Polynomial<T, U>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
        }
    }
}

impl<T, U> Display for Polynomial<T, U>
where
    T: Display + Zero + One,
    U: Default + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for item in self.members.iter() {
            if item.0.is_zero() {
                continue;
            }
            if !first {
                write!(f, " + ")?;
            }
            first = false;
            if item.0.is_one() && item.1.power == 0 {
                write!(f, "{}", item.0)?;
                continue;
            }
            if item.0.is_one() {
                write!(f, "{}", item.1)?;
                continue;
            }
            write!(f, "{}{}", item.0, item.1)?;
        }
        if first {
            write!(f, "{}", T::zero())?;
        }
        std::fmt::Result::Ok(())
    }
}
