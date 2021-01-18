pub fn run() {
    another_function(2);
    let x = five();
    println!("The value of x is: {}", x);
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1 // no semicolon here as it will become a statement instead of expression!
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}