// Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }

fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);
    for c in 97..=122 {
        println!("{}", c);
    }
}
