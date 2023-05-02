pub trait Display {
    fn open(&self,display_impl: &dyn DisplayImpl) {
        display_impl.raw_open();
    }

    fn print(&self,display_impl: &dyn DisplayImpl) {
        display_impl.raw_print();
    }

    fn close(&self,display_impl: &dyn DisplayImpl) {
        display_impl.raw_close();
    }

    fn display(&self,display_impl: &dyn DisplayImpl) {
        self.open(display_impl);
        self.print(display_impl);
        self.close(display_impl);
    }
}

pub trait DisplayImpl {
    fn raw_open(&self);
    fn raw_print(&self);
    fn raw_close(&self);
}



pub struct PlainDisplay;
impl Display for PlainDisplay {}

pub struct CountDisplay;
impl Display for CountDisplay {}


pub struct StringDisplayImpl {
    string: String,
    width: usize,
}

impl StringDisplayImpl {
    pub fn new(string: &str) -> Self {
        StringDisplayImpl { string:String::from(string), width:string.len() }
    }
    fn print_line(&self) {
        print!("+");
        for _i in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl DisplayImpl for StringDisplayImpl {
    fn raw_open(&self) {
        self.print_line();
    }
    fn raw_print(&self) {
        println!("|{}|",self.string);
    }
    fn raw_close(&self) {
        self.print_line();
    }
}


impl CountDisplay {
    pub fn multi_display(&self,display_impl: &dyn DisplayImpl,times: i64) {
        self.open(display_impl);
        for _i in 0..times {
            self.print(display_impl);
        }
        self.close(display_impl);
    }
}


