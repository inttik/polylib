//! Defines type `Matrix`.

use std::{
    cmp::min,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{One, Zero};


/// Type `Matrix`. N, M - sizes of matrix (N - count of rows).
/// T - type of element.
/// 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<const N: usize, const M: usize, T> {
    data: Vec<T>,
}

impl<const N: usize, const M: usize, T> Matrix<N, M, T> {

    /// Returns matrix<N, M>, where each element is value
    /// 
    /// Example:
    /// ```
    /// # use polylib::custom_types::Matrix;
    /// let m = Matrix::<2, 3, i32>::full(3); // is matrix [[3, 3, 3], [3, 3, 3]]
    /// assert_eq!(m.get_data(), &vec![3, 3, 3, 3, 3, 3]);
    /// ```
    pub fn full(value: T) -> Matrix<N, M, T>
    where
        T: Clone,
    {
        Matrix::<N, M, T> {
            data: vec![value; N * M],
        }
    }

    /// Returns matrix<N, M>, where each element on
    /// main diagonal is value
    /// 
    /// Example:
    /// ```
    /// # use polylib::custom_types::Matrix;
    /// let m = Matrix::<2, 3, i32>::eye(3); // is matrix [[3, 0, 0], [0, 3, 0]]
    /// assert_eq!(m.get_data(), &vec![3, 0, 0, 0, 3, 0]);
    /// ```
    pub fn eye(value: T) -> Matrix<N, M, T>
    where
        T: Clone + Zero,
    {
        let mut ans = Self::full(T::zero());
        for i in 0..min(N, M) {
            ans[(i, i)] = value.clone();
        }
        ans
    }

    /// Returns matrix<N, M>, where 
    /// elements are got from data
    /// 
    /// Example:
    /// ```
    /// # use polylib::custom_types::Matrix;
    /// let m = Matrix::<2, 3, i32>::from_data(vec![1, 2, 3, 4, 5, 6]); // is matrix [[1, 2, 3], [4, 5, 6]]
    /// assert_eq!(m.get_data(), &vec![1, 2, 3, 4, 5, 6]);
    /// ```
    pub fn from_data(data: Vec<T>) -> Matrix<N, M, T>
    {
        if data.len() != N * M {
            panic!("Can't create matrix<{}, {}> from {} elems", N, M, data.len())
        }
        Matrix::<N, M, T>{data}
    }

    /// Returns matrix<N, M> elements in 1d vector
    /// 
    pub fn get_data(&self) -> &Vec<T>
    {
        &self.data
    }

    /// Set matrix<N, M> elements from 1d vector
    /// 
    pub fn set_data(&mut self, data: Vec<T>) {
        if data.len() != N * M {
            panic!("Can't set data to matrix<{}, {}> while data has {} elems", N, M, data.len())
        }
        self.data = data
    }
}

impl<const N: usize, T> One for Matrix<N, N, T> 
where
    T: Zero + One + Clone + PartialEq,
{
    fn one() -> Self {
        Self::eye(T::one())
    }

    fn is_one(&self) -> bool {
        self.data == Self::one().data
    }
}

impl<const N: usize, const M: usize, T> Zero for Matrix<N, M, T>
where
    T: Zero + Clone + PartialEq,
{
    fn zero() -> Self {
        Self::full(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.data == Self::zero().data
    }
}


impl<const N: usize, const M: usize, T> Index<(usize, usize)> for Matrix<N, M, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= N {
            panic!("try to get [{}, {}] from matrix<{}, {}>", index.0, index.1, N, M)
        }
        if index.1 >= M {
            panic!("try to get [{}, {}] from matrix<{}, {}>", index.0, index.1, N, M)
        }
        &self.data[index.0 * M + index.1]
    }
}

impl<const N: usize, const M: usize, T> IndexMut<(usize, usize)> for Matrix<N, M, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= N {
            panic!("try to set [{}, {}] to matrix<{}, {}>", index.0, index.1, N, M)
        }
        if index.1 >= M {
            panic!("try to set [{}, {}] to matrix<{}, {}>", index.0, index.1, N, M)
        }
        &mut self.data[index.0 * M + index.1]
    }
}

