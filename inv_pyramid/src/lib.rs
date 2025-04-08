fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();
    
    // الجزء العلوي من الهرم (التوسع)
    for j in 1..=i {
        let spaces = " ".repeat((i - j) as usize);
        let symbols = v.repeat(j as usize);
        result.push(format!("{}{}", spaces, symbols));
    }

    // الجزء السفلي من الهرم (التقليص)
    for j in (1..i).rev() {
        let spaces = " ".repeat((i - j) as usize);
        let symbols = v.repeat(j as usize);
        result.push(format!("{}{}", spaces, symbols));
    }

    result
}

fn main() {
    let a = inv_pyramid(String::from("#"), 1);
    let b = inv_pyramid(String::from("a"), 2);
    let c = inv_pyramid(String::from(">"), 5);
    let d = inv_pyramid(String::from("&"), 8);

    for v in a.iter() {
        println!("{:?}", v);
    }
    for v in b.iter() {
        println!("{:?}", v);
    }
    for v in c.iter() {
        println!("{:?}", v);
    }
    for v in d.iter() {
        println!("{:?}", v);
    }
}
