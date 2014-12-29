use ForthResult;
use error::Error::UnknownWord;
use stack::Stack;
use std::rc::Rc;
use std::result::Result::{Ok, Err};
use word::Word;
use word::WordKind::{Builtin, Words};

#[deriving(Default)]
pub struct State {
    stack: Stack,
    dict: Vec<Rc<Word>>,
}

impl State {
    pub fn new() -> State {
        State {
            stack: Stack::new(),
            dict: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.dict.push(Rc::new(word));
    }

    pub fn run_word(&mut self, command: &str) -> ForthResult {
        let mut next = None;
        for word in self.dict.iter() {
            if command == word.command {
                next = Some(word.clone());
                break;
            }
        }
        match next {
            Some(word) => {
                match word.kind {
                    Builtin(ref f) => try!((*f)(&mut self.stack)),
                    Words(ref ws) => for w in ws.iter() {
                        try!(self.run_word(w.as_slice()))
                    },
                }
            },
            None => {
                match command.parse() {
                    Some(i) => self.stack.push(i),
                    None => return Err(UnknownWord),
                }
            },
        }
        Ok(())
    }

    pub fn parse_line(&mut self, line: &str) -> ForthResult {
        for token in line.split(' ') {
            try!(self.run_word(token.trim_right_chars('\n')));
        }
        debug!("stack={}", self.stack);
        Ok(())
    }
}
