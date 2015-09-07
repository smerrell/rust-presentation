pub fn lifetimes() {
    // lifetimes are what help the compiler reason about how long
    // references have to live in the program.
    //
    // Local references live only as long as their scope, but once
    // we bring in borrowing, references now can live longer. Hence
    // the reason for needing lifetimes to help the compiler. C#
    // doesn't need this because it is Garbage Collected and the GC
    // does the work of deciding when a reference can be collected.
    // Rust by default collects as soon as the reference goes out of
    // scope
    //
    // so for example:
    let a = 5;                 // +--- a goes into scope ---
    {                          //                          |
        let x = 5;             // +--- x goes into scope   |
        println!("x: {}", x);  // |                        |
        println!("a: {}", a);  // |                        |
    }                          // ---- x is freed          |
                               //                          |
                               //                          |
    println!("a: {}", a);      //                          |
}                              // ---- a is freed ---------|
