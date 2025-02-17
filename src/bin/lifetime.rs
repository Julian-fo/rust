
fn main() {
    let s1 = String::from("hello world");
    let s2 = String::from("hello rust");
    let s3 = longest_string(&s1, &s2);
    println!("The longest string is {}", s3);
}

fn longest_string<`a>(s1: &`a str, s2: &`a str) -> &`a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}