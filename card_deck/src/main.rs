use card_deck::*;

fn main() {
    let your_card = Card {
        suit: Suit::random(),
        rank: Rank::random(),
    };

    println!("Your card is {:?}", your_card);

    if winner_card(&your_card) {
        println!("You are the winner!");
    }
}
