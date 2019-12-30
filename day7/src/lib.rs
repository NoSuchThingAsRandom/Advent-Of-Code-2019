use std::collections::VecDeque;
use std::fs;

use intcode::computer;

fn get_highest(filename: &String, input_value: isize) -> isize {
    let mut max_value = 0;
    let mut phase_value = 0;
    for phase in 0..5 {
        let mut inputs = VecDeque::new();
        inputs.push_back(phase);
        inputs.push_back(input_value);
        let new_value: isize = intcode::computer::Computer {
            memory: intcode::computer::load_data(
                fs::read_to_string(String::from(filename)).expect("Failed to read file"),
            ),
            memory_size: 0,
            relative_base: 0,
            inputs,
        }
        .start();
        if new_value > max_value {
            max_value = new_value;
            phase_value = phase;
        }
    }
    println!("Highest {} with phase {}", max_value, phase_value);
    max_value
}

pub fn start(filename: String) {
    //let code="109,99,21101,0,13,0,203,1,203,2,1105,1,16,204,1,99,1205,1,26,22101,1,2,1,2105,1,0,1205,2,40,22101,-1,1,1,21101,0,1,2,1105,1,16,21101,0,57,3,22101,0,1,4,22101,-1,2,5,109,3,1105,1,16,109,-3,22101,0,4,2,22101,-1,1,1,1105,1,16";
    let mut input = 1;
    for _x in 0..5 {
        input = get_highest(&filename, input);
        //println!("{}", input);
    }
}
