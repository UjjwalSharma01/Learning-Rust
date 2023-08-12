/*
REFERENCE AND BORROWING

BORROWING

- way of temporarily access data without taking ownership of it
- when borrowing, you are taking a reference (pointer) to the data and not the data itself
- prevention of dangling pointers and data races
- data can be borrowed immutabily and mutably
- there are certain rules when borrowing, which we have to comply with, otherwise the program won't compile


RULES OF REFERENCES

- at ny given time you can either have one mutable reference or any number of immutablr reference
- references must be always valid



IMPORTANT POINTS
- we can have only one mutuable reference to the data and a given point of time
- SCREENSHOTS
*/

#[allow(unused_variables)]

// QUESTIONS WITH COMMENTS FOR REFERENCES

fn main() {
   let x = 5;
   // Fill the blank
   let p = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}







fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}





// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}





// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}







fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}








fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}



// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}", r2); // if i use r1 then compiler will say - > hey you are using two references at one

    println!("Success!");
}





fn main() {
    // Fix error by modifying this line
    let mut  s = String::from("hello, "); // should be mutable for mutable referencing

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}



// Comment one line to make it work
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1); we cannot use more than one mutable references at a time if we remove this then we are using r2 only at the given time and the work of r1 is ended after the next line so it won't give any error
}








fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
  print!("{}, {}", r1, r2) //will throw an arror because we cannot use them in a single time
}
