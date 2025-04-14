pub fn prev_prime(nbr: u64) -> u64 {
    if nbr < 2 {
        return 0;
    }

    for c in (2..=nbr).rev() {
        if is_prime(c) {
            return c;
        }
    }

    0
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
