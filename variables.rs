// fn main() {
//     // excution starts from the main function
//     println!("ujjwal sharma"); //println means print line
// }

// VARIABLES

/*
- > Assigned using "let keyword"
Print to standard output by print!() or println!()
scope of a variable defined by the block of the code in which it is declared
functin is a named block of code which is resuable
- > shadowingt allows a variable to be re decalred in the same scope with the same name


- > be default all the variables are treated as constants we have to expilcityl define if we want to change the values of he variable as "MUTABLE" using the "mut" keyword

*/
#[allow(unused_variables)]
fn main() {
    let x = 5;
    let y = 12; // this will show an error if i donot use this variable saying this is the unused variable so wer have to use the prefix _VARIABLE NAME in order to remove that warning during the execution

    assert_eq!(x, 5); // this means assert equal - > this checks if the value of the variable is equal to the provided value
    print!("success");

    let mut a = 3;
    a = a + 4;
    assert_eq!(a, 7);
    print!("Sucessfull");

    // printing different thing with variables in rust

    print!("the value of x is {} and the value of y is {} ", x, y);

    // **************************************************************************************

    /*
                                  CONCEPT OF SHADOWING
    You can declare a new variable with the same name as a previous variable, here we can say   the first one is shadowed by the second one.


     - > if a variable was mutable and you shadowed it, it will again become immutable, so you again have to define whether it's mutable and immutable

    */

    let z = 5;

    let z = 8;
    print!("{}", z);

    // ****************************************************************************************

    // removing warnings of undused variables
    // there aret two methods ,  wither use "_" before the variable name or use #[allow(unused_variables)]


  // ****************************************************************************************








  // DESTRUCTING 


  
// Fix the error below with least amount of modification

    let (mut x, y) = (1, 2); // we can also define x and y like this
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");


  // another way

    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");


}
