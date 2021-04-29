use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ty {
    F,
    C,
}

impl Ty {
    pub fn size(&self) -> usize {
        use Ty::*;
        match self {
            F => 4,
            C => 4,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct A {
    pub ty: Ty,
    pub r: usize,
    pub l: usize,
    pub s: Vec<usize>,
    pub c: Vec<u8>,
}

impl A {
    pub fn new(ty: Ty, r: usize, l: usize, s: Vec<usize>, c: Vec<u8>) -> Self {
        Self { ty, r, l, s, c }
    }

    pub fn new_f(n: f32) -> Self {
        let v = &vec![n];
        Self::new(Ty::F, 0, 1, vec![1], vf_to_u8(v).to_vec())
    }
}

pub fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn u8_to_vf(v: &[u8]) -> &[f32] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const f32, v.len() / 4) }
}

pub fn u8_f(v: &[u8]) -> f32 {
    f32::from_le_bytes(v.try_into().expect("Couldn't cast from u8 slice to f32"))
}
