mod print;
mod var;
mod typeexcercise;
mod strings;
mod tuples;
mod arrays;
mod vector;
mod conditional;
mod loops;
mod function;
fn main() {
    println!("Hello, world! from main");
    function::function_excercise();
    loops::for_Loop();
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

