// All items in a library are private by default, even to binaries in the same project.
// "pub" as a prefix in the function, makes this public.
pub fn greet() {
    println!("Hi from greet library function!");
}
