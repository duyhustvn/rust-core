// 01. MODULE DEFINITION
pub mod factory {
    // By default, modules are private. We add pub so main_office can see it
    pub mod production_floor {
        // this function is public; anyone with access to 'production_floor' can call it 
        pub fn assemble_car() {
            println!("Car assembled!");
            install_engine(); 
        }

        // this function is private. Only items inside 'production_floor' can call it 
        fn install_engine() {
            println!("Engine installed");
        }
    }

    pub mod management {
        pub fn report_status() {
            println!("Everything is running smoothly");

            // 02. THE 'SUPER' KEYWORD
            // We are inside 'management'. We want to call 'assemble_car' in 'production_floor'
            // 'super' goes up one level to 'factory', then down to 'production_floor'
            super::production_floor::assemble_car();
        }
    }
}

// 03. PUBLIC STRUCTS (Private Fields)
pub struct Car {
    pub model: String, // public field: accessible from anywhere
    serial_number: u32,
}

impl Car {
    // Because 'serial_number' is private, we MUST provide a public constructor method
    // to create an instance of Car 
    pub fn new(model: String) -> Car {
        Car {
            model,
            serial_number: 10101, // We can access private fields inside the implementation
        }
    }
}

// 04. PUBLIC ENUMS (All Varians Public)
// if the Enum is pub, you dont need to pub for Red, Blue, etc.
pub enum Color {
    Red,
    Blue,
}
