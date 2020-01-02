use permutohedron::Heap;
use std::collections::VecDeque;
use std::fs;
fn calculate_phase_result(filename: &String, phase: Vec<isize>, input: isize) -> isize {
    let mut input_value = input;
    for phase_value in phase {
        let mut inputs = VecDeque::new();
        inputs.push_back(phase_value.clone());
        inputs.push_back(input_value.clone());
        input_value = intcode::computer::Computer {
            memory: intcode::computer::load_data(
                fs::read_to_string(String::from(filename)).expect("Failed to read file"),
            ),
            memory_size: 0,
            relative_base: 0,
            inputs, //exit_on_output:false
        }
        .start();
    }
    input_value
}

fn part_a(filename: &String) {
    let mut data = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let mut max_value = 0;
    let mut phase = vec![0; 5];
    for data in heap {
        let result = calculate_phase_result(&filename, data.clone(), 0);
        if result > max_value {
            max_value = result;
            phase = data;
        }
    }
    println!("Phase {:?} produced {}", phase, max_value);
}

fn part_b(filename: &String) {
    let mut data = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);
    let mut max_value = 0;
    let mut phase = vec![0; 5];
    for data in heap {
        let result = part_b_calculate(&filename, data.clone(), 0);
        if result > max_value {
            max_value = result;
            phase = data;
        }
    }
    println!("Phase {:?} produced {}", phase, max_value);
}

fn part_b_calculate(filename: &String, phase: Vec<isize>, input: isize) -> isize {
    println!("Starting");
    let mut input_value = input;
    let mut amps: Vec<intcode::computer::Computer> = Vec::new();
    for x in 0..phase.len() {
        let mut inputs = VecDeque::new();
        inputs.push_back(phase[x]);
        inputs.push_back(input_value);
        amps.push(intcode::computer::Computer {
            memory: intcode::computer::load_data(
                fs::read_to_string(String::from(filename)).expect("Failed to read file"),
            ),
            memory_size: 0,
            relative_base: 0,
            inputs,
            //exit_on_output:true
        });
        input_value = amps[x].start();
    }
    println!("Starting");
    let mut count = 0;
    loop {
        count += 1;
        for amp in &mut amps {
            let mut inputs = VecDeque::new();
            inputs.push_back(input_value);
            amp.update_inputs(inputs);
            input_value = amp.start();
        }
        println!("{}", input_value);
        if count == 20 {
            break;
        }
    }
    input_value
}

pub fn start(filename: String) {
    part_a(&filename);
}
