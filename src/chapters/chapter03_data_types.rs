pub const MAX_POINTS: u32 = 100_000;

pub fn run() {
    let mut y = 5 + MAX_POINTS;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // https://stackoverflow.com/questions/4971286/javas-final-vs-cs-const
    // https://stackoverflow.com/questions/14116003/difference-between-constexpr-and-const
    let spaces = "   ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // https://en.wikipedia.org/wiki/Two%27s_complement
    // https://en.cppreference.com/w/cpp/language/ub
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let a = [1, 2, 3, 4, 5];
    println!("The value of element is: {}", a[10]);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of element is: {}", tup.0);
}