pub fn loop_excercise(){
    let mut count = 10;
    // infinite loop
    loop{
        count+=1;
        println!("Number : {}",count);
        if count == 20{
            break;
        }
    }
}
// whileloop fizzbuzz
pub fn while_excercise(){
  let  count =10;
    while count <=100 {
        if count %15 ==0{
            println!("fizzbuzz");

        }
        else 
        {
        println!("please try again");
        break;

        }

    }
}

pub fn for_loop(){
      let  count =10;
    for x in  0..count  {
        if x %15 ==0{
            println!("fizzbuzz");

        }
        else 
        {
        println!("please try again for loop");
        break;

        }

    }

}