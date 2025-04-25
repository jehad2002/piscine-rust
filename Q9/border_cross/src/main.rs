mod vehicle;
use vehicle::{Car, Truck, Vehicle, all_models};

fn main() {
    let car = Car {
        plate_nbr: "123ABC",
        model: "Toyota",
        horse_power: 150,
        year: 2010,
    };

    let truck = Truck {
        plate_nbr: "456DEF",
        model: "Volvo",
        horse_power: 400,
        year: 2015,
        load_tons: 10,
    };

    let vehicles: Vec<&dyn Vehicle> = vec![&car, &truck];

    println!("{:?}", all_models(vehicles));
}
