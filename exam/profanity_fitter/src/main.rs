// use profanity_fitter::*;

// fn main() {
//     ["hello there", "", "you are stupid", "stupid"]
//         .into_iter()
//         .for_each(|m| println!("{:?}", check_ms(m)));
//     }

use profanity_fitter::*;

fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", profanity_fitter::check_ms(m)));
      }
