// lib.rs

// Define all the structs here
pub struct Mall {
    pub name: String,
    pub stores: Vec<Store>,
    pub guards: Vec<Guard>,
}

pub struct Store {
    pub name: String,
    pub employees: Vec<Employee>,
}

pub struct Employee {
    pub name: String,
    pub position: String,
}

pub struct Guard {
    pub name: String,
    pub shift: String,
}

// Example function that returns a dummy mall
pub fn create_sample_mall() -> Mall {
    Mall {
        name: String::from("Sunset Plaza"),
        stores: vec![
            Store {
                name: String::from("Tech World"),
                employees: vec![
                    Employee {
                        name: String::from("Alice"),
                        position: String::from("Cashier"),
                    },
                    Employee {
                        name: String::from("Bob"),
                        position: String::from("Manager"),
                    },
                ],
            },
            Store {
                name: String::from("Fashion Hub"),
                employees: vec![
                    Employee {
                        name: String::from("Carol"),
                        position: String::from("Stylist"),
                    },
                ],
            },
        ],
        guards: vec![
            Guard {
                name: String::from("Dave"),
                shift: String::from("Morning"),
            },
        ],
    }
}
