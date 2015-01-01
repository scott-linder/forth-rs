use stack::Stack;
use Result;

pub struct Word {
    pub command: String,
    pub kind: WordKind,
}

pub enum WordKind {
    Builtin(Box<Fn(&mut Stack) -> Result + 'static>),
    Words(Vec<String>),
}
