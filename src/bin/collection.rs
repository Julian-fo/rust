fn main() {
    println!("hello_world::server");

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.remove(2);

    let first = &v[0];

    println!("The value of first is: {}", first);

    if let Some(second) = v.get(100) {
        println!("The value of second is: {}", second);
    } else {
        println!("There is no second value.");
    }

    v.iter().for_each(|e| println!("The value of e is: {}", e));
    v.iter().for_each(|e| println!("The value of e is: {}", e));

    println!("The value of v[0] is: {:#?}", v);

    for v in &mut v {
        *v += 1;
        println!("The value of v is: {}", v);
    }
}
