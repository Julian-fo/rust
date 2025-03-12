// This file is used to learn about Box<T> and Deref trait
// Box<T> is a smart pointer that implements the Deref trait, which allows Box<T> to be treated like a reference
use std::{fmt::Debug, ops::Deref};
#[derive(Debug)]
pub struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> 
where T:Debug
{
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_coercion() {
        // Deref coercion is a convenience that Rust performs on arguments to functions and methods
        // It works only on types that implement the Deref trait
        // Deref coercion converts a reference to a type that implements Deref into a reference to a type that Deref returns
        // Deref coercion happens automatically when we pass a reference
        // to a value of a type that implements Deref to a place that needs a reference to a type that Deref implements
        fn hello(name: &str) {
            assert_eq!("Rust", name);
            println!("Hello, {}!", name);
        }

        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        
        // call std::mem::drop to explicitly drop the value
        // drop is a function that is used to explicitly drop a value before it goes out of scope
        // drop is used to free the memory that the value occupies
        // drop is called automatically when a value goes out of scope
        drop(m);
    }
}