impl<const N: usize, const M: usize, T> AddAssign for Matrix<N, M, T>
where
    T: AddAssign<T> + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self[(i, j)] += rhs[(i, j)].clone()
            }
        }
    }
}

impl<const N: usize, const M: usize, T> Add for Matrix<N, M, T>
where
    T: AddAssign<T> + Clone,
{
    type Output = Matrix<N, M, T>;

    fn add(mut self, rhs: Matrix<N, M, T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize, const M: usize, T> Neg for Matrix<N, M, T>
where
    T: Neg<Output = T> + Clone,
{
    type Output = Matrix<N, M, T>;

    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            for j in 0..M {
                self[(i, j)] = -self[(i, j)].clone();
            }
        }
        self
    }
}

impl<const N: usize, const M: usize, T> SubAssign for Matrix<N, M, T>
where
    T: SubAssign<T> + Clone,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            for j in 0..M {
                self[(i, j)] -= rhs[(i, j)].clone();
            }
        }
    }
}

impl<const N: usize, const M: usize, T> Sub for Matrix<N, M, T>
where
    T: SubAssign<T> + Clone,
{
    type Output = Matrix<N, M, T>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize, const K: usize, const M: usize, T> Mul<Matrix<K, M, T>> for Matrix<N, K, T>
where
    T: Clone + Zero,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    type Output = Matrix<N, M, T>;

    fn mul(self, rhs: Matrix<K, M, T>) -> Self::Output {
        let mut ans = Self::Output::full(T::zero());

        for n in 0..N {
            for k in 0..K {
                for m in 0..M {
                    ans[(n, m)] += self[(n, k)].clone() * rhs[(k, m)].clone();
                }
            }
        }
        ans
    }
}

impl<const N: usize, T> MulAssign for Matrix<N, N, T>
where
    T: Clone + Zero,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.data = (self.clone() * rhs).data;
    }
}

impl<const N: usize, const M: usize, T, A> MulAssign<A> for Matrix<N, M, T>
where
    A: From<u8> + Clone,
    T: MulAssign<A>,
{
    fn mul_assign(&mut self, rhs: A) {
        for i in 0..N {
            for j in 0..M {
                self[(i, j)] *= rhs.clone();
            }
        }
    }
}

