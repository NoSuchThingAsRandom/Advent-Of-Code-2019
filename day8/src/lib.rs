use std::fs;

fn read_file(filename: &String) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

fn create_layers(width: u8, height: u8, data: String) -> Vec<(Vec<Vec<u32>>, u32)> {
    let mut values: Vec<char> = data.chars().collect();
    let mut layers = Vec::new();
    let mut exit = false;
    let mut index = 0;
    while index < values.len() {
        let mut current_layer = Vec::new();
        let mut zeros = 0;
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let current_val = values[index].to_digit(10).unwrap();
                if current_val == 0 {
                    zeros += 1;
                }
                row.push(current_val);
                index += 1;
            }
            current_layer.push(row);
        }
        layers.push((current_layer, zeros));
    }
    layers
}

fn checksum(layers: &Vec<(Vec<Vec<u32>>, u32)>) -> usize {
    let mut sorted = layers.clone();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ones = 0;
    let mut twos = 0;
    for row in &sorted[0].0 {
        for val in row {
            match val {
                1 => ones += 1,
                2 => twos += 1,
                _ => {}
            }
        }
    }
    ones * twos
}

fn display_layers(layers: &Vec<(Vec<Vec<u32>>, u32)>) {
    for l in 0..layers.len() {
        println!("Layer: {}", l);
        for y in &layers[l].0 {
            for x in y {
                print!("{}", x);
            }
            print!("\n");
        }
    }
}

fn get_colour(index: usize, x: usize, y: usize, layers: &Vec<(Vec<Vec<u32>>, u32)>) -> bool {
    match layers[index].0[y][x] {
        0 => false,
        1 => true,
        _ => get_colour(index + 1, x, y, &layers),
    }
}

fn calculate_pixels(
    width: usize,
    height: usize,
    layers: &Vec<(Vec<Vec<u32>>, u32)>,
) -> Vec<Vec<bool>> {
    let mut pixels = Vec::new();
    for y in 0..height {
        let mut current_row = Vec::new();
        for x in 0..width {
            current_row.push(get_colour(0, x, y, &layers));
        }
        pixels.push(current_row);
    }
    pixels
}

fn display(pixels: Vec<Vec<bool>>) {
    println!("Message: ");
    for y in &pixels {
        for x in y {
            if x == &true {
                print!("{}", 11);
            } else {
                print!("  ",);
            }
        }
        print!("\n");
    }
}

pub fn start(filename: String, width: usize, height: usize) {
    let data = read_file(&filename);
    let layers = create_layers(width as u8, height as u8, data);
    println!("Checksum {}", checksum(&layers));
    //display_layers(&layers);
    display(calculate_pixels(width, height, &layers));
}
