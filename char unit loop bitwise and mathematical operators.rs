#[allow(unused_variables)]
use std::ops::{Range, RangeInclusive}; // for using loops in the form of range
use std::mem::size_of_val; // to find the size of the variables
fn main(){

  //******************************************************************
  // the LOOPS

  
  assert_eq!((1..5), Range{ start: 1, end: 5 });
  assert_eq!((1..=5), RangeInclusive::new(1, 5)); // here 5 will be included 




  // ************************************************************************************
      // Different operations
      // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2f32 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);


  
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); //XOR opertor same number gives 0 as the output and different numbers gives one as the output
    println!("1 << 5 is {}", 1u32 << 5); // it will become 32 after shifting
  // how it works?
  // write the binary code for 1 and then shift it to 5 positions it will become 2^5 and will give 32 as the output
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);























  // Char, Bool and Unit
  /*
  size of the character in rust is 4 
  -> and to find the size of any variable we use size_of_val(&<variable name>) function and we pass the function by address
  
  
  */
 let c1 = 'a';
  assert_eq!(size_of_val(&c1),4); 

  let c2 = '中';
  assert_eq!(size_of_val(&c2),4); 



  // unit type - > unit type is a type which doesn't hold any value it's size is 0 bytes - > if a function doesn't return anything then it returns a unit type and this happens impicitly

  
}
