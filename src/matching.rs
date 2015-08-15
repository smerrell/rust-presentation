#[allow(dead_code)]
enum OptionalInt {
    Value(i32),
    Missing,
}

pub fn matching() {
    // Matching
    let x = 2;
    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        4 => println!("four!"),
        5 => println!("five!"),
        _ => println!("Something else!"), // matches anything
    }

    let y = 7;
    match y {
        1 | 2 => println!("one or two"),
        3 => println!("three!"),
        4 => println!("four!"),
        m @ 5 ... 9 => println!("five through nine! Got {}", m),
        _ => println!("something else!"),
    }

    let z = '💅';
    match z {
        'a' ... 'h' => println!("early letters"),
        'j' ... 'z' => println!("late letters"),
        _ => println!("something else"),
    }

    // Enums
    let test_enum = OptionalInt::Value(4);
    match test_enum {
        OptionalInt::Value(i) if i > 5 => println!("greater than five! Got {}", i),
        OptionalInt::Value(i)  => println!("less than five! Got {}", i),
        // Completeness is enforced, if the Missing part gets commented out...
        OptionalInt::Missing  => println!("no value"),
    }

    // Matching works for a form of if statement
    let test_enum_if = Some(32);
    if let Some(x) = test_enum_if {
        println!("Got x: {}", x);
    } else {
        println!("missing");
    }

    // instead of
    if test_enum_if.is_some() {
        let x = test_enum_if.unwrap();
        println!("Got x: {}", x);
    } else {
        println!("missing");
    }

    // or
    match test_enum_if {
        Some(x) => { println!("Got x: {}", x) },
        None => { println!("mising") }
    }

    // note: Some is part of the Option<T> enum, it is
    // similar to a nullable type in C# but we get
    // better usage semantics from how Rust does this
}
