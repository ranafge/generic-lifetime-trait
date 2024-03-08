use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String{
        "default implementation".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} , by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}" , self.username, self.content)
    }
}

pub fn notify<T: Summary >(item:&T) {
    println!("form notify meto {} ", item.summarize())
}

