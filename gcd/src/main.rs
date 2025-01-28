use std::io;

// Loop function to return gcd of a and b
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

// Recursive function to return gcd of a and b
fn gcd_recursive(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd_recursive(b, a % b)
    }
}

fn main() {
    let mut str_input = String::new();

    println!("Input the numbers (separate by ' ' e.g. 5 10 15): ");

    io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read line");

    let splited: Vec<&str> = str_input.split_whitespace().collect();

    // gcd of n numbers
    let mut result = splited[0].parse().unwrap();
    for &num in &splited[1..] {
        result = gcd(result, num.parse().unwrap());
    }

    println!("GCD of given numbers is: {}", result);
}
