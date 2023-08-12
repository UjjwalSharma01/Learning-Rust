#[allow(unused_variables)]
fn main(){

  
  // statement and expressions
   let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x // this is considered to be a return value -
    }; // the entire code inside the curlt braces is considered as an expression because it produces a value and the act of assigning value to the variable y that ends with ";" is called as an statement because it is not producing any value - > it is just assigning the value

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x; // this doesn't assign value to z 
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z); 









  //    let v = {
  //      let mut x = 1;
  //      x += 2; // - > this is an statement as this is assigning value to x and we cannot return a assignment we have to return a value to the variable
  //      x // the return value
  //  };


  // assert_eq!(v, 3);

   let v = {
       let mut x = 1;
       x += 2 // -> the value will not get assigned
   };

   assert_eq!(v, ());

   println!("Success!");







  // ASSIGNING VALUES
  // The wrong way
   // let v = (let x = 3);

   // assert!(v == 3);


  // The correct way

   let v = {let x = 3; // assinging value to x
       x // -> the function is returning x
   };

   assert!(v == 3);







 let s = sum(1 , 2);
    assert_eq!(s, 3);
}
// - > it doesn't matter where you assign the functions in the rust


// the corrected function
fn sum(x: i32, y: i32) -> i32 {
    let z: i32;
    z = x + y;
    z
}

// original function -> this was not returning any value 
// fn sum(x: i32, y: i32) -> i32 {
//     x + y;
// }
