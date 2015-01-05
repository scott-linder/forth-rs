use Result;
use context::Context;
use std::rc::Rc;

pub enum Word {
    Builtin(Box<Fn(&mut Context) -> Result + 'static>),
    Literal(i64),
    Words(Vec<Rc<Word>>),
}