impl<const N: usize, const M: usize, T, A> Mul<A> for Matrix<N, M, T>
where
    A: From<u8> + Clone,
    T: MulAssign<A>,
{
    type Output = Matrix<N, M, T>;

    fn mul(mut self, rhs: A) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const N: usize, const M: usize, T> Mul<Matrix<N, M, T>> for i32
where
    T: MulAssign<i32>,
{
    type Output = Matrix<N, M, T>;

    fn mul(self, mut rhs: Matrix<N, M, T>) -> Self::Output {
        rhs *= self;
        rhs
    }
}

#[cfg(test)]
mod matrix_test {
    use std::panic;

    use crate::{One, Zero};

    use super::Matrix;

    type M12 = Matrix<1, 2, i32>;
    type M21 = Matrix<2, 1, i32>;
    type M22 = Matrix<2, 2, i32>;

    #[test]
    fn test_full() {
        let m = M22::full(12);
        assert_eq!(m.data, vec![12, 12, 12, 12]);

        let m = M12::full(-2);
        assert_eq!(m.data, vec![-2, -2]);

        let m = M21::full(0);
        assert_eq!(m.data, vec![0, 0]);
    } 

    #[test]
    fn test_eye() {
        let m = M22::eye(1);
        assert_eq!(m.data, vec![1, 0, 0, 1]);

        let m = M12::eye(-1);
        assert_eq!(m.data, vec![-1, 0]);

        let m = M21::eye(0);
        assert_eq!(m.data, vec![0, 0]);

        let m = Matrix::<2, 3, i32>::eye(-2);
        assert_eq!(m.data, vec![-2, 0, 0, 0, -2, 0]);

        let m = Matrix::<3, 2, i32>::eye(-2);
        assert_eq!(m.data, vec![-2, 0, 0, -2, 0, 0]) 
    }

    #[test]
    fn test_from_data() {
        let m = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!(m.data, vec![1, 2, 3, 4]);

        let m = M12::from_data(vec![1, 2]);
        assert_eq!(m.data, vec![1, 2]);

        let m = M21::from_data(vec![1, 2]);
        assert_eq!(m.data, vec![1, 2]);

        panic::catch_unwind(|| {
            let _ = M22::from_data(vec![1, 2]);
        }).expect_err("incorrect size of m");

        panic::catch_unwind(|| {
            let _ = M22::from_data(vec![1, 2, 3, 4, 5]);
        }).expect_err("incorrect size of m");


        panic::catch_unwind(|| {
            let _ = M12::from_data(vec![1, 2, 3, 4]);
        }).expect_err("incorrect size of m");
    }

    #[test]
    fn test_get_data() {
        let m = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!(m.get_data(), &vec![1, 2, 3, 4]);

        let m = M12::from_data(vec![1, 2]);
        assert_eq!(m.get_data(), &vec![1, 2]);

        let m = M21::from_data(vec![1, 2]);
        assert_eq!(m.get_data(), &vec![1, 2]);
    }

    #[test]
    fn test_set_data() {
        let mut m = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!(m.get_data(), &vec![1, 2, 3, 4]);
        m.set_data(vec![4, 3, 2, 1]);
        assert_eq!(m.get_data(), &vec![4, 3, 2, 1]);

        let mut m = M12::from_data(vec![1, 2]);
        assert_eq!(m.get_data(), &vec![1, 2]);
        m.set_data(vec![2, 1]);
        assert_eq!(m.get_data(), &vec![2, 1]);

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m.set_data(vec![1, 2]);
        }).expect_err("incorrect size of set");

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m.set_data(vec![1, 2, 3, 4, 5]);
        }).expect_err("incorrect size of set");

        panic::catch_unwind(|| {
            let mut m = M12::from_data(vec![1, 2]);
            m.set_data(vec![1, 2, 3, 4]);
        }).expect_err("incorrect size of set");
    }

    #[test]
    fn test_get_index () {
        let m = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 1)], 2);
        assert_eq!(m[(1, 0)], 3);
        assert_eq!(m[(1, 1)], 4);

        panic::catch_unwind(|| {
            let m = M22::from_data(vec![1, 2, 3, 4]);
            let _ = m[(2, 0)];
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let t = M22::from_data(vec![1, 2, 3, 4]);
            t[(3, 0)]
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let m = M22::from_data(vec![1, 2, 3, 4]);
            m[(0, 2)]
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let m = M22::from_data(vec![1, 2, 3, 4]);
            m[(0, 3)]
        }).expect_err("index out of bounds");

        let m = M12::from_data(vec![1, 2]);
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 1)], 2);

        let m = M21::from_data(vec![1, 2]);
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(1, 0)], 2);
    }

    #[test]
    fn test_set_index() {
        let mut m = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!(m.data, vec![1, 2, 3, 4]);
        m[(0, 0)] = 5;
        assert_eq!(m.data, vec![5, 2, 3, 4]);
        m[(1, 0)] = 6;
        assert_eq!(m.data, vec![5, 2, 6, 4]);
        m[(1, 1)] = 7;
        assert_eq!(m.data, vec![5, 2, 6, 7]);
        m[(0, 1)] = 8;
        assert_eq!(m.data, vec![5, 8, 6, 7]);

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m[(2, 0)] = 5;
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m[(3, 0)] = 5;
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m[(0, 2)] = 5;
        }).expect_err("index out of bounds");

        panic::catch_unwind(|| {
            let mut m = M22::from_data(vec![1, 2, 3, 4]);
            m[(0, 3)] = 5;
        }).expect_err("index out of bounds");

        let mut m = M12::from_data(vec![1, 2]);
        assert_eq!(m.data, vec![1, 2]);
        m[(0, 1)] = 3;
        assert_eq!(m.data, vec![1, 3]);
        m[(0, 0)] = 4;
        assert_eq!(m.data, vec![4, 3]);

        let mut m = M21::from_data(vec![1, 2]);
        assert_eq!(m.data, vec![1, 2]);
        m[(1, 0)] = 3;
        assert_eq!(m.data, vec![1, 3]);
        m[(0, 0)] = 4;
        assert_eq!(m.data, vec![4, 3]);
    }
    
    #[test]
    fn test_add() {
        let a = M22::from_data(vec![1, 2, 3, 4]);
        let b = M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!((a + b).data, vec![4, -3, 5, 4]);

        let mut a = M22::from_data(vec![1, 2, 3, 4]);
        a += M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!(a.data, vec![4, -3, 5, 4]);

        let a = M12::from_data(vec![1, 2]);
        let b = M12::from_data(vec![3, -2]);
        assert_eq!((a + b).data, vec![4, 0]);

        let mut a = M12::from_data(vec![1, 2]);
        a += M12::from_data(vec![3, -2]);
        assert_eq!(a.data, vec![4, 0]);

        let a = M21::from_data(vec![1, 2]);
        let b = M21::from_data(vec![3, -2]);
        assert_eq!((a + b).data, vec![4, 0]);

        let mut a = M21::from_data(vec![1, 2]);
        a += M21::from_data(vec![-1, 2]);
        assert_eq!(a.data, vec![0, 4]);
    }

    #[test]
    fn test_neg() {
        let a = M22::from_data(vec![1, 2, 3, 0]);
        assert_eq!((-a).data, vec![-1, -2, -3, 0]);

        let a = M12::from_data(vec![0, -1]);
        assert_eq!((-a).data, vec![0, 1]);
    }

    #[test]
    fn test_sub() {
        let a = M22::from_data(vec![1, 2, 3, 4]);
        let b = M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!((a - b).data, vec![-2, 7, 1, 4]);

        let mut a = M22::from_data(vec![1, 2, 3, 4]);
        a -= M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!(a.data, vec![-2, 7, 1, 4]);

        let a = M12::from_data(vec![1, 2]);
        let b = M12::from_data(vec![3, -2]);
        assert_eq!((a - b).data, vec![-2, 4]);

        let mut a = M12::from_data(vec![1, 2]);
        a -= M12::from_data(vec![3, -2]);
        assert_eq!(a.data, vec![-2, 4]);

        let a = M21::from_data(vec![1, 2]);
        let b = M21::from_data(vec![3, -2]);
        assert_eq!((a - b).data, vec![-2, 4]);

        let mut a = M21::from_data(vec![1, 2]);
        a -= M21::from_data(vec![-1, 2]);
        assert_eq!(a.data, vec![2, 0]);
    }

    #[test]
    fn test_mul() {
        let a = M22::from_data(vec![1, 2, 3, 4]);
        let b = M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!((a * b).data, vec![7, -5, 17, -15]);

        let mut a = M22::from_data(vec![1, 2, 3, 4]);
        a *= M22::from_data(vec![3, -5, 2, 0]);
        assert_eq!(a.data, vec![7, -5, 17, -15]);

        let a = M12::from_data(vec![1, 2]);
        let b = M21::from_data(vec![3, 4]);

        let c = a.clone() * b.clone();
        assert_eq!(c.data, vec![11]);

        let c = b.clone() * a.clone();
        assert_eq!(c.data, vec![3, 6, 4, 8]);
    }

    #[test]
    fn test_scalar_mul() {
        let a = M22::from_data(vec![1, 2, 3, 4]);
        assert_eq!((a * -3).data, vec![-3, -6, -9, -12]);

        let mut a = M22::from_data(vec![1, 2, 3, 4]);
        a *= -3;
        assert_eq!(a.data, vec![-3, -6, -9, -12]);

        let a = M12::from_data(vec![1, 2]);
        assert_eq!((a * -3).data, vec![-3, -6]);

        let mut a = M12::from_data(vec![1, 2]);
        a *= -3;
        assert_eq!(a.data, vec![-3, -6]);

        let a = M21::from_data(vec![1, 2]);
        assert_eq!((a * -3).data, vec![-3, -6]);

        let mut a = M21::from_data(vec![1, 2]);
        a *= -3;
        assert_eq!(a.data, vec![-3, -6]);
    }

    #[test]
    fn test_zero() {
        let a = M22::zero();
        assert_eq!(a.data, vec![0; 4]);

        let a = Matrix::<100, 200, i32>::zero();
        assert_eq!(a.data, vec![0; 20_000]);
    }

    #[test]
    fn test_one() {
        let a = M22::one();
        assert_eq!(a.data, vec![1, 0, 0, 1]);

        let a = Matrix::<1, 1, i32>::one();
        assert_eq!(a.data, vec![1]);
    }
}
