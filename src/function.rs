pub fn function_excercise(){
        greet("hello", "zain");

    fn greet(greet:&str,name:&str) 
    {
        println!(" {}{}",greet,name)
    }
    // bind function values to variables
    let a =sum(5,5);
println!("sum: {}",a);
// closure
let add_num=|n1:i32,n2:i32|n1+n2 ;
println!("C sum :{}",add_num(3,3));

// closure example 2
let n3:i32=10;
let add_num=|n1:i32,n2:i32|n1+n2 +n3;
println!("C sum :{}",add_num(3,3));

}

// we can call a function before function definition 

pub fn sum(n:i32,x:i32)-> i32 {
    n+x

    

}





