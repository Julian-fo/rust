#[derive(Debug)]

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_list() {
        use super::List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
        println!("{:?}", list);
    }

    #[test]
    fn test_message() {
        use super::Message::{Quit, Move, Write, ChangeColor};
        let m = Quit;
        println!("{:?}", m);
        let m = Move { x: 1, y: 2 };
        println!("{:?}", m);
        let m = Write(String::from("hello"));
        println!("{:?}", m);
        let m = ChangeColor(1, 2, 3);
        println!("{:?}", m);
    }
}