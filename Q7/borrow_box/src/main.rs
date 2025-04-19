use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner()); // ("Same score! tied", 0)

    game.update_score("Joao".to_string());
    game.update_score("Joao".to_string());
    game.update_score("Susana".to_string());
    game.update_score("Susana".to_string());
    println!("{:?}", game.read_winner()); // ("Same score! tied", 2)

    game.update_score("Joao".to_string()); // Joao wins (3 out of 5)
    game.update_score("Susana".to_string()); // ignored

    println!("{:?}", game.read_winner()); // ("Joao", 3)

    println!("{}", game.delete()); // game deleted: id -> 0
}
