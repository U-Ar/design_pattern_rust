pub trait Product {
    fn consume(&self);
    fn to_string(&self) -> String;
}

pub struct IDCard {
    owner: String,
    id: i64
}

impl IDCard {
    pub fn new(owner: &str, id: i64) -> IDCard {
        IDCard { owner: owner.to_string(), id }
    }
}

impl Product for IDCard {
    fn consume(&self) {
        println!("{}を使います。",self.to_string());
    }
    fn to_string(&self) -> String {
        format!("[IDCard {} owned by {}]",self.id, self.owner)
    }
}

pub trait Factory {
    fn create_product(&mut self,owner: &str) -> Box<dyn Product>;
    fn register_product(&mut self,product: &Box<dyn Product>);
}

pub struct IDCardFactory {
    last: [i64;1],
}

impl IDCardFactory {
    pub fn new() -> IDCardFactory {
        IDCardFactory { last: [0] }
    }

    pub fn create(&mut self, string: &str) -> Box<dyn Product> {
        let p = self.create_product(string);
        self.register_product(&p);
        p
    }
}

impl Factory for IDCardFactory {
    fn create_product(&mut self, owner: &str) -> Box<dyn Product> {
        self.last[0] = self.last[0] + 1;
        Box::new(IDCard::new(owner,self.last[0]-1))
    }

    fn register_product(&mut self, product: &Box<dyn Product>) {
        println!("{}を登録しました。",product.to_string());
    }


}

