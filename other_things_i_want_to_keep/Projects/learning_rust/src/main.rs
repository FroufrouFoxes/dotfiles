// scalar types:
// int, float, bool, & char
//
// Rust's floating-point types are f32 & f64
// all float are signed
//
// booleans can be either true or false
// main way to use them is through conditionals
// such as an if expression
//
// char literals are specified with single qotes
// as opposed to string literals which use double quotes
// cahr is 4b in size and represents a Unicode scalar value
//
// compound types group multi values into one type
// rust has two prim types:
// tuples and arrays
//
// tuples have a fixed-length: once declared, they cannot grow or shirnk
// it's created through comma-separated list of values inside parentheses
//
// you can access a tuple element directly using a period followed by
// the index of the balue we want to access
// tuple w/o any values is a unit 
//
// an array, unlike a tuple, must have every element remain the same type
// arrays have a fixed length
// it's created through comma-separated list inside brackets
//
// arrays are useful when you want your data allocated on the stack
// rather than the heap and to ensure you have a fixed # of elements
//
// a vector is a similar collection type that is allowed to grow or shrink
// because its contents live on the heap
//
// you write an arrays type using brackets with the type of each element, 
// a semicolon, and the number of elements in the array
// you can also initialize an array to contain the same value for each element
// by specifying the value, followed by a ; and then the length of the array
//
// an array is a single chunk of memory of a known, fixed size that can be
// allocated on the stack
// it can be accessed through indexing like 'a[0];' and so on
//

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

// the main error message, mismatch types, relveals the core issue with this code.
// the definition of the function plus_one says that it will return an i32, but statements
// don't evaluate to a balue, which is expressed by (), the unit type. 
// therefore, nothing is returned, which contradicts the function definition
// and results in an error.
//
// in Rust, c-brace block is an expression and a syntactic scope
