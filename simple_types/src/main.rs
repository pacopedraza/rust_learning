// Silence the warning so they don't distract the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::{on_off, print_distance, print_array, print_difference, ding};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1); // Access the elements.

    // let coords_arr = [coords.0, coords.1]; // Create an array literal out of parts of `coord` here
    let coords_arr: [f32; 2] = [coords.0, coords.1]; // Same as above, but specify type 2 elements of f32.
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    // Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing. Done correctly, `cargo run` will produce the additional output.
    // "Ding, you found 13!"
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // Pass the `on_off` function the value `true` from the variable `mess`. Done correctly,
    // `cargo_run` will produce the additional output "Lights are on!" I'll get you started:
    on_off(mess.2[1].0); // This will get the `true` in the subindex.

    print_distance(coords);

}