// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
// Define with double quotes

pub fn run() {

    // Primitive string
    let hello = "Hello";
 
    // String
    let mut hello2 = String::from("Hello");

    // Get length
    println!("Length: {}", hello2.len());

    // Push a character 
    hello2.push('W');

    // Pushes a string
    hello2.push_str(" World");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Check if empty
    println!("Is Empty: {}", hello2.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("Hello", "There"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello2);
}