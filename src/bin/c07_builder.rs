use design_pattern_rust::c07_builder::*;

use std::env;
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid argument.");
    }

    if args[1] == "text" {
        let mut textbuilder = TextBuilder::new();
        let mut director = Director::new(&mut textbuilder);
        director.construct();
        let result = textbuilder.get_text_result();
        println!("{}",result);
    } else if args[1] == "html" {
        let mut htmlbuilder = HTMLBuilder::new();
        let mut director = Director::new(&mut htmlbuilder);
        director.construct();
        let result = htmlbuilder.get_html_result();
        println!("{}",result);
    }



}