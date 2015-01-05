use Result;
use dict::Dict;
use error::Error::{UnknownWord, SyntaxError};
use stack::Stack;
use std::borrow::ToOwned;
use std::default::Default;
use std::rc::Rc;
use std::result::Result::{Ok, Err};
use word::Word::{Builtin, Literal, Words};
use word::Word;

#[deriving(Default)]
pub struct Context {
    stack: Stack,
    dict: Dict,
    compiling: Option<(String, Vec<Rc<Word>>)>,
}

impl Context {
    pub fn new() -> Context {
        Default::default()
    }

    pub fn from_dict(dict: Dict) -> Context {
        Context { dict: dict, ..Default::default() }
    }

    fn run_word(&mut self, word: Rc<Word>) -> Result {
        match *word {
            Builtin(ref f) => try!((**f).call((&mut self.stack,))),
            Words(ref ws) => for w in ws.iter() {
                try!(self.run_word(w.clone()));
            },
            Literal(i) => self.stack.push(i),
        }
        Ok(())
    }

    pub fn parse_line(&mut self, line: &str) -> Result {
        let mut tokens = line.trim_right_matches('\n').words();
        loop {
            let token = match tokens.next() {
                Some(t) => t,
                None => break,
            };
            match token {
                ":" => {
                    match self.compiling {
                        None => {
                            let name = try!(tokens.next().ok_or(SyntaxError));
                            self.compiling = Some((name.to_owned(), Vec::new()));
                        },
                        Some(..) => return Err(SyntaxError),
                    }
                },
                ";" => {
                    match self.compiling.take() {
                        Some((name, words)) =>
                            self.dict.push_word(name, Words(words)),
                        None => return Err(SyntaxError),
                    }
                },
                name => {
                    let word = match self.dict.find_word(name) {
                        Some(w) => w,
                        None => {
                            let i = try!(name.parse().ok_or(UnknownWord));
                            Rc::new(Literal(i))
                        },
                    };
                    match self.compiling {
                        Some((_, ref mut words)) => words.push(word),
                        None => try!(self.run_word(word)),
                    }
                },
            }
        }
        debug!("stack={}", self.stack);
        Ok(())
    }
}

