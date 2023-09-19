use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

pub struct VecBoard<T>{
    pub d: Vec<T>
}

#[cfg(test)]
mod tests {
    use crate::C;

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
}


fn main() {
    let a: C<i32, 2> = C::new();
    println!("{a:?} {a:?}");
}
