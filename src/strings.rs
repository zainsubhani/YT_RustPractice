
// two type of string
// prmivitve string = immutable string fixed length string somewhere in memory
// string = growable,heap-allocated data structure - use when you need to modify own string data
pub fn stringexcercise(){
let _hello = "hello world";
let _ghello = String::from("Hello");
println!("{} {} {}",_ghello,_hello,_hello.len());




}
pub fn stringmutable(){
let mut hello = String::from("Hello");
    hello.push('z'); // only can push one character
    println!("{}",hello);
    hello.push_str("zain ");
    println!("{}",hello.capacity());
    assert_eq!(2,hello.len()); // to testing
    let mut s = String::with_capacity(2);
    

}
