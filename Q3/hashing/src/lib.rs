use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    let len = sorted.len();

    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        let mid1 = sorted[len / 2 - 1];
        let mid2 = sorted[len / 2];
        (mid1 + mid2) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut frequency = HashMap::new();
    let mut max_count = 0;
    let mut mode = list[0];

    for &value in list {
        let count = frequency.entry(value).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            mode = value;
        }
    }

    mode
}
