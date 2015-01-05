#![feature(phase, macro_rules)]
#[phase(link, plugin)] extern crate log;
extern crate forth;

use forth::context::Context;
use forth::dict::Dict;
use forth::error::Error::StackUnderflow;
use forth::stack::Stack;
use forth::word::Word::Builtin;
use std::borrow::ToOwned;
use std::io::stdin;

fn main() {
    let mut dict = Dict::new();
    dict.push_word("+".to_owned(), Builtin(box |&: s: &mut Stack| {
        let n1 = try!(s.pop().ok_or(StackUnderflow));
        let n2 = try!(s.pop().ok_or(StackUnderflow));
        s.push(n2 + n1);
        Ok(())
    }));
    dict.push_word("-".to_owned(), Builtin(box |&: s: &mut Stack| {
        let n1 = try!(s.pop().ok_or(StackUnderflow));
        let n2 = try!(s.pop().ok_or(StackUnderflow));
        s.push(n2 - n1);
        Ok(())
    }));
    dict.push_word(".".to_owned(), Builtin(box |&: s: &mut Stack| {
        let n = try!(s.pop().ok_or(StackUnderflow));
        print!("{}", n);
        Ok(())
    }));
    dict.push_word("DUP".to_owned(), Builtin(box |&: s: &mut Stack| {
        let n = try!(s.peek().ok_or(StackUnderflow));
        s.push(n);
        Ok(())
    }));
    let mut context = Context::from_dict(dict);
    for line in stdin().lock().lines() {
        match line {
            Ok(l) => match context.parse_line(l.as_slice()) {
                Ok(()) => print!(" ok "),
                Err(e) => error!("{}", e),
            },
            Err(e) => error!("{}", e),
        }
    }
}
