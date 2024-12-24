use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.headline, self.content)
    }
}

pub fn notify<T: Summary + Display> (item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item);
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.72 is released!"),
        content: String::from("The rust programming language has a new version!"),
    };

    notify(&article);
}