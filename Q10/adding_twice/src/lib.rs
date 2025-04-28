pub fn add_curry(num: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x: i32| x + num)
}

pub fn twice<T: Clone + std::ops::Add<Output = T> + 'static>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> {
    Box::new(move |x| f(f(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_curry() {
        let add_five = add_curry(5);
        assert_eq!(add_five(10), 15);
        assert_eq!(add_five(3), 8);
    }

    #[test]
    fn test_twice() {
        let add_two = add_curry(2); // x + 2
        let add_four = twice(add_two); // (x + 2) + 2

        assert_eq!(add_four(3), 7); // 3 + 2 = 5 -> 5 + 2 = 7
        assert_eq!(add_four(10), 14); // 10 + 2 = 12 -> 12 + 2 = 14
    }
}
