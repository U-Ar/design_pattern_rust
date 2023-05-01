use design_pattern_rust::c06_prototype::*;

pub fn main() {
    let mut manager = Manager::new();

    let upen = Box::new(UnderlinePen::new('-'));
    let mbox = Box::new(MessageBox::new('*'));
    let sbox = Box::new(MessageBox::new('/'));

    manager.register("strong message",upen);
    manager.register("warning box", mbox);
    manager.register("slash box", sbox);

    let p1 = manager.create("strong message");
    p1.consume("Hello, world!");

    let p2 = manager.create("warning box");
    p2.consume("Hello, world!");

    let p3 = manager.create("slash box");
    p3.consume("Hello, world!");


}