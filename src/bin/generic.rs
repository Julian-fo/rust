fn main() {
    println!("Hello, world!");

    let p: Point<u32> = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
}

#[allow(dead_code)]
struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // generic implementation, defined for all types <T>
    #[allow(dead_code)]
    fn x(&self) -> &T {
        &self.x
    }
}
#[allow(dead_code)]
trait Summary {
    fn summarize(&self) -> String; // method signature, without default implementation

    fn default_summarize(&self) -> String {
        String::from("(Read more...)") // default implementation
    }
}
