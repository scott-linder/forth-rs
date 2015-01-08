#[macro_use] extern crate log;
extern crate forth;

use forth::context::Context;
use forth::dict::Dict;
use forth::error::Error::StackUnderflow;
use forth::word::Word::Builtin;
use std::borrow::ToOwned;
use std::io::stdin;

macro_rules! push_builtin {
    ($dict:ident : $name:expr $func:expr) => {
        $dict.push_word($name.to_owned(), Builtin($func));
    }
}

fn main() {
    let mut dict = Dict::new();
    push_builtin!(dict : "." box |context| {
        let n = try!(context.stack.pop().ok_or(StackUnderflow));
        print!("{}", n);
        Ok(())
    });
    push_builtin!(dict : "DUP" box |context| {
        let n = try!(context.stack.peek().ok_or(StackUnderflow));
        context.stack.push(n);
        Ok(())
    });
    push_builtin!(dict : "+" box |context| {
        let n1 = try!(context.stack.pop().ok_or(StackUnderflow));
        let n2 = try!(context.stack.pop().ok_or(StackUnderflow));
        context.stack.push(n2 + n1);
        Ok(())
    });
    push_builtin!(dict : "-" box |context| {
        let n1 = try!(context.stack.pop().ok_or(StackUnderflow));
        let n2 = try!(context.stack.pop().ok_or(StackUnderflow));
        context.stack.push(n2 - n1);
        Ok(())
    });
    push_builtin!(dict : ".s" box |context| {
        print!("{}", context.stack.vec);
        Ok(())
    });
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
