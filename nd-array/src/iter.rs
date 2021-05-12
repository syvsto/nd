use std::iter::{Iterator, FromIterator};
use crate::{Array, A1, A2, A3};

pub struct Iter<I> {
    iter: I,
    dims: Vec<usize>,
}

impl<I> Iter<I> {
    pub fn new(iter: I, dims: &[usize]) -> Self {
        Self {
            iter, dims: dims.to_vec()
        }
    }
}

impl<I: Iterator> Iterator for Iter<I> 
    where <I as Iterator>::Item: Clone {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn collect<T>(self) -> T
    where T: FromIterator<Self::Item> {
        self.iter.collect()
    }
}
