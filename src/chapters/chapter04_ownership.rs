pub fn run() {
    let s = String::from("hello");
    let mut s = String::from(s);
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    let mut owned_string: String = "hello ".to_owned(); // same as String::from("hello")
    let borrowed_string: &str = "world";
    owned_string.push_str(borrowed_string);
    owned_string.push_str("!");
    println!("{}", owned_string);

    let borrowed_string: &str = "hello ";
    let another_borrowed_string: &str = "world";
    let together = format!("{}{}", borrowed_string, another_borrowed_string);
    println!("{}", together);

    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();
    println!("{}, world!", s1);

    let s = String::from("hello");   // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    // s.clone();                             // does not work
    // ... and so is no longer valid here

    let x = 5;                           // x comes into scope
    makes_copy(x);                 // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

    run2();
}

fn takes_ownership(some_string: String) {    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {           // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

pub fn run2() {
    let s1 = gives_ownership();                // gives_ownership moves its return
    // value into s1
    let s2 = String::from("hello");         // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}