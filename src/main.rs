mod arrays;
mod cli;
mod conditional;
mod enums;
mod function;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod typeexcercise;
mod var;
mod vector;
fn main() {
    println!("Hello, world! from main");
    cli::cli_excercise();
    enums::enum_excercise();

    structs::struct_excercise2();
    structs::tuple_struct();
    structs::struct_excercise();
    pointer_ref::pointer_vector2();
    pointer_ref::pointer_vector();
    pointer_ref::pointer_excercise();
    function::function_excercise();
    loops::for_loop();
    loops::while_excercise();
    loops::loop_excercise();
    conditional::condition_excercise();
    vector::loopmutatblevalue();

    vector::loop_vector();
    vector::vector_excercise();

    arrays::mutarrays_excercise();
    arrays::arrays_excercise();

    tuples::tuples_excercise();

    strings::stringmutable();
    strings::stringexcercise();
    typeexcercise::types();
    print::run();
    print::simple_case();
    print::formating_case();
    print::position_argument();
    print::argument_case();
    print::placeholder_trait();
    print::placeholder_debug_trait();
    print::basic_math();
    var::variable();
    var::constant();
    var::multivalue();
}
