pub trait Display {
    fn display(&self);
}

pub struct CharDisplay {
    char: char
}

impl CharDisplay {
    pub fn new(char: char) -> CharDisplay {
        CharDisplay { char }
    }
}

impl Display for CharDisplay {
    fn display(&self) {
        println!("{}",self.char);
    }
}

pub struct StringDisplay {
    string: String
}

impl StringDisplay {
    pub fn new(string: &str) -> StringDisplay {
        StringDisplay { string: string.to_string() }
    }
}

impl Display for StringDisplay {
    fn display(&self) {
        println!("{}",self.string);
    }
}

