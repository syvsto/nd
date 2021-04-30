#[derive(Debug, Clone, PartialEq)]
pub enum A {
    F(Vec<f32>),
    C(Vec<char>),
}

impl A {
    pub fn len(&self) -> usize {
        match self {
            A::F(a) => a.len(),
            A::C(a) => a.len(),
        }
    }
}
