use std::time::Instant;
mod structs;
//mod structs;

fn main() {

    let mut greedy_world = structs::World::new(0.0,1920.0,0.0,1080.0);
    greedy_world.add_rand_cities(20);
    println!("{}", greedy_world);
    println!("Running greedy algorithm...");
    let now = Instant::now();
    greedy_world.salesman_greedy();
    let elapsed = now.elapsed();
    println!("Greedy algorithm finished with path length {}", greedy_world.sum_dist());
    println!("Greedy algorithm finished running in {:.2?}", elapsed);
    structs::world_to_svg(&greedy_world,"svg/greedy.svg");
    let mut brute_world = structs::World::new_with_cities(0.0,1920.0,0.0,1080.0, greedy_world.get_cities().clone());
    println!("Running brute force algorithm...");
    let now = Instant::now();
    brute_world.salesman_brute();
    let elapsed = now.elapsed();
    println!("Brute force algorithm finished with path length {}", brute_world.sum_dist());
    println!("Brute force algorithm finished running in {:.2?}", elapsed);
    structs::world_to_svg(&brute_world,"svg/brute.svg");

}
