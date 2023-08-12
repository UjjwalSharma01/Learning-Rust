// OWNERSHIP - > THE MOST DIFFICULT PART OF RUST
/*
nothing but a set of rules that governs memory management
rules are enforced at compile time
if any rule is violated the program won't compile


THREE RULES OF OWNERSHIP
1. each value in rust has an owner
2. there can only be one owner at a time
3. when the owner goes out of scope, the value will be dropped


OWNER - > the owner of a value is the vriable or data structure that holds it and is responsible for allocating and freeeing the memory used to store data


SCOPE
range within a prgram for which an item is valid

GLOBAL SCOPE
- accessible throughout the entire program


LOCAL SCOPE
- accessible within particular function or block of code 
- not accessible outside of that function or block


SCOPE KE BAHAR CONTROL JAATE HI JO MEMORY USE KR RHA H VO VARIABLE OR THE MEMORY SPACE IN THE VERY PRECISE MANNER- > VO KHALI HO JAEGA



Memory 

- Component in a computer used to store data and instructions for the processor to execute
- Random access memory (RAM) is volatile when power is turned off all contents are lost
two types of region is the memory used by program at runtime : stack memory and heap memory


STACK - > LIFO - > last in first out

HEAP MEMORY
- data of no known, fixed size belongs on the heap
- allocating data on the heap will aleays return an pointer (address to the memory location where the data is being stored)
- allocating on the heap is slower than pushing to stack
- access data on the heap is also slower as it has to be accessed by the pointer which points to an address - > pehle pointer execute hoga , fir dereferencing hogi fir value return hogi and print hogi , whereas in stack direct variable execute hoga and it will return the value
- size is not fixed, usually used when the type of data is not known at the compile time


EXAMPLE

THE STRING TYPE
- all types covered so far were fixes size
- string is mutable
- sting size can be changed at runtime
- sting stores on the stack with a pointer to the heap
- > tumhara jo stack memory ke liye pointer bnega vo stack main stored hoga and the real data will be stored in the heap itself
 - value of string is stored on the heap




                                  COPY VS MOVE
- scaler values with fixed sizes (all types are covered in the beginning - > int, bool, char,float) will automatically get copied in the stackm copying here is cheap
- Dynamically sized data won't get copied but moved, copying would be too expensive
-> SCREENSHOT
but for more simpler understanding it's just passed by value and passed by reference kinda thing -> herw when we do let x =10; let y = x; -> the copy of the value of x is stored in the y but when i do same with the string, the pointer because it is stored in the stack memory get's copied and not the entire data




let s1 = string::from("hello");
let s2 = s1;

the pointer s1 is coped to s2 and they both are pointing to the same location in the memory - > this violates the second law of owenership that there can only be one owner for the given data


IMPORTANT
-> the rust will drop the s1 and s2 will be the real owner now



DEEP COPY  - > when you want to copy the actual data allocated in the heap memory
we have to use
s2 = s1.clone();



********************************************************************************


NEED OF THE CONCEPT - > OWNERSHIP

PREVENTING ISSUES
- ownership prevents memory saftey issues
- dangling pointers
- Double free - > trying to free memory that has been already freed
- Memory Leaks - > Not freeing memory that should have been freed 

********************************************************************************
 POINTS TO REMEMBER 
- > always check if thr function is returning something
- > the "s" of the datatype string is always a captial word
- > even when the string is passed through the function the function becames the owner of the string and not the actual varriable so better use. string.clone() function while passing that into a function unline c++








THE DIFFERENCE BETWEEN CLONING AND COPYING 
- copying is done for the variables stored in stack i.e static or scaler variables
- clone can copy even the dynamic datatypes 
- to make a string scaler datatype > we use &str instead of String
for cloning we use variable.clone() and for copying we directly copy the value
like x= 1;
y =x;




DYNAMIC ALLOCATION

let x = Box::new(5); //- > this will allocate the integer in the heap memory and the 

datatype of the pointer in rust
box<i32>





*/

#[allow(unused_variables)]

// fn main() {
//   // Problem 1
//     // Use as many approaches as you can to make it work
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}",x,y);

  // problem 2 
  // Don't modify code in main!
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1); // in the first place the function is not returning anything so we have to generate a line of code for the return function

//     println!("{}", s2);
// }

// }







// problem 3
// // initial problem
// fn main() {
//     let s :String = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // Convert String to Vec
//     let _s = s.into_bytes(); // consumes the original string values so the string cannot be used anymore
//     s
// }
// // final code 
// fn main() {
//     let s :String = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // Convert String to Vec
//     let _s = s.as_bytes(); // it won't consume the string
//     s
// }



















// Only modify the code below!
fn take_ownership(s: String) -> String { // "s" of string should be in capital letter
    println!("{}", s);
    s
}





// WARNING - > ONE CONCEPT LEFT LAST 2 QUESTIONS - > WILL DO AFTER STRUCT
