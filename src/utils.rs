use std::intrinsics;

pub fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}
