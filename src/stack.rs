use std::default::Default;

#[deriving(Default, Show)]
pub struct Stack {
    vec: Vec<i64>,
}

impl Stack {
    pub fn new() -> Stack {
        Default::default()
    }

    pub fn push(&mut self, var: i64) {
        self.vec.push(var);
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.vec.pop()
    }

    pub fn peek(&self) -> Option<i64> {
        self.vec.as_slice().last().and_then(|x| Some(*x))
    }
}
