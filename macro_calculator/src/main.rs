use macro_calculator::*;

fn main() {
    let a = Food {
        name: "big mac".to_owned(),
        calories: ["2133.84kJ".to_owned(), "510kcal".to_owned()],
        proteins: 27.0,
        fats: 26.0,
        carbs: 41.0,
        nbr_of_portions: 2.0,
    };

    let b = Food {
        name: "pizza margherita".to_owned(),
        calories: ["1500.59kJ".to_owned(), "358.65kcal".to_owned()],
        proteins: 13.89,
        fats: 11.21,
        carbs: 49.07,
        nbr_of_portions: 4.9,
    };

    // Directly pass the Vec<Food>, not a reference to it
    let macros = calculate_macros([a, b].to_vec());

    // You can print the results here
    println!("{:#}", macros);
}
