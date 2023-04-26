pub trait Print {
    fn print_weak(&self);
    fn print_strong(&self);
}

pub struct Banner {
    string: String
}

impl Banner {
    pub fn new(string: &str) -> Banner {
        Banner { string: string.to_string() }
    }

    pub fn show_with_paren(&self) {
        println!("({})",self.string);
    }

    pub fn show_with_aster(&self) {
        println!("*{}*",self.string);
    }

}

// Rustに継承とか存在しないので委譲型でAdapterを実装する
pub struct PrintBanner {
    banner: Banner
}


impl PrintBanner {
    pub fn new(string: &str) -> PrintBanner {
        PrintBanner { banner: Banner::new(string) }
    }
}

impl Print for PrintBanner {
    
    fn print_weak(&self) {
        self.banner.show_with_paren();
    }

    fn print_strong(&self) {
        self.banner.show_with_aster();
    }
    
}

