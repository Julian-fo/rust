use std::fs::File;
use std::io::{ErrorKind, Write};

const FILE_NAME: &str = "hello.txt";

fn main() {
    let f: Result<_, _> = File::open(FILE_NAME);
    // let v = f.err().unwrap();
    // println!("The value of v is: {:?}", v);

    // match f {
    //     Ok(file) => println!("The value of f is: {:?}", file),
    //     Err(err) => println!("The value of err is: {:?}", err),
    // }

    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create(FILE_NAME) {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // match f.write("Hello, world!".as_bytes()) {
    //     Ok(_) => println!("The value of f is: {:?}", f),
    //     Err(e) => {
    //         panic!("There was a problem writing the file: {:?}", e)
    //     }
    // }
    let msg = format!("Failed to write file into: {}", FILE_NAME);
    f.write("Hello, world!".as_bytes()).expect(&msg);
}
