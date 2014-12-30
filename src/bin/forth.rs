#![feature(phase, macro_rules)]
#[phase(link, plugin)] extern crate log;
extern crate forth;

use forth::error::Error::StackUnderflow;
use forth::stack::Stack;
use forth::context::Context;
use forth::word::Word;
use forth::word::WordKind::Builtin;
use std::io::{stdin, stderr};

macro_rules! builtin {
    ($context:ident : $command:expr $function:expr) => {
        $context.add_word(Word {
            command: $command.to_string(),
            kind: Builtin($function),
        });
    }
}
 
fn main() {
    let mut stderr = stderr();
    let mut context = Context::new();
    builtin!(context : "+" box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        let y = try!(s.pop().ok_or(StackUnderflow));
        s.push(y + x);
        Ok(())
    });
    builtin!(context : "-" box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        let y = try!(s.pop().ok_or(StackUnderflow));
        s.push(y - x);
        Ok(())
    });
    builtin!(context : "." box |&: s: &mut Stack| {
        let x = try!(s.pop().ok_or(StackUnderflow));
        println!("{}", x);
        Ok(())
    });
    builtin!(context : "DUP" box |&: s: &mut Stack| {
        let x = try!(s.peek().ok_or(StackUnderflow));
        s.push(x);
        Ok(())
    });
    for line in stdin().lock().lines() {
        match line {
            Ok(l) => match context.parse_line(l.as_slice()) {
                Ok(()) => match writeln!(&mut stderr, "ok") {
                    Ok(()) => (),
                    Err(e) => error!("{}", e),
                },
                Err(e) => error!("{}", e),
            },
            Err(e) => error!("{}", e),
        }
    }
}
