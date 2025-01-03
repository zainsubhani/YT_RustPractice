. Primitive Data Types
Primitive data types are the basic building blocks provided by Rust. They are simple and efficient, often stored on the stack. These types implement the Copy trait, meaning their values are copied rather than moved when assigned to another variable.

Characteristics:
Fixed size in memory.
Stored on the stack.
Implement the Copy trait (no ownership transfer).
No special methods; direct operations are performed.
Examples:
Scalar Types:

Integer Types: i8, i16, i32, u8, u16, etc.
Floating-Point Types: f32, f64.
Character: char.
Boolean: bool.
Example:

rust
Copy code
let x = 5; // Integer type
let y = x; // `x` is copied to `y`
println!("x: {}, y: {}", x, y); // Both `x` and `y` are valid
Compound Types:

Tuples: Fixed-size collections of multiple types.
Arrays: Fixed-size collections of the same type.
Example:

rust
Copy code
let tup: (i32, f64, u8) = (500, 6.4, 1);
let arr = [1, 2, 3, 4, 5]; 2. Non-Primitive Data Types
Non-primitive types are more complex and often involve heap allocation. These types are not copied by default; instead, ownership is moved when assigned to another variable or passed to a function.

Characteristics:
May involve heap allocation.
Stored partially on the stack (metadata) and heap (actual data).
Ownership rules apply; values are moved unless references (&) are used.
Often have associated methods and implement the Drop trait to manage cleanup.
Examples:
String:

Dynamic, growable text stored on the heap.
String vs. &str: String is heap-allocated; &str is a slice.
Example:

rust
Copy code
let s1 = String::from("hello");
let s2 = s1; // Ownership moves to `s2`
// println!("{}", s1); // Error: `s1` is no longer valid
println!("{}", s2); // Works: `s2` owns the data
Vector (Vec<T>):

A growable, heap-allocated list.
Example:

rust
Copy code
let v1 = vec![1, 2, 3];
let v2 = v1; // Ownership moves to `v2`
// println!("{:?}", v1); // Error: `v1` is no longer valid
println!("{:?}", v2); // Works
User-Defined Types:

Structs, Enums, Traits.
Comparison Table
Feature Primitive Types Non-Primitive Types
Memory Location Stack Stack (metadata) + Heap
Ownership Not applicable (copied) Ownership rules apply
Implements Copy Trait Yes No
Complexity Simple Complex
Examples i32, f64, bool String, Vec<T>, struct
Summary
Primitive types are lightweight, fixed in size, and are copied when assigned or passed.
Non-primitive types are heap-allocated, involve ownership and borrowing rules, and require careful memory management in Rust.
