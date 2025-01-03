
pub fn run(){
    println!("run function");
}
pub fn simple_case(){
    println!("{}", 1); 
}
pub fn position_argument(){
    println!("{0} is learning {1} and {0} is good","Zain","Rust")
}
pub fn formating_case() {
      println!("{} is from {} excerise is {}","Zain","france","basic formatting ") 

}
pub fn argument_case(){
    println!("{name} is good",name="Zain");



}
pub fn placeholder_trait(){
    println!("Binary :{:b} Hex:{:x} Octal:{:o}",10,10,10  )

}
pub fn placeholder_debug_trait(){
    println!("{:?}",(12,true,"hello"));
}
pub fn basic_math(){
    println!("10+10={}",10+10);

}