// varialbe hold primite data or reference to data
// varialbe are immutable by default
// rust is a block scope language

pub fn variable(){
    let name = "zain";
    let mut age = 18;
    age =24;
    println!("my name is {} i am {}",name,age);

     

}
pub fn constant(){
    const ID:i32 = 001;
    println!("my id is {}",ID)


}
pub fn multivalue(){
    let (my_name,my_age,my_country)=("Zain",24,"France");
    println!("{} is {} from {}",my_name,my_age,my_country);

}