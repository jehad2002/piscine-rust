use roman_numbers::RomanNumber;

fn main() {
    println!("{:?}", RomanNumber::from(32)); // RomanNumber([X, X, X, I, I])
    println!("{:?}", RomanNumber::from(9));  // RomanNumber([I, X])
    println!("{:?}", RomanNumber::from(45)); // RomanNumber([X, L, V])
    println!("{:?}", RomanNumber::from(0));  // RomanNumber([Nulla])
}
