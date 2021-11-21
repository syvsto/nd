use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct A {
    pub rank: usize,
    pub d: Vec<Prim>,
}

impl A {
    pub(crate) fn new(rank: usize, d: Vec<Prim>) -> Self {
        Self { rank, d }
    }
    pub(crate) fn from_num(x: f64) -> Self {
        Self {
            rank: 1,
            d: vec![Prim::Num(x)],
        }
    }
    pub(crate) fn from_nums(x: &[f64]) -> Self {
        Self {
            rank: 1,
            d: x.iter().map(|x| Prim::Num(*x)).collect(),
        }
    }
    pub(crate) fn from_str(x: &str) -> Self {
        Self {
            rank: 1,
            d: x.chars().map(|x| Prim::Char(x)).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prim {
    Num(f64),
    Char(char),
}

impl Prim {
    pub fn and(&self, rhs: &Self) -> Option<Self> {
        use Prim::Num;

        match (self, rhs) {
            (Num(x), Num(y)) if *x == 1. && *y == 1. => Some(Num(1.)),
            _ => Some(Num(0.)),
        }
    }

    pub fn or(&self, rhs: &Self) -> Option<Self> {
        use Prim::Num;

        match (self, rhs) {
            (Num(x), Num(y)) if *x == 1. && *y == 1. => Some(Num(1.)),
            (Num(x), Num(y)) if *x == 1. && *y == 0. => Some(Num(1.)),
            (Num(x), Num(y)) if *x == 0. && *y == 1. => Some(Num(1.)),
            _ => Some(Num(0.)),
        }
    }

    pub fn eql(&self, rhs: &Self) -> Self {
        use Prim::{Char, Num};

        match (self, rhs) {
            (Num(x), Num(y)) if x == y => Num(1.),
            (Char(x), Char(y)) if x == y => Num(1.),
            _ => Num(0.),
        }
    }

    pub fn eq_type(&self, rhs: &Self) -> bool {
        use Prim::{Char, Num};

        match (self, rhs) {
            (Num(_), Num(_)) => true,
            (Char(_), Char(_)) => true,
            _ => false,
        }
    }

    pub fn is_num(&self) -> bool {
        if let Prim::Num(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Prim::Num(x) => Some(*x),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<char> {
        match self {
            Prim::Char(x) => Some(*x),
            _ => None,
        }
    }
}

impl<'a> Add<&'a Prim> for &'a Prim {
    type Output = Option<Prim>;

    fn add(self, rhs: Self) -> Self::Output {
        use Prim::{Char, Num};
        match (self, rhs) {
            (Num(a), Num(b)) => Some(Num(a + b)),
            (Char(a), Char(_)) => Some(Char(*a)),
            _ => None,
        }
    }
}

impl Add<f64> for &Prim {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        use Prim::Num;
        match (self, rhs) {
            (Num(a), b) => a + b,
            _ => 0.0,
        }
    }
}

impl<'a> Sub<&'a Prim> for &'a Prim {
    type Output = Option<Prim>;

    fn sub(self, rhs: Self) -> Self::Output {
        use Prim::{Char, Num};
        match (self, rhs) {
            (Num(a), Num(b)) => Some(Num(a - b)),
            (Char(a), Char(_)) => Some(Char(*a)),
            _ => None,
        }
    }
}

impl<'a> Mul<&'a Prim> for &'a Prim {
    type Output = Option<Prim>;

    fn mul(self, rhs: Self) -> Self::Output {
        use Prim::{Char, Num};
        match (self, rhs) {
            (Num(a), Num(b)) => Some(Num(a * b)),
            (Char(a), Char(_)) => Some(Char(*a)),
            _ => None,
        }
    }
}

impl<'a> Div<&'a Prim> for &'a Prim {
    type Output = Option<Prim>;

    fn div(self, rhs: Self) -> Self::Output {
        use Prim::{Char, Num};
        match (self, rhs) {
            (Num(a), Num(b)) => Some(Num(a / b)),
            (Char(a), Char(_)) => Some(Char(*a)),
            _ => None,
        }
    }
}
