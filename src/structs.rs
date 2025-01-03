// Struct used to create a custom data type
// Traditional struct
struct colors (u16,u16,u16);

struct Color {
    red: u8,
    green: u16,
    blue: u32,
}
struct Person{
    first_name:String,
    last_name:String
}
impl Person {
    fn new(first:&str,last:&str)->Person{
        Person{
            first_name:first.to_string(),
            last_name:last.to_string()
        }

    }
    
}
pub fn struct_excercise2(){
    let mut p =Person::new("john","Sss" );
    println!("{}{}",p.first_name,p.last_name);
    
}

pub fn struct_excercise() {
    let mut c = Color {
        red: 255,
        green: 300,
        blue: 200,
    };
    c.red = 10; // Modify the `red` field
    println!("Color: {} {} {}", c.red, c.green, c.blue);
}
// tuple Struct

pub fn tuple_struct(){
    let mut c =colors(0,12,200);
        println!("Colors value tuple : {} {} {}", c.0, c.1, c.2);


}
