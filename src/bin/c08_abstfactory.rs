use design_pattern_rust::c08_abstfactory::*;

use std::env;

fn main() {/*
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --bin c08_abstfactory list.html ListFactory");
        return;
    }

    let filename = args[1];
    let classname = args[2];

    let mut factory = Factory::getFactory(&classname).unwrap();


    let blog1 = factory.create_link("Blog 1", "https://example.com/blog1");
    let blog2 = factory.create_link("Blog 2", "https://example.com/blog2");
    let blog3 = factory.create_link("Blog 3", "https://example.com/blog3");

    let mut blog_tray = factory.create_tray("Blog Site");
    blog_tray.add(blog1);
    blog_tray.add(blog2);
    blog_tray.add(blog3);

    let news1 = factory.create_link("News 1", "https://example.com/news1");
    let news2 = factory.create_link("News 2", "https://example.com/news2");
    let mut news3 = factory.create_tray("News 3");
    news3.add(factory.create_link("News 3 (US)","https://example.com/news3us"));
    news3.add(factory.create_link("News 3 (JP)","https://example.com/news3jp"));

    let mut news_tray = factory.create_tray("News Site");
    news_tray.add(news1);
    news_tray.add(news2);
    news_tray.add(news3);

    let mut page = factory.create_page("Blog and News", "Hiroshi Yuki");
    page.add(blog_tray);
    page.add(news_tray);

    page.output(filename);*/
}