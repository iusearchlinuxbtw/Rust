use std::env;

pub fn run() {

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = args[2].clone();
    let status = args[2].clone();

    if command == "hello" {

        println!("Hi {}, How are you?", name);

    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is a valid command.");
    }

    //println!("Args: {:?}", command);

}