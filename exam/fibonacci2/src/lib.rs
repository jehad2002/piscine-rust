pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
