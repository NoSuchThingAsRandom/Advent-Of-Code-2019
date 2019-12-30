use std::collections::VecDeque;
use std::fs;

use intcode::computer;

pub fn start(filename: String) {
    let mut inputs = VecDeque::new();
    inputs.push_back(1);
    println!(
        "Part 1: {}",
        intcode::computer::Computer {
            memory: computer::load_data(
                fs::read_to_string(&filename).expect("Failed to read file!")
            ),
            memory_size: 5000,
            relative_base: 0,
            inputs,
        }
        .start()
    );

    let mut inputs = VecDeque::new();
    inputs.push_back(2);
    println!(
        "Part 2: {}",
        intcode::computer::Computer {
            memory: computer::load_data(
                fs::read_to_string(&filename).expect("Failed to read file!")
            ),
            memory_size: 5000,
            relative_base: 0,
            inputs,
        }
        .start()
    );
}

//let mut data=vec![109, -1, 104, 1, 99];
//let mut data=vec![109, -1, 204, 1, 99];
//let mut data=vec![109, 1, 9, 2, 204, -6, 99];
//let mut data=vec![109, 1, 109, 9, 204, -6, 99];

//let mut data=vec![109, 1, 209, -1, 204, -106, 99] ;
//let mut data=vec![109, 1, 3, 3, 204, 2, 99];

//let mut data=vec!    [109, 1, 203, 2, 204, 2, 99];
