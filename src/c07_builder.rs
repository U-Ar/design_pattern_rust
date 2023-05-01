use std::fs::File;
use std::io::Write;
use crate::c03_template::StringDisplay;

pub trait Builder {
    fn make_title(&mut self, title: &str);
    fn make_string(&mut self, string: &str);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
}

pub struct Director<'a> {
    builder: &'a mut dyn Builder
}

impl<'a> Director<'a> {
    pub fn new(builder: &'a mut dyn Builder) -> Self {
        Director { builder }
    }

    pub fn construct(&mut self) {
        self.builder.make_title("Greeting");
        self.builder.make_string("一般的な挨拶");
        self.builder.make_items(vec![String::from("How are you?"),String::from("Hello."),String::from("Hi.")]);
        self.builder.make_string("時間帯に応じた挨拶");
        self.builder.make_items(vec![String::from("Good morning."),String::from("Good afternoon"),String::from("Good evening.")]);
        self.builder.close();
    }
}


pub struct HTMLBuilder {
    filename: String,
    content: String
}

impl HTMLBuilder {
    pub fn new() -> Self {
        HTMLBuilder {
            filename: String::from(""),
            content: String::from("")
        }
    }

    pub fn get_html_result(&self) -> String {
        String::from(&self.filename)
    }
}


impl Builder for HTMLBuilder {
    fn make_title(&mut self, title: &str) {
        self.filename = String::from(title) + ".html";
        self.content = String::from("<!DOCTYPE html>\n");
        self.content += "<html>\n<head><title>";
        self.content += title;
        self.content += "</title></head>\n<body>\n<h1>";
        self.content += title;
        self.content += "</h1>\n\n";

    }

    fn make_string(&mut self, string: &str) {
        self.content += "<p>";
        self.content += string;
        self.content += "</p>\n\n";
    }

    fn make_items(&mut self, items: Vec<String>) {
        self.content += "<ul>";
        for s in items {
            self.content += "<li>";
            self.content += &s;
            self.content += "</li>";
        }
        self.content += "</ul>\n\n";
    }

    fn close(&mut self) {
        self.content += "</body></html>\n";
        let mut file = File::create(format!("resources/{}",&self.filename)).unwrap();
        //file.write_(&self.content);

        println!("close called.");
        write!(file,"{}",self.content).unwrap();
    }
}

pub struct TextBuilder {
    content: String
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder { content: String::from("") }
    }

    pub fn get_text_result(&self) -> String { String::from(&self.content) }
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: &str) {
        self.content += "=====================\n";
        self.content += "『";
        self.content += title;
        self.content += "』\n\n";
    }
    fn make_string(&mut self, string: &str) {
        self.content += "■";
        self.content += string;
        self.content += "\n\n";
    }
    fn make_items(&mut self, items: Vec<String>) {
        for item in items {
            self.content += "　・";
            self.content += &item;
            self.content += "\n";
        }
        self.content += "\n";
    }
    fn close(&mut self) {
        self.content += "=====================\n"
    }
}
