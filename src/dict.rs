use std::default::Default;
use std::rc::Rc;
use word::Word;

struct NamedWord {
    pub name: String,
    pub word: Rc<Word>,
}

#[derive(Default)]
pub struct Dict {
    vec: Vec<NamedWord>,
}

impl Dict {
    pub fn new() -> Dict {
        Default::default()
    }

    pub fn push_word(&mut self, name: String, word: Word) {
        self.vec.push(NamedWord { name: name, word: Rc::new(word) } )
    }

    pub fn find_word(&mut self, name: &str) -> Option<Rc<Word>> {
        self.vec.iter()
            .find(|named_word| named_word.name == name)
            .and_then(|named_word| Some(named_word.word.clone()))
    }
}
