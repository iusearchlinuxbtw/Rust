// Vectors - Resizable arrays

use std::mem;

pub fn run() {

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} byes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    println!("{:?}", numbers);

    // Loop through vecor values

    for x in numbers.iter() {

        println!("Number: {}", x);

    }

    // Loop & mutate values
    for x in numbers.iter_mut() {

        *x *= 2;

    }

    println!("Numbers Vec: {:?}", numbers);

}