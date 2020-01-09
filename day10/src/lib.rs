use std::fs;

fn parse_asteroid(filename: &String) -> Vec<(i32, i32)> {
    let lines = fs::read_to_string(filename).expect("Failed to read file!");

    let mut asteroids: Vec<(i32, i32)> = Vec::new();
    let mut row_index = 0;
    for row in lines.split("\n") {
        //Has to start at -1 because split creates an empty element
        let mut column_index = -1;
        for position in row.split("") {
            if position == "#" {
                asteroids.push((column_index as i32, row_index));
            }
            column_index += 1;
        }
        row_index += 1;
    }
    asteroids
}

fn display_amount(amount: Vec<u8>, filename: &String) {
    let lines = fs::read_to_string(filename).expect("Failed to read file!");

    let mut row_index = 0;
    let mut ast_index = 0;
    for row in lines.split("\n") {
        //Has to start at -1 because split creates an empty element
        let mut column_index = -1;
        for position in row.split("") {
            if column_index == -1 {
                column_index += 1;
                continue;
            }
            if position == "#" {
                print!("{}", amount[ast_index]);
                ast_index += 1;
            //asteroids.push((column_index as i32, row_index));
            } else {
                print!(".");
            }
            column_index += 1;
        }
        println!();
        row_index += 1;
    }
}

fn calculate_line(a: &(i32, i32), b: &(i32, i32), c: &(i32, i32)) -> bool {
    if a == b || a == c || b == c {
        false
    } else {
        (b.1 - a.1) * (c.0 - b.0) == (c.1 - b.1) * (b.0 - a.0)
    }
    /*

    } else if b.0 - a.0 == 0 && c.0 - a.0 == 0 {
        b.1 - a.1 == c.1 - a.1
    } else if b.0 - a.0 == 0 {
        (b.1 - a.1) == (c.1 - a.1) / (c.0 - a.0)
    } else if c.0 - a.0 == 0 {
        (b.1 - a.1) / (b.0 - a.0) == (c.1 - a.1)
    } else {
        (b.1 - a.1) / (b.0 - a.0) == (c.1 - a.1) / (c.0 - a.0)
    }*/
}

fn calculate_distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    ((((a.1 - b.1) * (a.1 - b.1)) + ((a.0 - b.0) * (a.0 - b.0))) as f32).sqrt() as i32
}

fn calculate_visible_asteroids(location: &(i32, i32), asteroids: &Vec<(i32, i32)>) -> u8 {
    let mut visible_asteroid: Vec<bool> = vec![true; asteroids.len()];

    for index_a in 0..asteroids.len() {
        let distance = calculate_distance(&location, &asteroids[index_a]);
        for index_b in 0..asteroids.len() {
            let test = calculate_line(location, &asteroids[index_a], &asteroids[index_b]);
            if test {
                if distance < calculate_distance(&location, &asteroids[index_b]) {
                    visible_asteroid[index_a] = false;
                } else {
                    visible_asteroid[index_b] = false;
                }
            }
        }
    }
    let mut total = 0;
    for ast in visible_asteroid {
        if ast {
            total += 1;
        }
    }
    //As it includes itself
    total - 1
}

pub fn start(filename: String) {
    /*    println!("{}", calculate_line(&(0, 0), &(1, 1), &(2, 2)));
    println!("{}", calculate_line(&(0, 0), &(0, 1), &(0, 2)));
    println!("{}", calculate_line(&(0, 0), &(1, 0), &(2, 0)));*/

    let asteroids = parse_asteroid(&filename);
    let mut res = Vec::new();
    let mut highest = 0;
    let mut position = (0, 0);
    for ast in &asteroids {
        let temp = calculate_visible_asteroids(ast, &asteroids);
        if temp > highest {
            highest = temp;
            position = *ast;
        }
        res.push(temp);
        //println!("{},       {:?}", temp, ast);
    }
    println!(
        "      The highest is {} at position {:?}",
        highest, position
    );
    //println!("\n\n\n");
    //display_amount(res, &filename);
}
