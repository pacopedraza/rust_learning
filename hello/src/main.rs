use hello::greet;

fn main() {
    println!("Hello, world! from main function");
    // hello::greet(); // This works but is not the best approach
                    // specifying an absolute path of something every call
                    // could be painful, specially if the path is quite long.
                    // we have to use "use"
    // After adding "use hello::greet;" the use it's outside of any smaller scope
    // brings greet into scope for all of main. So the file can be simplified.
    greet();
}
