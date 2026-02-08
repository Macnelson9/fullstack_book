pub fn say_hello () {
    println!("Hello, world!");
}

// Print function using arrays
// pub fn print() {
//     let numbers = [1,2,3,4,5];

//     let () = numbers;

//     for n in numbers.iter() {
//         println!("{}", n);
//     }
// }

// Print function using vector 
// pub fn print() {
//     let numbers = vec![1,2,3,4,5];

//     for n in numbers {
//         println!("{}", n);
//     }
// }

// Print function using a helper function
pub fn print() {
    let numbers = [1,2,3,4,5];

    output_sequence(numbers);
}

fn output_sequence(numbers: [u8; 5]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}