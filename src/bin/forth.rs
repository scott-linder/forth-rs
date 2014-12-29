#![feature(phase, macro_rules)]
#[phase(link, plugin)] extern crate log;
extern crate forth;

use forth::ForthResult;
use forth::error::Error::StackUnderflow;
use forth::stack::Stack;
use forth::state::State;
use forth::word::Word;
use forth::word::WordKind::Builtin;
use std::io::stdin;

fn add(s: &mut Stack) -> ForthResult {
    let x = try!(s.pop().ok_or(StackUnderflow));
    let y = try!(s.pop().ok_or(StackUnderflow));
    s.push(y + x);
    Ok(())
}

fn sub(s: &mut Stack) -> ForthResult {
    let x = try!(s.pop().ok_or(StackUnderflow));
    let y = try!(s.pop().ok_or(StackUnderflow));
    s.push(y - x);
    Ok(())
}

fn dot(s: &mut Stack) -> ForthResult {
    let x = try!(s.pop().ok_or(StackUnderflow));
    println!("{}", x);
    Ok(())
}

macro_rules! builtin {
    ($state:ident : $command:expr $function:ident) => {
        $state.add_word(Word {
            command: $command.to_string(),
            kind: Builtin($function),
        });
    }
}
 
fn main() {
    let mut state = State::new();
    builtin!(state : "+" add);
    builtin!(state : "-" sub);
    builtin!(state : "." dot);
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
