use stack::Stack;
use ForthResult;

pub type WordFn = fn(&mut Stack) -> ForthResult;

pub struct Word {
    pub command: String,
    pub kind: WordKind,
}

pub enum WordKind {
    Builtin(WordFn),
    Words(Vec<String>),
}
