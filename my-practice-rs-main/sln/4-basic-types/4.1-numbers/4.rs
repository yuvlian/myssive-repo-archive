
// // Fill the blanks to make it work
// fn main() {
//     assert_eq!(i8::MAX, __); 
//     assert_eq!(u8::MAX, __); 

//     println!("Success!");
// }


// Fill the blanks to make it work
fn main() {
    let i8_max = 2i32.pow(7) - 1;
    let u8_max = 2i32.pow(8) - 1;
    assert_eq!(i8::MAX, i8_max as i8);
    assert_eq!(u8::MAX, u8_max as u8);

    println!("Success!");
}