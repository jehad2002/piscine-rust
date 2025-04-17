pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let vec: Vec<u32> = s
        .split_whitespace() 
        .map(|part| {
            if part.ends_with('k') {
                let num = part.trim_end_matches('k').parse::<f32>().unwrap();
                (num * 1000.0) as u32
            } else {
                part.parse::<u32>().unwrap()
            }
        })
        .collect();

    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
*a
}
