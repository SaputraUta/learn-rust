pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as I said before, people"),
        reply: false,
        retweet: false,
    };

    let article = Article {
        title: String::from("Penguins"),
        author: String::from("Jane Doe"),
        content: String::from("I saw penguins today"),
    };

    let news_article = NewsArticle {
        title: String::from("Cats"),
        author: String::from("John Doe"),
        content: String::from("I saw cats today"),
    };

    notify(&tweet);
    notify(&article);
    notify(&news_article);

    // println!("Tweet: {} ", tweet.summarize());
    // println!("Article: {} ", article.summarize());
    // println!("News Article: {} ", news_article.summarize());

    // println!("Tweet: {} ", tweet.summarize_author());
    // println!("Article: {} ", article.summarize_author());
    // println!("News Article: {} ", news_article.summarize_author());
}
