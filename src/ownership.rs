pub fn ownership() {
    let v = vec![1, 2, 3];

    //let z = v; // transfer ownership of the vec to z

    println!("v is {:?}", v); // can't use v anymore

    // Functions can take ownership too
    let s = vec!["hey", "you"];
    //take(s);

    println!("s is {:?}", s);
}

fn take<T>(v: Vec<T>) {
    // consume the vec
}
