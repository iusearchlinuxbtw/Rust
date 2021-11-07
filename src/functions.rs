// Functions - Used to store blocks of code for re-use

pub fn run() {

    greeting("Hello", "Matthew");

    // Bind function values to variables
    let get_sum = add(34, 35);

    println!("Sum: {}", get_sum);

    // Closure
    let z: i32 = 10;
    let add_nums = |x: i32, y: i32| x + y + z;
    println!("C Sum: {}", add_nums(3, 3));

}

fn greeting(greet: &str, name: &str) {

    println!("{} {}, Nice to meet you!", greet, name)

}

fn add(x: i32, y: i32) -> i32 {

    x + y
    
}