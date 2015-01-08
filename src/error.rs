#[derive(Show, Copy)]
pub enum Error {
    StackUnderflow,
    UnknownWord,
    SyntaxError,
}
