use std::ops::Index;
use std::marker::Sized;

pub enum ATy {
    F32,
    C,
}

impl ATy {
    fn size(&self) -> usize {
        match self {
            F32 => 4,
            C => 4,
        }
    }
}

pub enum R {
    R0,
    R1(usize),
    R2([usize; 2]),
    R3([usize; 3]),
}

/// Row-major n-dimensional array
pub struct A<T>
  where T: Sized {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> A<T>
  where T: Sized {
    fn new(shape: &[usize], data: &[T]) -> Self {
        Self {
            shape: shape.to_vec(), data: data.to_vec()
        }
    }
}

impl<T: Sized> Index<R> for A<T> {
    type Output = usize;

    fn index(&self, idx: R) -> &Self::Output {
        use R::*;

        match idx {
            R0 => 0,
            R1(x) => x,
            R2([x, y]) => (self.shape[0] * x + y),
            R3([x, y, z]) => ((self.shape[0] * self.shape[1]) * z + (self.shape[0] * x + y)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        use R::*;
        let a = A::new();
        let ix0 = R0;
        let ix1 = R1(2);
        let ix2 = R2([2, 3]);
        let ix4 = R3([2, 3, 1]);
        assert!
    }
}
