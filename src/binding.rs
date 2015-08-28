use utils;

pub fn binding() {
    let inferred_type = "I'm a String".to_string();
    let string = "I'm a static string slice (&str)";

    print!("inferred_type is: ");
    utils::print_type_of(&inferred_type);
    print!("&string is: ");
    utils::print_type_of(&string);

    let explicit_type: i32 = 2;
    print!("&explicit_type is: ");
    utils::print_type_of(&explicit_type);

    let mut mutable_type = 1;
    println!("mutable_type: {}", mutable_type);

    mutable_type = 10;

    println!("mutable_type: {}", mutable_type);

    let tuple = (1, 2, 3);

    // destructuring
    let (a, b, c) = tuple;

    println!("a: {}, b: {}, c: {}", a, b, c);

    // destructuring a struct
    let thing = Thing { point: (1, -2), name: "Bar" };
    println!("{:?}", thing);
    let Thing { name: n, point: (x, y) } = thing;
    println!("name: {}, point x {}, point y {}", n, x, y);

}

#[derive(Debug)]
struct Thing<'a> {
    point: (i32, i32),
    name: &'a str
}

