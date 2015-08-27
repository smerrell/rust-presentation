pub fn ownership() {
    let v = vec![1, 2, 3];

    //let z = v; // transfer ownership of the Vector to z

    println!("v is {:?}", v); // can't use v anymore

    // Functions can take ownership too
    let s = vec!["hey", "you"];
    //take(s);

    println!("s is {:?}", s);

    // The Copy trait (i.e. interface)
    let mut a = 1;
    let b = a; // primitives are all Copy

    // a and b are both on the Stack

    println!("a: {}, b: {}", a, b);

    a = 99;
    println!("a: {}, b: {}", a, b);

    // Vectors do not implement Copy so they reference
    // memory in the Heap
}

fn take<T>(v: Vec<T>) {
    // consume the Vector
}
