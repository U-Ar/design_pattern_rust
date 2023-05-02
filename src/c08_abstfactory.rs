/*pub trait Factory {
    fn create_link(&self, caption: &str, url: &str) -> Box<dyn Link>;
    fn create_tray(&self, caption: &str) -> Box<dyn Tray>;
    fn create_page(&self, title: &str, author: &str) -> Box<dyn Page>;
}

pub trait Item {
    fn makeHTML(&self) -> String;
}
pub trait Link {
    fn makeHTML(&self) -> String;
}
pub trait Page {
    fn add(&mut self,item: Item);
    fn output(&self,filename: &str);
    fn makeHTML(&self) -> String;
}
pub trait Tray {
    fn add(&mut self,item: Item);
    fn makeHTML(&self) -> String;
}

pub fn get_factory(classname: &string) -> Option<Box<dyn Factory>>{
    match classname {
        "ListFactory" => Some(Box::new(ListFactory::new())),
        "DivFactory" => Some(Box::new(DivFactory::new())),
        _ => None,
    }
}

struct DivFactory;

impl Factory for DivFactory {
    fn create_link(&self, caption: &str, url: &str) -> Box<dyn Link> {
        Box::new(DivLink::new(caption,url))
    }
    fn create_tray(&self, caption: &str) -> Box<dyn Tray> {
        Box::new(DivTray::new(caption))
    }
    fn create_page(&self, title: &str, author: &str) -> Box<dyn Page> {
        Box::new(DivPage::new(title,author))
    }
}

struct DivLink {
    caption: String,
    url: String
}

impl DivLink {
    pub fn new(caption: &str, url: &str) -> Self {
        DivLink { caption: String::from(caption),
            url: String::from(url )}
    }
}

impl Link for DivLink {
    fn makeHTML(&self) -> String {
        format!("<div class=\"LINK\"><a href=\"{}\">{}</a></div>\n",self.url,self.caption)
    }
}

struct DivPage {
    title: String,
    author: String,
    content: Vec<DivItem>
}*/

