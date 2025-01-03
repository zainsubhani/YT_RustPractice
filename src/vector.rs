use core::num;

pub fn vector_excercise(){
    let mut numbers: Vec<i32>=vec![1,2,3,4,5];

    println!("{:?}",numbers);
println!("{}",numbers[0]);
println!("Arrays occupied numbers {} bytes",std::mem::size_of_val(&numbers));
numbers.push(2);
numbers.push(8);
numbers.push(9);

    println!(" After Push {:?}",numbers);


}
pub fn loop_vector(){
        let mut numbers: Vec<i32>=vec![1,2,3,4,5];

    for x in numbers.iter()
    {
        println!("Numeber: {}",x);

    }
  


}
  pub fn loopmutatblevalue(){
                let mut numbers: Vec<i32>=vec![1,2,3,4,5];

                for x in numbers.iter_mut()
                {
                    *x*=2;
                    println!("number multiply by 2 {:?}",x);
                    

                }
                                    println!("numbers array after multiply {:?}",numbers);

    }





