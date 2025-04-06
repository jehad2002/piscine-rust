use card_deck::*;

fn main() {
    let your_card = generate_card();
    println!("Your card: {:?}", your_card);

    if winner_card(&your_card) {
        println!("Congratulations! You drew the winning card: Ace of Spades!");
    } else {
        println!("Not the winner card. Better luck next time!");
    }

    // Example test (can be removed or used for checking)
    let test_card = Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    };
    assert_eq!(winner_card(&test_card), true);
}
