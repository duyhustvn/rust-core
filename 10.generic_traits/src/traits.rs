use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
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

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// syntax sugar: short form
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bounds syntax: longer form
pub fn notify_trait_bound_syntax<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary)
// you can pass 2 different type to this function: NewsArticle and SocialPost for item1 and item2

// pub fn notify<T: Summary>(item1: &T, item2: &T)
// you can only pass 1 type to this function either NewsArticle or SocialPost,
// cannot pass NewsArticle for item1 and SocialPost for item2

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest value is {}", self.x);
        } else {
            println!("The largest value is {}", self.y);
        }
    }
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
