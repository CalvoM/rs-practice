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

pub fn notify(item: &impl Summary) {
    println!("Breaking information: {}", item.summarize());
}
pub fn verbose_notify<T>(item: &T)
    where T: Summary,
{
    print!("Verbose description of {}", item.summarize());
}

pub fn get_summarizable() -> impl Summary {
    Tweet{
        username: String::from("elonmusk"),
        content: String::from("Now the technoking"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("d1r3ct0r"),
        content: String::from("Mental illness is prevalent"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    let mechanics = LectureNotes {
        title: String::from("Forces"),
        pages: String::from("12-13 pages"),
        references: String::from("Static mechanics"),
    };
    let new_tweet = get_summarizable();
    notify(&mechanics);
    verbose_notify(&mechanics);
    verbose_notify(&new_tweet);
}
