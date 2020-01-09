use std::fs;

mod grid_output {
    use std::fs;

    #[derive(Debug)]
    pub struct Wire {
        direction: String,
        length: usize,
        name: GridType,
    }

    fn create_wire(to_parse: String, name: u8) -> Wire {
        let separated = to_parse.split_at(1);
        if name == 0 {
            Wire {
                direction: separated.0.parse().unwrap(),
                length: separated.1.parse().unwrap(),
                name: GridType::WireA,
            }
        } else {
            Wire {
                direction: separated.0.parse().unwrap(),
                length: separated.1.parse().unwrap(),
                name: GridType::WireB,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum GridType {
        EMPTY,
        START,
        WireA,
        WireB,
    }

    pub fn create_blank_grid() -> Vec<Vec<GridType>> {
        let mut grid: Vec<Vec<GridType>> = Vec::new();
        let mut row: Vec<GridType> = Vec::new();
        row.push(GridType::START);
        grid.push(row);
        grid
    }

    pub fn generate_grid(
        grid: &mut Vec<Vec<GridType>>,
        start: &mut (usize, usize),
        wire: &Vec<Wire>,
    ) {
        let mut position = start.clone();
        for movement in wire {
            extend_grid(grid, &mut position, movement, start);
            match movement.direction.as_ref() {
                "L" => {
                    for index in 0..movement.length + 1 {
                        if grid[position.1][index + position.0] == GridType::EMPTY {
                            grid[position.1][index + position.0] = movement.name;
                        }
                    }
                }
                "R" => {
                    for index in 0..movement.length + 1 {
                        if grid[position.1][position.0 - index] == GridType::EMPTY {
                            grid[position.1][position.0 - index] = movement.name;
                        }
                    }
                }
                "D" => {
                    for index in 0..movement.length + 1 {
                        if grid[position.1 + movement.length - index][position.0] == GridType::EMPTY
                        {
                            grid[position.1 + movement.length - index][position.0] = movement.name;
                        }
                    }
                }
                "U" => {
                    for index in 0..movement.length + 1 {
                        if grid[position.1 - index][position.0] == GridType::EMPTY {
                            grid[position.1 - index][position.0] = movement.name;
                        }
                    }
                }
                _ => panic!("UNEXPECTED DIRECTION"),
            }
            grid[position.1][position.0] = GridType::EMPTY;
        }
    }

    fn extend_grid(
        grid: &mut Vec<Vec<GridType>>,
        position: &mut (usize, usize),
        movement: &Wire,
        start: &mut (usize, usize),
    ) {
        match movement.direction.as_ref() {
            "L" => {
                if (position.0 as isize - movement.length as isize) < 0 {
                    for index in 0..grid.len() {
                        for _count in 0..movement.length - position.0 {
                            grid[index].insert(0, GridType::EMPTY);
                        }
                    }
                    position.0 = 0;
                    start.0 += movement.length - position.0;
                } else {
                    position.0 -= movement.length;
                }
            }
            "R" => {
                position.0 += movement.length;
                while position.0 >= grid[0].len() {
                    for index in 0..grid.len() {
                        grid[index].push(GridType::EMPTY);
                    }
                }
            }
            "D" => {
                if (position.1 as isize - movement.length as isize) < 0 {
                    for _count in 0..movement.length - position.1 {
                        grid.insert(0, vec![GridType::EMPTY; grid[0].len().clone()]);
                    }

                    start.1 += movement.length - position.1;
                    position.1 = 0;
                } else {
                    position.1 -= movement.length;
                }
            }
            "U" => {
                position.1 += movement.length;
                while position.1 >= grid.len() {
                    grid.push(vec![GridType::EMPTY; grid[0].len().clone()]);
                }
            }
            _ => panic!("UNEXPECTED DIRECTION"),
        }
    }

    pub fn get_movements(filename: &String) -> Vec<Vec<Wire>> {
        let mut wires: Vec<Vec<Wire>> = Vec::new();
        let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
        let contents = raw_contents.trim().split("\n");
        let mut index: u8 = 0;
        for element in contents {
            wires.push(
                element
                    .split(",")
                    .map(|x| create_wire(x.to_string(), index))
                    .collect(),
            );
            index += 1
        }
        wires
    }

    pub fn display_grid(grid: &Vec<Vec<GridType>>) {
        for row in grid.iter().rev() {
            let mut line = String::new();
            for element in row.iter() {
                match element {
                    GridType::EMPTY => {
                        line.push('-');
                    }
                    GridType::START => {
                        line.push('0');
                    }
                    GridType::WireA => {
                        line.push('A');
                    }
                    GridType::WireB => {
                        line.push('B');
                    }
                }
            }
            println!("{}", line);
        }
    }
}

#[derive(Debug, Clone)]
struct Wire {
    direction: String,
    length: isize,
}

fn create_wire(to_parse: String, name: u8) -> Wire {
    let separated = to_parse.split_at(1);
    if name == 0 {
        Wire {
            direction: separated.0.parse().unwrap(),
            length: separated.1.parse().unwrap(),
        }
    } else {
        Wire {
            direction: separated.0.parse().unwrap(),
            length: separated.1.parse().unwrap(),
        }
    }
}

struct Line {
    start: (isize, isize),
    end: (isize, isize),
    vertical: bool,
    distance_to: isize,
    origin: (isize, isize),
}

fn get_distance(point: &(isize, isize)) -> usize {
    (point.0.abs() + point.1.abs()) as usize
}

fn get_intersect_point(a: &Line, b: &Line) -> (isize, isize) {
    if a.vertical && b.vertical {
        a.start
    } else if a.vertical {
        (a.start.0, b.start.1)
    } else if b.vertical {
        (b.start.0, a.start.1)
    } else {
        a.start
    }
}

fn test_intersect(a: &Line, b: &Line) -> bool {
    if a.vertical && b.vertical {
        a.start.0 == b.start.0
    } else if a.vertical {
        (a.start.1 <= b.start.1 && b.start.1 <= a.end.1)
            && (b.start.0 <= a.start.0 && a.start.0 <= b.end.0)
    } else if b.vertical {
        (b.start.1 <= a.start.1 && a.start.1 <= b.end.1)
            && (a.start.0 <= b.start.0 && b.start.0 <= a.end.0)
    } else {
        a.start.1 == b.start.1
    }
}

fn generate_lines(wires: &Vec<Wire>) -> Vec<Line> {
    let mut position: (isize, isize) = (0, 0);
    let mut lines: Vec<Line> = Vec::new();
    let mut distance = 0;
    for wire in wires {
        match wire.direction.as_ref() {
            "L" => {
                lines.push(Line {
                    start: (position.0 - wire.length, position.1),
                    end: (position.0, position.1),
                    vertical: false,
                    distance_to: distance,
                    origin: (position.0, position.1),
                });
                position.0 -= wire.length;
            }
            "R" => {
                lines.push(Line {
                    start: (position.0, position.1),
                    end: (position.0 + wire.length, position.1),
                    vertical: false,
                    distance_to: distance,
                    origin: (position.0, position.1),
                });
                position.0 += wire.length;
            }
            "D" => {
                lines.push(Line {
                    start: (position.0, position.1 - wire.length),
                    end: (position.0, position.1),
                    vertical: true,
                    distance_to: distance,
                    origin: (position.0, position.1),
                });
                position.1 -= wire.length;
            }
            "U" => {
                lines.push(Line {
                    start: (position.0, position.1),
                    end: (position.0, position.1 + wire.length),
                    vertical: true,
                    distance_to: distance,
                    origin: (position.0, position.1),
                });
                position.1 += wire.length;
            }
            _ => panic!("UNKNOWN INSTRUCTION"),
        }
        distance += wire.length;
    }
    lines
}

fn get_movements(filename: &String) -> Vec<Vec<Wire>> {
    let mut wires: Vec<Vec<Wire>> = Vec::new();
    let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
    let contents = raw_contents.trim().split("\n");
    let mut index: u8 = 0;
    for element in contents {
        wires.push(
            element
                .split(",")
                .map(|x| create_wire(x.to_string(), index))
                .collect(),
        );
        index += 1
    }
    wires
}

fn find_closest_manhatten(filename: &String) {
    let wires = get_movements(&filename);
    let line_a = generate_lines(wires.get(0).unwrap());
    let line_b = generate_lines(wires.get(1).unwrap());
    let mut distance = 10000000;
    for line in &line_a {
        for test_line in &line_b {
            if test_intersect(&line, &test_line) {
                let test_distance = get_distance(&get_intersect_point(&line, &test_line));
                if test_distance < distance && test_distance != 0 {
                    distance = test_distance;
                }
            }
        }
    }
    println!("The manhatten distance is: {}", distance)
}

fn calculate_wire_distance(line: &Line, point: &(isize, isize)) -> isize {
    if line.vertical {
        if point.1 - line.origin.1 > 0 {
            line.distance_to + point.1 - line.origin.1
        } else {
            line.distance_to + line.origin.1 - point.1
        }
    } else {
        if point.0 - line.origin.0 > 0 {
            line.distance_to + point.0 - line.origin.0
        } else {
            line.distance_to + line.origin.0 - point.0
        }
    }
}

fn find_raw_closest(filename: &String) {
    let wires = get_movements(&filename);
    let line_a = generate_lines(wires.get(0).unwrap());
    let line_b = generate_lines(wires.get(1).unwrap());
    let mut distance = 10000000;
    for line in &line_a {
        for test_line in &line_b {
            if test_intersect(&line, &test_line) {
                let point = get_intersect_point(&line, &test_line);
                if point != (0, 0) {
                    let test_distance = calculate_wire_distance(&line, &point)
                        + calculate_wire_distance(&test_line, &point);
                    if test_distance < distance && test_distance != 0 {
                        distance = test_distance;
                    } else {
                    }
                }
            }
        }
    }
    println!("The total wire distance is: {}", distance)
}

fn output_grid(filename: &String) {
    let wire = grid_output::get_movements(&filename);
    let mut start: (usize, usize) = (0, 0);
    let mut grid = grid_output::create_blank_grid();
    grid_output::generate_grid(&mut grid, &mut start, &wire[0]);
    grid_output::generate_grid(&mut grid, &mut start, &wire[1]);
    grid_output::display_grid(&grid);
}

pub fn start(filename: String) {
    //output_grid(&filename);
    find_closest_manhatten(&filename);
    find_raw_closest(&filename);
}
