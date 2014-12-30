#![feature(phase, macro_rules)]
#[phase(link, plugin)] extern crate log;
extern crate forth;

use forth::error::Error::StackUnderflow;
use forth::stack::Stack;
use forth::state::State;
use forth::word::Word;
use forth::word::WordKind::Builtin;
use std::io::stdin;

macro_rules! builtin {
    ($state:ident : $command:expr $function:expr) => {
        $state.add_word(Word {
            command: $command.to_string(),
            kind: Builtin($function),
        });
    }
}
 
fn main() {
    let mut state = State::new();
    builtin!(state : "+" box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        let y = try!(s.pop().ok_or(StackUnderflow));
        s.push(y + x);
        Ok(())
    });
    builtin!(state : "-" box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        let y = try!(s.pop().ok_or(StackUnderflow));
        s.push(y - x);
        Ok(())
    });
    builtin!(state : "." box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        println!("{}", x);
        Ok(())
    });
    for line in stdin().lock().lines() {
        match line {
            Ok(l) => match state.parse_line(l.as_slice()) {
                Ok(()) => (),
                Err(e) => error!("{}", e),
            },
            Err(e) => error!("{}", e),
        }
    }
}
