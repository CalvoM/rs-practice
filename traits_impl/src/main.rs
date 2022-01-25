pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Loading...")
}
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
        format!("{}: {}", self.username, self.content)
    }
}

pub struct LectureNotes {
    pub title: String,
    pub pages: String,
    pub references: String,
}

impl Summary for LectureNotes {
}
fn main() {
    let tweet = Tweet {
        username: String::from("d1r3ct0r"),
        content: String::from("Mental illness is prevalent"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let mechanics = LectureNotes {
        title: String::from("Forces"),
        pages: String::from("12-13 pages"),
        references: String::from("Static mechanics"),
    };
    println!("Upcoming lecture: {}", mechanics.summarize());
}
