mod structs;
//mod structs;

fn main() {
    let mut world = structs::World::new(-1000.0,0.0,-1000.0,0.0);
    world.add_rand_cities(1000);
    let greedy = structs::sum_dist(&world.salesman_greedy());
    structs::world_to_svg(&world,"svg/greedy.svg");
    println!("{}",world);
    println!("{}", greedy);
}
