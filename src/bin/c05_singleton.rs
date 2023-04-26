use design_pattern_rust::c05_singleton::*;

fn main() {
    println!("Start.");
    let obj1 = Triple::get_instance("ALPHA");
    let obj2 = Triple::get_instance("ALPHA");
    if obj1 == obj2 {
        println!("obj1とobj2は同じインスタンス。");
    } else {
        println!("obj1とobj2は違うインスタンス。");
    }
    let obj3 = Triple::get_instance("BETA");
    if obj1 == obj3 {
        println!("obj1とobj3は同じインスタンス。");
    } else {
        println!("obj1とobj3は違うインスタンス。");
    }
}