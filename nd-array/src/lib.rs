use std::ops::Index;
use std::marker::Sized;
use std::slice;

mod iter;

use iter::Iter;

pub trait Array {}

impl<T: Clone + Sized> Array for A3<T> {}
impl<T: Clone + Sized> Array for A2<T> {}
impl<T: Clone + Sized> Array for A1<T> {}

/// Row-major 3-dimensional array
#[derive(Debug)]
pub struct A3<T>
  where T: Sized + Clone {
    shape: [usize; 3],
    data: Vec<T>,
}
impl<T> A3<T>
  where T: Sized + Clone {
    pub fn new(shape: [usize; 3], data: &[T]) -> Self {
        Self {
            shape, data: data.to_vec()
        }
    }

    pub fn iter(&self) -> Iter<slice::Iter<'_, T>> {
        Iter::new(self.data.iter(), &self.shape)
    }
}

impl<T: Sized + Clone> Index<[usize; 3]> for A3<T> {
    type Output = T;

    fn index(&self, idx: [usize; 3]) -> &Self::Output {
        &self.data[self.shape[1] * self.shape[0] * idx[2] + self.shape[0] * idx[1] + idx[0]]
    }
}

/// Row-major 2-dimensional array
#[derive(Debug)]
pub struct A2<T>
  where T: Sized + Clone {
    shape: [usize; 2],
    data: Vec<T>,
}
impl<T> A2<T>
  where T: Sized + Clone {
    pub fn new(shape: [usize; 2], data: &[T]) -> Self {
        Self {
            shape, data: data.to_vec()
        }
    }

    pub fn iter(&self) -> Iter<slice::Iter<'_, T>> {
        Iter::new(self.data.iter(), &self.shape)
    }
}
impl<T: Sized + Clone> Index<[usize; 2]> for A2<T> {
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.data[self.shape[0] * idx[1] + idx[0]]
    }
}

/// Row-major 1-dimensional array
#[derive(Debug)]
pub struct A1<T>
  where T: Sized + Clone {
    shape: [usize; 1],
    data: Vec<T>,
}
impl<T> A1<T>
  where T: Sized + Clone {
    pub fn new(shape: usize, data: &[T]) -> Self {
        Self {
            shape: [shape], data: data.to_vec()
        }
    }

    pub fn iter(&self) -> Iter<slice::Iter<'_, T>> {
        Iter::new(self.data.iter(), &self.shape)
    }
}

impl<T: Sized + Clone> Index<usize> for A1<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

#[macro_export]
macro_rules! a {
    ([$([$([$($e:expr),*]),+]),+]) => {{
        let v = vec![$(vec![$(vec![$($e,)*],)*],)*];
        let x = &v[0][0].len();
        let y = &v[0].len();
        let z = &v.len();
        let c: Vec<_> = v.into_iter().flatten().flatten().collect();
        A3::new([*x, *y, *z], &c.as_ref())
    }};
    ([$([$($e:expr),*]),+]) => {{
        let v = vec![$(vec![$($e,)*],)*];
        let x = &v[0].len();
        let y = &v.len();
        let c: Vec<_> = v.into_iter().flatten().collect();
        A2::new([*x, *y], &c.as_ref())
    }};
    ([$($e:expr),*]) => {{
        let v = vec![$($e,)*];
        A1::new(v.len(), &v.as_ref())
    }};
    ($e:expr) => {{
      A0::new(&[$e])   
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        let a = a!([1, 2]);
        let b = a!([[1, 2], [3, 4]]);
        let c = a!([[[1, 2, 3], [3, 4, 5]], [[5, 6, 7], [7, 8, 9]]]);

        let ix0 = 1;
        let ix1 = [1, 0];
        let ix2 = [2, 1, 0];
        assert!(a[ix0] == 2);
        assert!(b[ix1] == 2);
        assert!(c[ix2] == 5);
    }

    #[test]
    fn iter() {
        let a = a!([1, 2]);
        let b = a!([[1, 2], [3, 4]]);
        let c = a!([[[1, 2, 3], [3, 4, 5]], [[5, 6, 7], [7, 8, 9]]]);
        let r_a = vec![2, 3];
        let r_b = vec![2, 3, 4, 5];
        let r_c = vec![2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10];

        assert!(r_a == a.iter().map(|x| x + 1).collect::<Vec<_>>());
        assert!(r_b == b.iter().map(|x| x + 1).collect::<Vec<_>>());
        assert!(r_c == c.iter().map(|x| x + 1).collect::<Vec<_>>());
    }
}
