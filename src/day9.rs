use std::fs;

fn get_arguments(
    instructions: &Vec<isize>,
    relative_base: isize,
    index: &usize,
    mut opcode: isize,
) -> Vec<usize> {
    let mut args: Vec<usize> = Vec::new();
    args.push((opcode % 10) as usize);
    opcode = opcode / 10;
    args.push((opcode % 10) as usize);
    opcode = opcode / 10;
    let mut arg_count = 2;
    while opcode >= 1 {
        args.push(if opcode % 10 == 0 {
            //Position mode
            instructions[index + arg_count - 1] as usize
        } else if opcode % 10 == 1 {
            //Immediate mode
            index + arg_count - 1
        } else if opcode % 10 == 2 {
            //Relative mode
            (instructions[index + arg_count - 1] + relative_base) as usize
        } else {
            0
        });
        opcode = opcode / 10;
        arg_count += 1;
    }
    //For missing arguments in presumed position mode e.g. opcode "1"
    let required_number = match args[0] {
        1 | 2 | 7 | 8 => 5,
        3 | 4 => 3,
        5 | 6 => 4,
        9 => 3,
        _ => 2,
    };
    let mut count = args.len() - 1;
    while args.len() < required_number {
        args.push(instructions[index + count] as usize);
        count += 1;
    }
    args
}

fn execute_program_immediate_mode(mut instructions: Vec<isize>) -> isize {
    let mut relative_base: isize = 0;
    let mut finished = false;
    let mut index: usize = 0;
    let mut output: Vec<isize> = Vec::new();
    loop {
        let args = get_arguments(
            &instructions,
            relative_base,
            &index,
            instructions[index].clone(),
        );
        match args[0] {
            // ADD
            1 => {
                instructions[args[4]] = instructions[args[2]] + instructions[args[3]];
                index += 4;
            }
            // MULTIPLY
            2 => {
                instructions[args[4]] = instructions[args[2]] * instructions[args[3]];
                index += 4;
            }
            // INPUT
            3 => {
                print!("    INPUT REQUIRED");
                instructions[args[2]] = 2;
                index += 2
            }
            // OUTPUT
            4 => {
                println!("    \nOUTPUT: {}\n", instructions[args[2]]);
                output.push(instructions[args[2]]);
                index += 2;
            }
            // JUMP IF TRUE
            5 => {
                if instructions[args[2]] != 0 {
                    index = instructions[args[3]] as usize;
                } else {
                    index += 3;
                }
            }
            // JUMP IF FALSE
            6 => {
                if instructions[args[2]] == 0 {
                    index = instructions[args[3]] as usize;
                } else {
                    index += 3;
                }
            }
            // LESS THAN
            7 => {
                if instructions[args[2]] < instructions[args[3]] {
                    instructions[args[4]] = 1;
                } else {
                    instructions[args[4]] = 0;
                }
                index += 4;
            }
            // EQUALS
            8 => {
                if instructions[args[2]] == instructions[args[3]] {
                    instructions[args[4]] = 1;
                } else {
                    instructions[args[4]] = 0;
                }
                index += 4;
            }
            9 => {
                if args[1] == 9 {
                    break;
                } else {
                    relative_base += instructions[args[2]];
                    index += 2;
                }
            }
            _ => {
                println!("UNEXPECTED OP CODE {}", args[0]);
                panic!();
            }
        }
    }
    println!("\n\n\nOutput: {:?}", output);
    0
}

pub fn start() {
    let filename = "data/input-09";
    let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
    let contents = raw_contents.trim().split(",");
    let mut data: Vec<isize> = contents.map(|x| x.parse().unwrap()).collect();
    //println!("{:?}",data);
    for x in 0..1000 {
        data.push(0);
    }
    let result = execute_program_immediate_mode(data.clone());
}
