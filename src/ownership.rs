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

    // returning ownership the hard way
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let (a, b, answer) = compute(a, b);
    println!("a: {:?}, b: {:?}, answer: {}", a, b, answer);
}

fn take<T>(v: Vec<T>) {
    // consume the Vector
}

fn compute(a: Vec<i32>, b: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // Look through the vecs

    // return everything
    (a, b, 42)
}
