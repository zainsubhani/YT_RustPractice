pub fn arrays_excercise(){
    let numbers:[i32;5]=[1,2,3,4,5];
    println!("{:?}",numbers);
println!("{}",numbers[0]);
println!("Arrays occupied numbers {} bytes",std::mem::size_of_val(&numbers));


}
pub fn mutarrays_excercise(){
    let mut num:[i32;5]=[1,2,3,4,5];
    num[2]=5;
    println!("{:?}",num);
    println!("{:?}",num.len());


// Arrays are stack allocated
println!("Arrays occupied {} bytes",std::mem::size_of_val(&num));

//Get slice
let slice: &[i32] =&num[0..2];
println!("Slice is {:?}",slice);



}