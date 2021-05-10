use std::ops::Index;
use std::marker::Sized;

pub enum R {
    R0,
    R1(usize),
    R2([usize; 2]),
    R3([usize; 3]),
}

/// Row-major n-dimensional array
#[derive(Debug)]
pub struct A<T>
  where T: Sized + Clone {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> A<T>
  where T: Sized + Clone {
    pub fn new(shape: &[usize], data: &[T]) -> Self {
        Self {
            shape: shape.to_vec(), data: data.to_vec()
        }
    }
}

impl<T: Sized + Clone> Index<R> for A<T> {
    type Output = T;

    fn index(&self, idx: R) -> &Self::Output {
        use R::*;

        let i = match idx {
            R0 => 0,
            R1(x) => x,
            R2([x, y]) => (self.shape[0] * y + x),
            R3([x, y, z]) => ((self.shape[0] * self.shape[1]) * z + (self.shape[0] * y + x)),
        };

        &self.data[i]
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
        A::new(&[*x, *y, *z], &c.as_ref())
    }};
    ([$([$($e:expr),*]),+]) => {{
        let v = vec![$(vec![$($e,)*],)*];
        let x = &v[0].len();
        let y = &v.len();
        let c: Vec<_> = v.into_iter().flatten().collect();
        A::new(&[*x, *y], &c.as_ref())
    }};
    ([$($e:expr),*]) => {{
        let v = vec![$($e,)*];
        A::new(&[v.len()], &v.as_ref())
    }};
    ($e:expr) => {{
      A::new(&[], &[$e])   
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        use R::*;
        let a = a!(1);
        let b = a!([1, 2]);
        let c = a!([[1, 2], [3, 4]]);
        let d = a!([[[1, 2, 3], [3, 4, 5]], [[5, 6, 7], [7, 8, 9]]]);

        let ix0 = R0;
        let ix1 = R1(1);
        let ix2 = R2([1, 0]);
        let ix4 = R3([2, 1, 0]);
        assert!(a[ix0] == 1);
        assert!(b[ix1] == 2);
        assert!(c[ix2] == 2);
        assert!(d[ix4] == 5);
    }
}
