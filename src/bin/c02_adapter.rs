use design_pattern_rust::c02_adapter::*;

fn main() {
    let p = PrintBanner::new("Hello");
    p.print_weak();
    p.print_strong();
}