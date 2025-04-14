// main.rs

use shopping_mall::{create_sample_mall, Mall, Store, Employee, Guard};

fn main() {
    let mall = create_sample_mall();

    println!("Welcome to {}", mall.name);

    for store in mall.stores {
        println!("Store: {}", store.name);
        for employee in store.employees {
            println!("  Employee: {} ({})", employee.name, employee.position);
        }
    }

    for guard in mall.guards {
        println!("Guard on duty: {} ({})", guard.name, guard.shift);
    }
}
