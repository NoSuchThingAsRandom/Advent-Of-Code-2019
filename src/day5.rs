use std::fs;


fn get_arguments(instructions: &Vec<isize>, index: &usize, mut opcode: isize) -> Vec<isize> {
    let opcode_length = opcode.to_string().len();
    let mut args: Vec<isize> = Vec::new();
    args.push(opcode % 10);
    opcode = opcode / 10;
    args.push(opcode % 10);
    opcode = opcode / 10;
    for arg_count in 2..opcode_length {
        args.push(
            if opcode % 10 == 0 {//Position mode
                instructions[instructions[index + arg_count - 1] as usize]
            } else {
                instructions[index + arg_count - 1]
            }
        );
        opcode = opcode / 10;
    }
    let required_number =
        match args[0] {
            1 | 2 | 7 | 8 => 4,
            4 =>3,
            5 | 6 => 4,
            9 => 1,
            _ => 2
        };
    let mut count = args.len() - 1;
    while args.len() < required_number {
        args.push(instructions[instructions[index + count] as usize]);
        count += 1;
    }
    //print!(" Args: {:?}", args);
    args
}


fn execute_program_immediate_mode(mut instructions: Vec<isize>) -> isize {
    let mut finished = false;
    let mut index: usize = 0;
    while (index as usize) < instructions.len() && !finished {
        //print!("\nIndex {}, ", index);
        ////print!("Data [{}, {}, {}, {}]  ", instructions[index], instructions[index + 1], instructions[index + 2], instructions[index + 3]);
        let args = get_arguments(&instructions, &index, instructions[index].clone());
        match args[0] {
            // ADD
            1 => {
                let result = instructions[index + 3] as usize;
                instructions[result] = args[2] + args[3];
                index += 4;
                //print!(", value {} is stored in {}", (args[2] + args[3]), result);
            }
            // MULTIPLY
            2 => {
                let result = instructions[index + 3] as usize;
                instructions[result] = args[2] * args[3];
                index += 4;
                //print!(", value {} is stored in {}", (args[2] * args[3]), result);
            }
            // INPUT
            3 => {
                //print!("    INPUT REQUIRED");
                let result = instructions[index + 1] as usize;
                instructions[result] = 5;
                index += 2
            }
            // OUTPUT
            4 => {
                println!("    \nOUTPUT: {}\n", args[2]);
                index += 2;
            }
            // JUMP IF TRUE
            5 => {
                if args[2] != 0 {
                    index = args[3] as usize;
                }else{
                    index+=3;
                }
            }
            // JUMP IF FALSE
            6 => {
                if args[2] == 0 {
                    index = args[3] as usize;
                }else{
                    index+=3;
                }
            }
            // LESS THAN
            7 => {
                let result = instructions[index + 3] as usize;
                if args[2] < args[3] {
                    instructions[result] = 1;
                } else {
                    instructions[result] = 0;
                }
                index += 4;
            }
            // EQUALS
            8 => {
                let result = instructions[index + 3] as usize;
                if args[2] == args[3] {
                    instructions[result] = 1;
                } else {
                    instructions[result] = 0;
                }
                index += 4;
            }
            9 => {
                finished = true;
            }
            _ => {
                //println!("UNEXPECTED OP CODE {}", args[0]);
                panic!();
            }
        }
    }
    instructions[0]
}

pub fn start() {
    let filename = "data/input-05";
    //let filename = "data/test-05-B";
    let raw_contents = fs::read_to_string(filename).expect("Failed to read file!");
    let contents = raw_contents.trim().split(",");
    let mut data: Vec<isize> = contents.map(|x| x.parse().unwrap()).collect();
    //data[1] = 12;
    //data[2] = 2;
    ////println!("{:?}", data);
    let _result = execute_program_immediate_mode(data.clone());
    ////println!("{}", result)
}