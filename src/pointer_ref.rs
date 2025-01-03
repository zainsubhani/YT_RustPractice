// Reference pointer == point to resouce in memory

use std::vec;

pub fn pointer_excercise(){
    let arr1 = [1,2,3];
    let arr2=arr1;
    // With non-primitive . if you assign another variable to a piece of data ,the first varialbe will no longer hold that value.
    // you will need to use a reference & to point to the resource .

     println!("{:?}", arr1); // Error: `arr1` no longer valid after the move
    println!("{:?}", arr2);    
}
pub fn pointer_vector(){
    let _vec1=vec![1,2,3];
    let vec2=_vec1;
    // println!("{:?}",(vec2,_vec1));
 //   in this case ownership moved to vector 2 non premitive data type



}
pub fn pointer_vector2(){
    let _vec1=vec![1,2,3];
    let vec2=&_vec1;
    println!("{:?}",(vec2,&_vec1));
 //   in this case ownership moved to vector 2 non premitive data type



}
