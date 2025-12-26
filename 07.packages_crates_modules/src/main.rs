use packages_crates_modules::Car;

// 05. USING PATHS
fn main() {
    // Absolute Path
    // Starts from the crate root (virtual filesystem root)
    packages_crates_modules::factory::management::report_status();

    packages_crates_modules::factory::production_floor::assemble_car();

    // Struct Usage 
    let mut my_car = Car::new(String::from("Sedan"));

    // We can read/change the public field 
    my_car.model = String::from("Suv");
    println!("model: {}", my_car.model);

    // ERROR: We cannot access the private field
    // println!("{}", my_car.serial_number);
}

