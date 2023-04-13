mod structs;
//mod structs;

fn main() {
    let city1 = structs::City::new_pos(44.979668, 47.765923);
    let city2 = structs::City::new_pos(33.991623, 204.22545);
    let dist = city1.dist(&city2);
    println!("{}", dist);
    let city1 = structs::City::new_pos(210.54239, 105.37291);
    let city2 = structs::City::new_pos(208.38857, 109.5274);
    let dist = city1.dist(&city2);
    println!("{}", dist);
    //17.620115
    //101.03795
    let mut world = structs::World::new(0.0,400.0,0.0,400.0);
    world.add_rand_cities(10);
    println!("{}",world);
    world.salesman_greedy();
    println!("{}",world);
    structs::world_to_svg(&world,"svg/greedy.svg");
}
