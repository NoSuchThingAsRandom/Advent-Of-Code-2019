use std::fs;

pub fn start(filename: String) {
    let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
    let contents = raw_contents.trim().split(",");
    let data: Vec<usize> = contents.map(|x| x.parse().unwrap()).collect();
    let result = execute_program(data.clone(), 12, 2);
    for noun in 0..99 {
        for verb in 0..99 {
            if execute_program(data.clone(), noun, verb) == 19690720 {
                println!(
                    "Noun {},  Verb {}, Output {}",
                    noun,
                    verb,
                    (100 * noun) + verb
                );
            }
        }
    }
    println!("{}", result);
    println!("{}", data[0])
}

fn execute_program(mut instructions: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut finished = false;
    let mut index = 0;
    instructions[1] = noun;
    instructions[2] = verb;
    while index < instructions.len() && !finished {
        if instructions.len() < instructions[index + 1] {
            println!("OUT OF MEMORY {}", instructions[index + 1]);
            panic!();
        } else if instructions.len() < instructions[index + 2] {
            println!("OUT OF MEMORY {}", instructions[index + 2]);
            panic!();
        } else if instructions.len() < instructions[index + 3] {
            println!("OUT OF MEMORY {}", instructions[index + 3]);
            panic!();
        } else {
            let first_arg = instructions[instructions[index + 1]] as usize;
            let second_arg = instructions[instructions[index + 2]] as usize;
            let result_pos = instructions[index + 3] as usize;
            if instructions[index] == 1 {
                instructions[result_pos] = first_arg + second_arg;
            } else if instructions[index] == 2 {
                instructions[result_pos] = first_arg * second_arg;
            } else if instructions[index] == 99 {
                finished = true;
            } else {
                println!("UNEXPECTED OP CODE {}", instructions[index]);
                panic!();
            }
        }
        index += 4;
    }
    instructions[0]
}
