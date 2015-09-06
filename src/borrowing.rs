pub fn borrowing() {
    let vec_a = vec![1, 2, 3];
    let vec_b = vec![4, 5, 6];

    let answer = compute2(&vec_a, &vec_b);
    println!("The answer to compute2: {}", answer);

    println!("We can still use the vecs! {:?}, {:?}", &vec_a, &vec_b);

    // an &T reference is immutable

    let names = vec![];
    add_names(&names);

    // &mut T is mutable
    let mut names2 = vec![];
    add_names2(&mut names2);

    println!("names2 = {:?}", &names2);

    let mut x = vec![1, 2, 3];
    let mut y = vec![1, 2, 3];
    take(&mut x, &mut y);
    //take(&mut x, &mut x);
    take2(&mut x, &y);
    //take2(&mut x, &x);
}

// Without Borrowing
fn compute(a: Vec<i32>, b: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // Look through the vecs

    // return everything
    (a, b, 42)
}

// Borrowing means this function won't deallocate a or b when they go out of scope
fn compute2(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    // look through the vecs, but not mutate

    // return an answer
    42
}

fn add_names(names: &Vec<String>) {
    //names.push("Jill".to_string());
}

fn add_names2(names: &mut Vec<String>) {
    names.push("Jill".to_string());
    names.push("Jane".to_string());
}

fn take(x: &mut Vec<i32>, y: &mut Vec<i32>) {

}

fn take2(x: &mut Vec<i32>, y: &Vec<i32>) {

}
