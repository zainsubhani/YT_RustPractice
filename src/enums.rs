enum Movement {
    // variant
    up,
    down,
    left,
    right,
}
fn move_avatar(m: Movement) {
    // perform action depending on info
    match m {
        Movement::down => println!("Avatar moving down"),
        Movement::up => println!("Avatar moving up"),
        Movement::left => println!("Avatar moving left"),
        Movement::right => println!("Avatar moving right"),
    }
}
pub fn enum_excercise() {
    let avatar1: Movement = Movement::left;
    let avatar2: Movement = Movement::right;
    let avatar3: Movement = Movement::up;
    let avatar4: Movement = Movement::down;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
