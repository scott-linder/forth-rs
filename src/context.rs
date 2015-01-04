use Result;
use error::Error::{UnknownWord, SyntaxError};
use stack::Stack;
use std::result::Result::{Ok, Err};
use word::Word;
use word::WordKind::{Builtin, Words};

#[deriving(Default)]
pub struct Context {
    stack: Stack,
    dict: Vec<Word>,
    compiling_word: Option<Word>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            stack: Stack::new(),
            dict: Vec::new(),
            compiling_word: None,
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.dict.push(word);
    }

    fn real_run_word(dict: &Vec<Word>, stack: &mut Stack,
                     command: &str) -> Result {
        match dict.iter().find(|word| word.command == command) {
            Some(word) => match word.kind {
                    Builtin(ref f) => try!((**f).call((stack,))),
                    Words(ref ws) => for w in ws.iter() {
                        try!(Context::real_run_word(dict, stack, w.as_slice()));
                    },
            },
            None => {
                let i = try!(command.parse().ok_or(UnknownWord));
                stack.push(i);
            },
        }
        Ok(())
    }

    pub fn run_word(&mut self, command: &str) -> Result {
        Context::real_run_word(&self.dict, &mut self.stack, command)
    }

    pub fn parse_line(&mut self, line: &str) -> Result {
        let mut tokens = line.trim_right_chars('\n').words();
        loop {
            let token = match tokens.next() {
                Some(t) => t,
                None => break,
            };
            match token {
                ":" => {
                    match self.compiling_word {
                        None => {
                            let command = try!(tokens.next().ok_or(SyntaxError));
                            self.compiling_word = Some(Word {
                                command: String::from_str(command),
                                kind: Words(Vec::new())
                            });
                        }
                        Some(..) => return Err(SyntaxError),
                    }
                },
                ";" => {
                    match self.compiling_word.take() {
                        Some(word) => {
                            self.dict.push(word);
                        },
                        None => return Err(SyntaxError),
                    }
                },
                word => {
                    match self.compiling_word {
                        Some(ref mut partial_word) => {
                            match partial_word.kind {
                                Words(ref mut ws) => ws.push(String::from_str(word)),
                                _ => panic!("compiling_word not Words kind"),
                            }
                        },
                        None => try!(self.run_word(word)),
                    }
                },
            }
        }
        debug!("stack={}", self.stack);
        Ok(())
    }
}
