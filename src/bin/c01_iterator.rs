//use crate::C01_iterator::*;

use design_pattern_rust::c01_iterator::*;

fn main() {
    println!("Hello, world!");

    let mut bookshelf = BookShelf::new();

    bookshelf.append_book(Book { name: "Around the World in 80 Days".to_string() });
    bookshelf.append_book(Book { name: "Bible".to_string() });
    bookshelf.append_book(Book { name: "Cinderella".to_string() });
    bookshelf.append_book(Book { name: "Daddy-Long-Legs".to_string() });

    for item in &bookshelf {
        println!("{}",item.name);
    }
    println!();

}