mod structs;
//mod structs;

fn main() {

    let mut greedy_world = structs::World::new(0.0,1920.0,0.0,1080.0);
    greedy_world.add_rand_cities(10);
    greedy_world.salesman_greedy();
    structs::world_to_svg(&greedy_world,"svg/greedy.svg");
    let mut brute_world = structs::World::new_with_cities(0.0,1920.0,0.0,1080.0, greedy_world.get_cities().clone());
    println!("{}", brute_world);
    //brute_world.add_rand_cities(5);
    brute_world.salesman_brute();
    structs::world_to_svg(&brute_world,"svg/brute.svg");

}
