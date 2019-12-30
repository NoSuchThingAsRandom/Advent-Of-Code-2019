use std::fs;

struct Computer {
    initial_file: String,
    memory: Vec<isize>,
    phase_number: isize,
    input_value: isize,
}

impl Computer {
    fn load_data(&mut self) {
        let raw_contents = fs::read_to_string(&self.initial_file).expect("Failed to read file!");
        let contents = raw_contents.trim().split(",");
        self.memory = contents.map(|x| x.parse().unwrap()).collect();
    }

    fn get_arguments(&mut self, index: &usize, mut opcode: isize) -> Vec<isize> {
        let opcode_length = opcode.to_string().len();
        let mut args: Vec<isize> = Vec::new();
        args.push(opcode % 10);
        opcode = opcode / 10;
        args.push(opcode % 10);
        opcode = opcode / 10;
        for arg_count in 2..opcode_length {
            args.push(if opcode % 10 == 0 {
                //Position mode
                self.memory[self.memory[index + arg_count - 1] as usize]
            } else {
                self.memory[index + arg_count - 1]
            });
            opcode = opcode / 10;
        }
        let required_number = match args[0] {
            1 | 2 | 7 | 8 => 4,
            4 => 3,
            5 | 6 => 4,
            9 => 1,
            _ => 2,
        };
        let mut count = args.len() - 1;
        while args.len() < required_number {
            args.push(self.memory[self.memory[index + count] as usize]);
            count += 1;
        }
        //print!(" Args: {:?}", args);
        args
    }

    fn execute_program(&mut self) -> isize {
        let mut finished = false;
        let mut index: usize = 0;
        let mut got_phase = false;
        while (index as usize) < self.memory.len() && !finished {
            //print!("\nIndex {}, ", index);
            ////print!("Data [{}, {}, {}, {}]  ", self.memory[index], self.memory[index + 1], self.memory[index + 2], self.memory[index + 3]);
            let args = self.get_arguments(&index, self.memory[index].clone());
            match args[0] {
                // ADD
                1 => {
                    let result = self.memory[index + 3] as usize;
                    self.memory[result] = args[2] + args[3];
                    index += 4;
                    //print!(", value {} is stored in {}", (args[2] + args[3]), result);
                }
                // MULTIPLY
                2 => {
                    let result = self.memory[index + 3] as usize;
                    self.memory[result] = args[2] * args[3];
                    index += 4;
                    //print!(", value {} is stored in {}", (args[2] * args[3]), result);
                }
                // INPUT
                3 => {
                    let result = self.memory[index + 1] as usize;
                    if got_phase {
                        self.memory[result] = self.input_value;
                    } else {
                        self.memory[result] = self.phase_number;
                        got_phase = true;
                    }
                    index += 2
                }
                4 => {
                    //println!("    \nOUTPUT: {}\n", args[2]);
                    //index += 2;
                    return args[2];
                }
                // OUTPUT
                // JUMP IF TRUE
                5 => {
                    if args[2] != 0 {
                        index = args[3] as usize;
                    } else {
                        index += 3;
                    }
                }
                // JUMP IF FALSE
                6 => {
                    if args[2] == 0 {
                        index = args[3] as usize;
                    } else {
                        index += 3;
                    }
                }
                // LESS THAN
                7 => {
                    let result = self.memory[index + 3] as usize;
                    if args[2] < args[3] {
                        self.memory[result] = 1;
                    } else {
                        self.memory[result] = 0;
                    }
                    index += 4;
                }
                // EQUALS
                8 => {
                    let result = self.memory[index + 3] as usize;
                    if args[2] == args[3] {
                        self.memory[result] = 1;
                    } else {
                        self.memory[result] = 0;
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
        -1
    }

    pub fn start(&mut self) -> isize {
        self.load_data();
        self.execute_program()
    }
}
fn get_highest(filename: String, input_value: isize) -> isize {
    println!("INput {}", input_value);
    let mut max_value = 0;
    let mut phase_value = 0;
    for phase in 0..5 {
        let new_value: isize = Computer {
            initial_file: String::from(filename.clone()),
            memory: Vec::new(),
            phase_number: phase,
            input_value,
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

pub fn start() {
    let filename = "data/test-07-C";
    //let code="109,99,21101,0,13,0,203,1,203,2,1105,1,16,204,1,99,1205,1,26,22101,1,2,1,2105,1,0,1205,2,40,22101,-1,1,1,21101,0,1,2,1105,1,16,21101,0,57,3,22101,0,1,4,22101,-1,2,5,109,3,1105,1,16,109,-3,22101,0,4,2,22101,-1,1,1,1105,1,16";
    let mut input = 0;
    for _x in 0..5 {
        input = get_highest(String::from(filename), input);
    }
}
