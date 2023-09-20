use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct C<T, const N: usize> {
    pub d: [T; N],
}

impl<T: Default + Copy, const N: usize> C<T, N> {
    pub fn new() -> Self {
        Self { d: [T::default(); N] }
    }

    pub fn from(d: [T; N]) -> Self {
        Self { d }
    }
}

impl<T: Default + Copy + AddAssign + Add<Output=T>, const N: usize> Add for C<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut d: [T; N] = [T::default(); N];
        for i in 0..N {
            d[i] += self.d[i] + rhs.d[i];
        }
        Self::Output {
            d
        }
    }
}

impl<T: Default + Copy + SubAssign, const N: usize> Sub for C<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut d: [T; N] = self.d.clone();
        for i in 0..N {
            d[i] -= rhs.d[i];
        }
        Self::Output {
            d
        }
    }
}

impl<T: Default + Copy + MulAssign, const N: usize> Mul<T> for C<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut d: [T; N] = self.d.clone();
        for i in 0..N {
            d[i] *= rhs;
        }
        Self::Output {
            d
        }
    }
}

#[derive(Debug)]
pub struct VecBoard<T, const N: usize> {
    pub d: Vec<T>,
    pub dims: [i32; N],
    coeffs: [i32; N],
}

impl<T: Default + TryInto<usize>, const N: usize> VecBoard<T, N> {
    pub fn new(dims: [i32; N]) -> Self {
        let mut coeffs = [i32::default(); N];
        let mut prod = 1;
        for i in 0..N {
            coeffs[coeffs.len() - i - 1] = prod;
            prod *= dims[i]
        }
        let mut d = Vec::with_capacity(prod as usize);
        d.resize_with(prod as usize, || T::default());

        Self { d, dims, coeffs }
    }

    pub fn index_of(&self, c: C<i32, N>) -> i32 {
        let mut index = 0;
        for i in (0..N).rev() {
            index += self.coeffs[i] * c.d[i]
        }
        index
    }
}

impl<T, const N: usize> Index<usize> for VecBoard<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index]
    }
}

impl<T: Default + Copy, const N: usize, U> Index<C<T, N>> for VecBoard<U, N> {
    type Output = U;

    fn index(&self, index: C<T, N>) -> &Self::Output {
        // self.index_of(index);
        // self.d[self.index()]
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{C, VecBoard};

    #[test]
    fn test_new() {
        assert_eq!(C::from([0]),
                   C::<i32, 1>::new());
    }

    #[test]
    fn test_add() {
        assert_eq!(C::from([5, 5]),
                   C::from([1, 4]) + C::from([4, 1]));
        assert_eq!(C::from([5, 5, 5, 5]),
                   C::from([1, 2, 3, 4]) + C::from([4, 3, 2, 1]));
        assert_eq!(C::from([10]),
                   C::from([5]) + C::from([5]));
        assert_eq!(C::from([10.1]),
                   C::from([0.1]) + C::from([10.0]));
        assert_eq!(C::<i32, 0>::from([]),
                   C::<i32, 0>::from([]) + C::<i32, 0>::from([]));
    }

    #[test]
    fn test_sub() {
        assert_eq!(C::from([1, 4]),
                   C::from([5, 5]) - C::from([4, 1]));
        assert_eq!(C::from([1, 2, 3, 4]),
                   C::from([5, 5, 5, 5]) - C::from([4, 3, 2, 1]));
        assert_eq!(C::from([5]),
                   C::from([10]) - C::from([5]));
        assert_eq!(C::<i32, 0>::from([]),
                   C::<i32, 0>::from([]) - C::<i32, 0>::from([]));
    }

    #[test]
    fn test_mul() {
        assert_eq!(C::from([1, 1]),
                   C::from([1, 1]) * 1);
        assert_eq!(C::from([0, 0, 0, 0]),
                   C::from([1, 1, 9, i32::MAX]) * 0);
    }

    #[test]
    fn test_chain() {
        assert_eq!(C::from([100, -50]), C::from([1, 0]) * 100 + C::from([0, -1]) * 50);
    }

    #[test]
    fn test_vecboard_new() {
        {
            let b: VecBoard<i32, 2> = VecBoard::new([3, 2]);
            assert_eq!(b.dims.len(), 2, "dims not expected {b:?}");
            assert_eq!(b.d.len(), 6, "d not expected {b:?}");
            assert_eq!(b.d[b.d.len() - 1], 0, "default value in d not expected");
        }
        {
            let b: VecBoard<i32, 3> = VecBoard::new([4, 3, 2]);
            assert_eq!(b.dims.len(), 3, "dims not expected {b:?}");
            assert_eq!(b.d.len(), 24, "d not expected {b:?}");
            assert_eq!(b.d[b.d.len() - 1], 0, "default value in d not expected");
        }
        {
            let b: VecBoard<i32, 0> = VecBoard::new([]);
            assert_eq!(b.dims.len(), 0, "dims not expected {b:?}");
            assert_eq!(b.d.len(), 1, "d not expected {b:?}");
            assert_eq!(b.d[b.d.len() - 1], 0, "default value in d not expected");
        }
    }

    #[test]
    fn test_vecboard_index_of() {
        {
            let b: VecBoard<i32, 2> = VecBoard::new([3, 2]);
            assert_eq!(b.coeffs.len(), 2, "{b:?}");
            {
                let p = b.index_of(C::from([2, 1]));
                assert_eq!(p, 7, "{b:?} {p:?}");
            }
            {
                let p = b.index_of(C::from([1, 1]));
                assert_eq!(p, 4, "{b:?} {p:?}");
            }
        }
        {
            let b: VecBoard<i32, 3> = VecBoard::new([4, 3, 2]);
            assert_eq!(b.coeffs.len(), 3, "{b:?}");
            {
                let p = b.index_of(C::from([3, 2, 1]));
                assert_eq!(p, 45, "{b:?} {p:?}");
            }
            {
                let p = b.index_of(C::from([1, 1, 1]));
                assert_eq!(p, 17, "{b:?} {p:?}");
            }
        }
        {
            let b: VecBoard<i32, 0> = VecBoard::new([]);
            assert_eq!(b.d.len(), 1, "d not expected {b:?}");
            assert_eq!(b.d[b.d.len() - 1], 0, "default value in d not expected");
        }
    }
}


fn main() {
    let a: C<i32, 2> = C::new();
    println!("{a:?}");
}
