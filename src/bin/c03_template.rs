use design_pattern_rust::c03_template::*;

fn main() {
    let d1 = CharDisplay::new('H');
    let d2 = StringDisplay::new("Hello, world.");

    d1.display();
    d2.display();
}