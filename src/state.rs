use ForthResult;
use error::Error::UnknownWord;
use stack::Stack;
use std::result::Result::{Ok, Err};
use word::Word;
use word::WordKind::{Builtin, Words};

#[deriving(Default)]
pub struct State {
    stack: Stack,
    dict: Vec<Word>,
}

impl State {
    pub fn new() -> State {
        State {
            stack: Stack::new(),
            dict: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.dict.push(word);
    }

    fn real_run_word(dict: &Vec<Word>, stack: &mut Stack,
                     command: &str) -> ForthResult {
        for word in dict.iter() {
            if command == word.command {
                match word.kind {
                    Builtin(ref f) => try!((*f)(stack)),
                    Words(ref ws) => for w in ws.iter() {
                        try!(State::real_run_word(dict, stack, w.as_slice()));
                    },
                }
                return Ok(());
            }
        }
        let i = try!(command.parse().ok_or(UnknownWord));
        stack.push(i);
        Ok(())
    }

    pub fn run_word(&mut self, command: &str) -> ForthResult {
        State::real_run_word(&self.dict, &mut self.stack, command)
    }

    pub fn parse_line(&mut self, line: &str) -> ForthResult {
        for token in line.split(' ') {
            try!(self.run_word(token.trim_right_chars('\n')));
        }
        debug!("stack={}", self.stack);
        Ok(())
    }
}
