use stack::Stack;
use ForthResult;

pub struct Word {
    pub command: String,
    pub kind: WordKind,
}

pub enum WordKind {
    Builtin(Box<Fn(&mut Stack) -> ForthResult + 'static>),
    Words(Vec<String>),
}
