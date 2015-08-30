pub fn functions() {

    println!("add_two(2) = {}", add_two(2));

    // returning unit ()
    let x = say_hello("person");
    println!("x is {:?}", x);

    // lambdas with explicit parameters and return
    let add_four = |x: i32| -> i32 { x + 4 };
    println!("add_four(4) = {}", add_four(4));

    // lambdas with inferred parameters and return
    let add_four_twice = |x| { add_four(add_four(x)) };
    println!("add_four_twice(4) = {}", add_four_twice(4));

    let name = "Ada Lovelace";
    let closure = || { println!("Hello, {}", name); };
    closure();

    let result = apply_to_5(|x| 5 + x);
    println!("apply_to_5: {}", result);

    let function = create_fn();
    function();

    let upper = 1000;

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n| n < upper)
            .filter(|n| is_odd(*n))
            .fold(0, |sum, i| sum + i);

    println!("sum of squared odd numbers: {}", sum_of_squared_odd_numbers);

}

pub fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn say_hello(name: &str) -> () {
    println!("Hello, {}", &name);
}

fn apply_to_5<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 { // declares the lambda signature

    f(5)
}

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}
