use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let c = b;
        b = a % b;
        a = c;
    }
    return a;
}

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();

    println!("Input the first number: ");

    io::stdin()
        .read_line(&mut a_str)
        .expect("Failed to read line");

    println!("Input the second number: ");

    io::stdin()
        .read_line(&mut b_str)
        .expect("Failed to read line");

    let a: i64 = a_str.trim().parse().unwrap(); // str->i64
    let b: i64 = b_str.trim().parse().unwrap(); // str->i64

    // gcd of n numbers
    let result = gcd(a, b);

    println!("GCD of given numbers is: {}", result);
}
