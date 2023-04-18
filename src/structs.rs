use std::fmt;
#[derive(Clone)]
pub struct City {
    x: f32,
    y: f32,
}
impl Eq for City{}
impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "City at {}, {}", self.x, self.y);
    }
}
impl City {
    pub const fn new_pos(x: f32, y: f32) -> Self {
        return City{x, y};
    }
    pub fn dist(&self, other: &City) -> f32 {
        let deltax = self.x - other.x;
        let deltay = self.y - other.y;
        return (deltax*deltax+deltay*deltay).sqrt();
    }
}
pub struct World {
    cities: Vec<City>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}
impl World {
    pub const fn new(l: f32, r: f32, b: f32, t: f32,) -> Self {
        return World {
            cities: Vec::new(),
            left: l,
            right: r,
            bottom: b,
            top: t,
        }
    }
    pub const fn new_with_cities(l: f32, r: f32, b: f32, t: f32, c:Vec<City>) -> Self {
        return World {
            cities: c,
            left: l,
            right: r,
            bottom: b,
            top: t,
        };
    }
    pub fn add_rand_cities(&mut self, n:u32){
        self.cities.append(&mut rand_cities(n, self.left, self.right, self.bottom, self.top));
        self.cities.shrink_to_fit();
    }
    pub fn get_cities(&self) -> &Vec<City> {
        return &self.cities;
    }
    pub fn sum_dist(&self) -> f32 {

        let mut acc: f32 = 0.0;
        for i in 1..self.cities.len() {
            acc += self.cities[i-1].dist(&self.cities[i]);
        }
        return acc;
    }
    //greedy nearest neighbor algorithm starting at first city
    pub fn salesman_greedy(&mut self) -> &Vec<City> {
        if self.cities.len() == 0 {
            return &self.cities;
        }
        let mut sol:Vec<City> = Vec::with_capacity(self.cities.len());
        sol.push(self.cities.remove(0));
        while self.cities.len() > 0 {
            let mut mindist = f32::MAX;
            let mut index = usize::MAX;
            //find closest city to last city in path
            let last = sol.last().unwrap();
            for i in 0..self.cities.len() {
                let next = &self.cities[i];
                let distance = last.dist(next);
                if distance < mindist {
                    index = i;
                    mindist = distance;
                }
            }
            //add closest city
            sol.push(self.cities.remove(index));
        }
        sol.shrink_to_fit();
        self.cities = sol;
        return &self.cities;
    }
    //brute force algorithm that can start at any city
    pub fn salesman_brute(&mut self) -> &Vec<City> {
        let sol = self.brute_helper(Vec::new(), 0.0).0;
        self.cities = sol;
        return &self.cities;
    }
    fn brute_helper(&self, list:Vec<City>, acc:f32) -> (Vec<City>, f32) {
        //if list is a complete traversal return it
        if list.len() == self.cities.len() {
            return (list, acc);
        }
        let mut ret = (Vec::new(), f32::MAX);
        for c in self.cities.iter() {
            //check if city has been visited
            let mut visited = false;
            for visited_city in &list{
                if c == visited_city{
                    visited = true;
                    break;
                }
            }
            if !visited {
                //copy values to new variables
                let mut clone = list.clone();
                clone.push(c.clone());
                let mut acc_copy = acc;
                if list.len() > 0 {
                    acc_copy += list.last().unwrap().dist(c);
                }
                //call self on next city c
                let res = self.brute_helper(clone, acc_copy);
                //use results if they are the best so far
                if res.1 < ret.1 {
                    ret = res;
                }
            }
        }
        //return best solution
        ret.0.shrink_to_fit();
        return ret;
    }
    //ant algorithm makes a pheromone map, sends out randomized ant agents that use pheromone map to
    // randomly select a path. n is the number of ant agents used. Starts at first city.
    pub fn salesman_ant(&mut self, n:u32) -> &Vec<City> {
        //pheromone map
        let mut pheromone: Vec<Vec<f32>> = Vec::new();

        //init pheromone graph with distances
        for i in 0..self.cities.len() {
            pheromone.push(Vec::new());
            for j in 0..self.cities.len() {
                pheromone[i].push(self.cities[i].dist(&self.citis[j]));
            }
        }
        //send out n ants
        for _i in 0..n {
            let ant = ant_helper(&pheromone);
            //update pheromones
            let p_factor:f32 = 1/ant.1;
            for i in 1..ant.0.len() {
                pheromone[ant.0[i-1]][ant.0[i]] += p_factor;
            }
        }


        return &self.cities;
    }
    //send out one ant, return its dist and city traversal
    fn ant_helper(&self, pheromone: &Vec<Vec<f32>>) -> (Vec<usize>, f32) {
        use rand::*;
        let mut rng = thread_rng();
        let mut traversal = Vec::new();
        let mut dist = 0.0;
        traversal.push(0);
        while traversal.len() < self.cities.len() {
            let line = pheromone[traversal.last().unwrap()];
            //sum pheromones out of city
            let mut sum_p:f32 = 0.0;
            for i in line.len() {
                if !traversal.contains(i) {
                    sum_p += line[i];
                }
            }
            //select randomly with weights
            let mut rval = rng.gen_range(0.0..sum_p);
            let mut i:usize = 0;
            loop {
                rval -= line[i];
                if rval > 0.0 {
                    i += 1;
                }else {
                    break;
                }
            }
            dist += self.cities[traversal.last().unwrap()].dist(&self.cities[i]);
            traversal.push(i);
        }
        return (traversal, dist)
    }
}
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str("Range X: ");
        str.push_str(&self.left.to_string());
        str.push_str(" to ");
        str.push_str(&self.right.to_string());
        str.push_str("\nRange Y: ");
        str.push_str(&self.bottom.to_string());
        str.push_str(" to ");
        str.push_str(&self.top.to_string());
        str.push_str("\n");

        for c in &self.cities {
            str.push_str(&c.to_string());
            str.push_str("\n");
        }
        return write!(f, "{}", str);
    }
}
fn rand_cities(n: u32, left: f32, right: f32, bottom: f32, top: f32) -> Vec<City> {
    use rand::*;
    let mut rng = thread_rng();
    let mut arr = Vec::with_capacity(n as usize);
    for _i in 0..n {
        arr.push(City::new_pos(rng.gen_range(left..right), rng.gen_range(bottom..top)));
    }
    arr.shrink_to_fit();
    return arr;
}
pub fn world_to_svg(w:&World, path:&str) {
    use draw::*;
    let width = (w.right - w.left) as u32;
    let height = (w.top - w.bottom) as u32;
    let mut size = width;
    if width > height {
        size = height;
    }
    let mut path_size = size / 400;
    if path_size == 0 {
        path_size = 1;
    }
    let mut city_size = size / 200;
    if city_size == 0 {
        city_size = 2;
    }
    //println!("{} {}", path_size,city_size);
    let mut canvas = Canvas::new(width, height);
    //add paths between cities
    for i in 1..w.cities.len() {
        let ln = Drawing::new()
            .with_shape(
                LineBuilder::new(w.cities[i-1].x - w.left, w.cities[i-1].y - w.bottom)
                    .line_to(w.cities[i].x - w.left, w.cities[i].y - w.bottom)
                    .build(),
            )

            .with_style(Style::stroked(path_size, RGB::new((200*i / w.cities.len()) as u8 + 55,0,0)));
        canvas.display_list.add(ln);
    }
    //add cities
    for c in &w.cities {
        let pt = Drawing::new()
            .with_shape(Shape::Circle{
                radius: city_size,
            })
            .with_xy(c.x - w.left, c.y - w.bottom)
            .with_style(Style::filled(Color::black()));
        canvas.display_list.add(pt);
    }
    render::save(
        &canvas,
        path,
        SvgRenderer::new(),
    ).expect("Failed to save");
}