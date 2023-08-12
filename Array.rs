/*
ARRAY
- fixed size collection of elements of same data type stored as conigout block in stack memory
 - signature of array is [T; length] which indicated that the length is fixed at the compile time - > you cannot define it's length later on
- array can neither grow nor shrink, they must retain their size - > the size is fixed


IMPORTANT POINTS

DECLARATION OF ARRAY
- the array can be initialzed using the following syntax
let arr: [i32 ; 5] = [1, 2, 3, 4, 5];
- another way for declaration of array let arr0 = [1, 2, 3];  and let arr: [_; 3] = ['a', 'b', 'c'];
- can can find length using the following syntax
arr.len();
- INITIALISATION OF ARRAY WITH SOME PARTICULAR VALUE
you can use the following code to do so
let list: [i32; 100] = [1;100] ; - > it will create 100 elements with value 1

ACCESSING THE ELEMENTS OF ARRAY
you can access the elements using the following code
list[0];  - > the better method is using "get" keyword like the code example below
 // `Get` returns an Option<T>, it's safe to use - > will be discussed later
    let name0 = names.get(0).unwrap();

ACCESING STRING IN ARRAY
&arr[index] - > use it like this
*/






fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
  

    println!("Success!");
}
