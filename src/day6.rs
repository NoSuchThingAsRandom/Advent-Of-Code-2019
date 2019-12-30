use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Orbit {
    base: String,
    planet: String,
}

fn count(
    current_depth: usize,
    current_planet: &String,
    orbits: &HashMap<String, Vec<String>>,
) -> usize {
    match orbits.get(current_planet) {
        Some(children) => {
            let mut total: usize = children.len() + children.len() * current_depth;
            for child in children {
                total += count(current_depth + 1, child, &orbits);
            }
            total
        }
        None => 0,
    }
}

fn create_children(orbits: Vec<Orbit>) -> HashMap<String, Vec<String>> {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    for orbit in orbits {
        if children.contains_key(&orbit.base) {
            let current_orbits = children.get_mut(&orbit.base);
            current_orbits.unwrap().push(orbit.planet);
        } else {
            let mut current_orbits = Vec::new();
            current_orbits.push(orbit.planet);
            children.insert(orbit.base, current_orbits);
        }
    }
    children
}

fn get_path_from_com(
    current_position: &String,
    target: &String,
    orbits: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    if current_position == target {
        return Some(Vec::new());
    }
    let mut path: Vec<String> = Vec::new();
    path.push(current_position.clone());
    match orbits.get(current_position) {
        Some(children) => {
            for child in children {
                match get_path_from_com(child, target, orbits) {
                    Some(x) => {
                        if x.len() == 0 {
                            path.push(child.clone());
                        } else {
                            for planet in x {
                                path.push(planet);
                            }
                        }
                        return Some(path);
                    }
                    None => {}
                }
            }
            return None;
        }
        None => return None,
    }
}

fn calculate_orbital_transfer(
    start: String,
    end: String,
    orbits: &HashMap<String, Vec<String>>,
) -> usize {
    let mut me = match get_path_from_com(&String::from("COM"), &start, &orbits) {
        Some(x) => x,
        None => panic!("YOU IS NOT FOUND!"),
    };
    me.reverse();
    let mut santa = match get_path_from_com(&String::from("COM"), &end, &orbits) {
        Some(x) => x,
        None => panic!("SANTA IS NOT FOUND!"),
    };
    santa.reverse();
    for position in 0..me.len() {
        let mut index = 0;
        let mut intercept = false;
        for planet in &santa {
            if planet == &me[position] {
                intercept = true;
                break;
            } else {
                index += 1;
            }
        }
        if intercept {
            println!("Intercept: {}", me[position]);
            return position + index - 2;
        }
    }
    0
}

fn display_orbits(
    current_position: &String,
    orbits: &HashMap<String, Vec<String>>,
) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    rows.push(Vec::new());
    match orbits.get(current_position) {
        Some(children) => {
            let mut index = 0;
            for element in children {
                rows[0].push(element.clone());
                for x in 1..rows.len() {
                    rows[x].push(String::from(" "));
                }
                let mut depth = 1;
                let mut index_increase = 1;
                for child_row in display_orbits(element, orbits) {
                    while rows.len() < 1 + depth {
                        let new_row = vec![String::from(" "); rows[0].len()];
                        rows.push(new_row);
                    }
                    let mut planet_index = 0;
                    if index_increase < child_row.len() {
                        index_increase = child_row.len();
                    }

                    for planet in &child_row {
                        if rows[0].len() < index + child_row.len() {
                            for x in 0..rows.len() {
                                rows[x].push(String::from(" "));
                            }
                        }
                        rows[depth][index + planet_index] = String::from(planet);
                        planet_index += 1;
                    }
                    depth += 1;
                }
                index += index_increase;
            }
        }
        None => {}
    };
    rows
}

pub fn start() {
    let filename = "data/input-06";
    let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
    let contents = raw_contents.trim().split("\n");
    let mut data: Vec<Orbit> = Vec::new();
    for element in contents {
        let split: Vec<&str> = element.split(")").collect();
        data.push(Orbit {
            base: split[0].to_string(),
            planet: split[1].to_string(),
        });
    }
    let orbits = create_children(data);
    //println!("{:?}",orbits);
    /*    for data in display_orbits(&String::from("COM"), &orbits){
        for element in data{
            print!("    {}",element);
        }
        print!("\n");
    }*/
    println!("Total Orbits: {}", count(0, &"COM".to_string(), &orbits));
    println!(
        "Orbital Transfer Length: {}",
        calculate_orbital_transfer(String::from("YOU"), String::from("SAN"), &orbits)
    );
}
