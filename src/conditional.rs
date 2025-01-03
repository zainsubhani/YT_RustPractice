pub fn condition_excercise(){
    let age = 21;
    let check_id = false;

  if age>21 && check_id {
    println! (" age is greate than {}",age)
      
  }
  else if age>21 || check_id  {
    println!("sorry your age is less than {}",age);

  }
  else {
        println!("sorry your age is id  than {}",age);

  }

  // Shorthand IF
  let isage = if age >18 {true} else {false};
  println!("{}",isage);

}