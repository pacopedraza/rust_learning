#![allow(unused_mut)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or points.
    // Usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Please supply an argument to this program.");
            std::process::exit(-1);
        }).to_owned();

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    let mut x:i32 = 1;
    let mut y:i32 = 2;
    add(&mut x, &mut y);
}

// 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
// prints whether the contents of String is plural or singular. The uncomment and run this
// code with `cargo run apple` and `cargo run apples`. Hint: use `.ends_with("s")` on the
// String reference
//

fn inspect(s: &String) { // &String, it is a reference to a String
    if s.ends_with("s") { // the "." dereferences down of the value and calls the `ends_with` method
    // on the value that we passed a borrow string slice "s"
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

// 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s"
// to the String if it doesn't already ends with "s".
// `cargo run apple`. Hint: use `.push_str("s")` on the mutable String reference to add an "s".
//
fn change(s: &mut String) {
    // This function only works if we have a mutable string and if we have a mutable reference
    // to a mutable string.
    if !s.ends_with("s") { // if it doesn't end with `s`
        s.push_str("s");
    }
}

// 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
// Indicating whether or not the String both starts with a "b" AND contains an "a".
// Hint 1: use `.starts_with("b")` and `.contains("a")`
// Hint 2: `&&` is the boolean "AND" operator
//


fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a") // This is a simplified way to do return true or false
    // returning the condition itself.
    // if s.starts_with("b") && s.contains("a") {
    //     true
    // } else {
    //     false
    // }
}

fn add(x: &mut i32, y: &mut i32) { // Function with mutable referenced values.
    let mut result: i32 = 0;
    result = *x + *y; // Dereference the x and y parameters
    println!("The add result is: {}", &result)
    // println!("1 + 2 = {}, even via references", add(&1, &2));
}
