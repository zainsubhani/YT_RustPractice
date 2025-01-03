//Primitive type
// integer : u8,i8,u16,i16,u32,i32,u64,i64,u128,i128,u256,i256,(number of bits they can take in memory)
// float: f32,f64
// boolean
// characters
// tuples
// arrays

// rust is static type language which mean that it must know the type of all variable at compile time ,how 
// the compiler can usually infer what type you want to use based on value provided and how we used it
pub fn types(){
    //default is i32
    let x=1;
    // defsult is f64
    let x=2.5;
    // add explicit type
    let y:i64=4543564354345;
    // find max number
    println!("Max number i32 is {}",std::u32::MAX);
    println!("Max number i64 is {} ",std::u64::MAX);
    
// boolean

let face: char = '\u{1F600}';
let is_active = true;
let is_greater = 2>3;
println!("{:?}",(x,y,is_active,is_greater,face));
// char
let a1='a';

// hoisting not work here like javascript







}