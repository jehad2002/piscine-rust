use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    
    let mut trials = 0; 

    loop {
        println!("{}", riddle); 

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        let answer = answer.trim();

        trials += 1; 

        if answer == correct_answer {
            println!("Number of trials: {}", trials); 
            break; 
        } else {
            println!("Incorrect answer. Try again!"); 
        }
    }
}
