use design_pattern_rust::c09_bridge::*;


fn main() {
    let i1 = StringDisplayImpl::new("Hello, Japan.");
    let i2 = StringDisplayImpl::new("Hello, World.");
    let i3 = StringDisplayImpl::new("Hello, Univerese.");
    let d1 = PlainDisplay{};
    let d2 = CountDisplay{};

    d1.display(&i1);
    d1.display(&i2);
    d2.display(&i3);
    d2.multi_display(&i3,5);
}