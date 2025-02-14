use std::ops::Add;

fn main() {
    println!("Hello, world!");
    let tup = (500, 6.4, 1);

    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
    println!("The value of x is: {}", z);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    another_function(11);
    five(11);

    let c = if x < 10 { "less" } else { "more" };
    println!("The value of c is: {}", c);

    let c = loop {
        println!("again");
        break 5;
    };
    println!("The value of c is: {}", c);

    for i in 1..=10 {
        println!("The value of i is: {}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for e in a.iter() {
        println!("The value of e is: {}", e);
    }

    for i in (1..4).rev() {
        println!("The value of i is: {}", i);
    }

    let v = [1, 2, 3, 4, 5];
    let v2 = v;
    println!("The value of v[0] is: {}", v[0]);
    println!("The value of v2[0] is: {}", v2[0]);

    let s1 = String::from("hello");
    let s2 = &s1[0..2];
    println!("The value of s1 is: {}", s1);
    println!("The value of s2 is: {}", s2);

    let s1: &[i32] = &[1, 2, 3, 4, 5];
    let s2: Vec<i32> = s1.to_vec();
    println!("The value of s1[0] is: {}", s1[0]);
    println!("The value of s2[0] is: {}", s2[0]);

    let v = vec![1, 2, 3];
    println!("Value is {}", v[0]);

    let s = String::from("hello");
    let s = &s;
    println!("String value: {}!", s);

    let st = ST(1, 2, String::from("hello world"));
    println!("Struct tuple value is: {}, {}, {}", st.0, st.1, st.2);

    let color = Color {
        green: String::from("green"),
        red: String::from("red"),
        yellow: String::from("yellow"),
    };
    let color2 = Color {
        green: String::from("yellow  green"),
        ..color
    };

    println!(
        "Color2 values is: {}, {}, {}",
        color2.green, color2.red, color2.yellow
    );

    let c = add_color(color2);
    println!("The color is {:?}", c);

    for i in 1..10 {
        println!("foreach value {}", i);
    }

    println!("hello {0} world {1} with {0:3.2}", 3.1412233, "foo");

    let v1 = (1, 2, 3);
    if matches!(v1, (_, 2, _)) {
        println!("match value");
    }

    if let (x, 2, y) = v1 {
        println!("match value: {} {}", x, y);
    }

    let v1 = Some(5);
    while let Some(5) = v1 {
        println!("match loop");
        break;
    }
}

fn another_function(x: i32) {
    println!("Another function {}.", x);
}

fn five(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    x + 5
}

struct ST(i32, i64, String);

#[derive(Debug)]
struct Color {
    green: String,
    red: String,
    yellow: String,
}

fn add_color(color: Color) -> Color {
    let c = Color {
        green: color.green.add("added"),
        red: color.red,
        yellow: color.yellow,
    };

    c
}
