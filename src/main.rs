mod structs;
use structs::City as City;
//mod structs;

fn main() {
    let city = City::new_pos(69.0,420.0);
    let city2 = City::new();
    println!("{}",city);
    println!("{}", city2);
}
