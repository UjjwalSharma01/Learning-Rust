/*
STRING vs &str (slice string)
-  string is a heap-allocated string type that owns it's contents and is mutable
- a &str is immutable sequence of UTF-8 bytes in memory and it doesn not own the underlyinh data and is immutable
- think of &str as a view on sequence of characters (stored as UTF -8 bytes) in memory
- use &str if you want to view a string
- &str is more lightweight and efficient than string
 - use string if you need to own the data and be able to mutate it

 STRING LITERAL
 - screenshot



 IMPORTANT POINTS
 - to inset a string in String datatype we use s.push_str("") function
 - to replace the words in the string we can use s.replace("word", "with whom") function
 - You can only concat a String with &str, and String's ownership can be moved to another variable, first argument should be of string type and other should be of &str type for contatinaton
- CONVERSION of - &str to string
1. by using s.to_string();
2. String::from(s);
- String escapes -> You can use escapes to write bytes by their hexadecimal values example -  let byte_escape = "I'm writing Ru\x73__!"; - > the x73 will give the value stores at this hexadecimal place which is s in this case
- Acessing members of the string
Syntax - > &s.[1..range];  or &s.[star..end];
- UTF - 8  printing - > chars() function puts the iterator to iterate on every single character of the string




*/



// some examples for better understanding
// We can only use str by boxed it, & can be used to convert Box<str> to &str

// solution 1 

// Fix the error with at least two solutions
fn main() {
    let s: &str = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}




// solution 2

// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}




// INITLIZATION
// &str = "";


// Fill the blank
fn main() {
    let mut s = String::from("");
    s.push_str("hello, world"); // we use push_str to push a string
    s.push('!'); // normal push to push a character

    assert_eq!(s, "hello, world!");

    println!("Success!");
}









// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world"); // we have to use push_str because we are inserting a string into it
    s += "!";

    println!("{}", s);
}



//ORIGINAL PROBLEM

// Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string(); // we have to pass a string literal because it is defined in the standard library

    println!("{}", s);
}











// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}


// original problem 

// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.__("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}



// CONCATINATION

// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2: &str = "world!";
    let s3 : String = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}


//ORIGINAL CODE

// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}






// PROBLEM


// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}





// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s.as_str();

    println!("Success!");
}









// CONVERSION PROBLEM



// PROBLEM


// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}





// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}


// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(String::from(s))
}

fn greetings(s: String) {
    println!("{}", s)
}







// ACCESSING VALUES


fn main() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}



// original problem


fn main() {
    let s1 = String::from("hi,中国");
    let h = s1[0]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}



// UTF-8 STRING


fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() { // puts the iterator to iterate on every single character of the string
        println!("{}", c)
    }
}
