// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {

    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar 1 moving up"),
        Movement::Down => println!("Avatar 2 moving down"),
        Movement::Left => println!("Avatar 3 moving left"),
        Movement::Right => println!("Avatar 4 moving right")
    }

}

pub fn run() {

    let av1 = Movement::Left;
    let av2 = Movement::Right;
    let av3 = Movement::Up;
    let av4 = Movement::Down;

    move_avatar(av1);
    move_avatar(av2);
    move_avatar(av3);
    move_avatar(av4);

}