use std::fmt::Display;

fn main() {
    let s1 = String::from("hello world");
    let s2 = String::from("hello rust");
    let s3 = longest_string(&s1, &s2);
    println!("The longest string is {}", s3);

    println!("The longest string is {}", longest_announcement("hello world", "hello rust", "rust"));
}

fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display + std::fmt::Debug,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}