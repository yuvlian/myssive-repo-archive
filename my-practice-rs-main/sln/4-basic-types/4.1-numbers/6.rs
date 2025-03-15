
// // Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1579);

//     println!("Success!");
// }


// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    // 1_024 = 1024
    // (0xff, 0b1111_1111) = (255, 255)
    // 0o77 = (7*8.pow(1)) + (7*8.pow(0)) = 56 + 7 = 63

    // v = 1024 + 255*2 + 63
    assert!(v == 1597);

    println!("Success!");
}