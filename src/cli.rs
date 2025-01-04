use std::env;
pub fn cli_excercise() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("Args {:?}", args);
    println!("Command {:?}", command);
    if command == "hello" {
        println!("Hello how are you hello to rust ");
    } else if command == "status" {
        println!("status is completed");
    } else {
        println!("that is not valid command");
    }
}
