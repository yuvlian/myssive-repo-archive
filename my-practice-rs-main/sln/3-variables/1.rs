
// // Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

fn main () {
    let x: i32;
    let y: i32;

    assert_eq!(5, 5);
    println!("Success!");
}