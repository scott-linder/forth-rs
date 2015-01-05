use Result;
use stack::Stack;
use std::rc::Rc;

pub enum Word {
    Builtin(Box<Fn(&mut Stack) -> Result + 'static>),
    Literal(i64),
    Words(Vec<Rc<Word>>),
}
