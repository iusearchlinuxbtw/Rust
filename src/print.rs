pub fn run() {
    // Prints to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{}", 1);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "California", "code"); 

    // Named arguments
    println!("{name} likes to player {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic path
    println!("10 + 10 = {}", 10 + 10);
}