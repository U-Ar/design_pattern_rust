use std::collections::HashMap;

pub trait Product {
    fn consume(&self, s: &str);
    fn create_copy(&self) -> Box<dyn Product>;
}

pub struct Manager {
    showcase: HashMap<String,Box<dyn Product>>
}

impl Manager {

    pub fn new() -> Self {
        Manager { showcase: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, prototype: Box<dyn Product>) {
        //showcase.put(String::from(name), prototype);
        //self.showcase[name] = prototype;
        self.showcase.insert(String::from(name),prototype);
    }

    pub fn create(&self, prototype_name: &str) -> Box<dyn Product> {
        let p = self.showcase.get(prototype_name);
        p.unwrap().create_copy()
    }
}

pub struct MessageBox {
    decochar: char
}

impl MessageBox {
    pub fn new(decochar: char) -> Self {
        MessageBox { decochar }
    }
}

impl Product for MessageBox {
    fn consume(&self, s: &str) {
        let decolen = 1 + s.len() + 1;
        for _i in 0..decolen {
            print!("{}",self.decochar);
        }
        println!();
        println!("{}{}{}",self.decochar,s,self.decochar);
        for _i in 0..decolen {
            print!("{}",self.decochar);
        }
        println!();
    }

    fn create_copy(&self) -> Box<dyn Product> {
        Box::new(MessageBox::new(self.decochar))
    }
}

pub struct UnderlinePen {
    ulchar: char
}

impl UnderlinePen {
    pub fn new(ulchar: char) -> Self {
        UnderlinePen { ulchar }
    }
}

impl Product for UnderlinePen {
    fn consume(&self,s: &str) {
        let ulen = s.len();
        println!("{}",s);
        for _i in 0..ulen {
            print!("{}",self.ulchar);
        }
        println!();
    }

    fn create_copy(&self) -> Box<dyn Product> {
        Box::new(UnderlinePen::new(self.ulchar))
    }
}