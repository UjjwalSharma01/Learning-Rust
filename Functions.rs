#[allow(unused_variables)]
fn main(){
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}
// modified function
fn sum(x:i32, y: i32) -> i32 { // we explicity have to define the return type using "-> i32" and we also have to define the datatype of the parameters of the function
    x + y
}

// original function
// fn sum(x, y: i32) {
//     x + y;
// }



// DIVERGING FUNCTION
fn never_return() -> ! { // this is a diverging function that means it won't ever return to the main function and causes panic in the program
    // Implement this function, don't modify the fn signatures

  // you can also add one macro here
  panic!()
    
}


// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp { // -> just like switch case
//         1 => {
//             // TODO 
//         }
//         _ => {
//             // TODO
//         }
//     };
    
//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!() 
//     // unimplemented!()  or todo!()
// }


// fn main() {
//     // FILL in the blank
//     let b = false;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value of any value
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic");
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }

