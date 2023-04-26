use design_pattern_rust::c04_factory::*;

fn main() {
    let mut factory = IDCardFactory::new();
    let card1 = factory.create("Hiroshi Yuki");
    let card2 = factory.create("Tomura");
    let card3 = factory.create("Hanako Sato");
    card1.consume();
    card2.consume();
    card3.consume();
}