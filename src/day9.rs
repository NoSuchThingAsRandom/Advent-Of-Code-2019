use std::fs;
pub struct Computer {
    memory: Vec<isize>,
    memory_size: usize,
    relative_base: isize,
    input_value: isize,
}

impl Computer {
    fn get_arguments(&mut self, index: &usize, mut opcode: isize) -> Vec<usize> {
        let mut args: Vec<usize> = Vec::new();
        args.push((opcode % 10) as usize);
        opcode = opcode / 10;
        args.push((opcode % 10) as usize);
        opcode = opcode / 10;
        let mut arg_count = 2;
        while opcode >= 1 {
            args.push(if opcode % 10 == 0 {
                //Position mode
                self.memory[index + arg_count - 1] as usize
            } else if opcode % 10 == 1 {
                //Immediate mode
                index + arg_count - 1
            } else if opcode % 10 == 2 {
                //Relative mode
                (self.memory[index + arg_count - 1] + self.relative_base as isize) as usize
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
            args.push(self.memory[index + count] as usize);
            count += 1;
        }
        args
    }

    fn execute_program(&mut self) -> isize {
        let mut finished = false;
        let mut index: usize = 0;
        let mut output: Vec<isize> = Vec::new();
        loop {
            let args = self.get_arguments(&index, self.memory[index].clone());
            match args[0] {
                // ADD
                1 => {
                    self.memory[args[4]] = self.memory[args[2]] + self.memory[args[3]];
                    index += 4;
                }
                // MULTIPLY
                2 => {
                    self.memory[args[4]] = self.memory[args[2]] * self.memory[args[3]];
                    index += 4;
                }
                // INPUT
                3 => {
                    print!("    INPUT REQUIRED");
                    self.memory[args[2]] = self.input_value;
                    index += 2
                }
                // OUTPUT
                4 => {
                    println!("    \nOUTPUT: {}\n", self.memory[args[2]]);
                    output.push(self.memory[args[2]]);
                    index += 2;
                }
                // JUMP IF TRUE
                5 => {
                    if self.memory[args[2]] != 0 {
                        index = self.memory[args[3]] as usize;
                    } else {
                        index += 3;
                    }
                }
                // JUMP IF FALSE
                6 => {
                    if self.memory[args[2]] == 0 {
                        index = self.memory[args[3]] as usize;
                    } else {
                        index += 3;
                    }
                }
                // LESS THAN
                7 => {
                    if self.memory[args[2]] < self.memory[args[3]] {
                        self.memory[args[4]] = 1;
                    } else {
                        self.memory[args[4]] = 0;
                    }
                    index += 4;
                }
                // EQUALS
                8 => {
                    if self.memory[args[2]] == self.memory[args[3]] {
                        self.memory[args[4]] = 1;
                    } else {
                        self.memory[args[4]] = 0;
                    }
                    index += 4;
                }
                9 => {
                    if args[1] == 9 {
                        break;
                    } else {
                        self.relative_base += self.memory[args[2]];
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
        //self.memory[0]
        output[0]
    }

    pub fn start(&mut self) -> isize {
        let start = self.memory.len();
        for x in start..self.memory_size {
            self.memory.push(0);
        }
        self.execute_program()
    }
}

fn load_data(raw_contents: String) -> Vec<isize> {
    let contents = raw_contents.trim().split(",");
    contents.map(|x| x.parse().unwrap()).collect()
}
pub fn sum_of_primes() {
    let code="  3,100,1007,100,2,7,1105,-1,87,1007,100,1,14,1105,-1,27,101,-2,100,100,101,1,101,101,1105,1,9,101,105,101,105,101,2,104,104,101,1,102,102,1,102,102,103,101,1,103,103,7,102,101,52,1106,-1,87,101,105,102,59,1005,-1,65,1,103,104,104,101,105,102,83,1,103,83,83,7,83,105,78,1106,-1,35,1101,0,1,-1,1105,1,69,4,104,99";
    let mut memory = load_data(String::from(code));
    Computer {
        memory,
        memory_size: 5000,
        relative_base: 0,
        input_value: 1,
    }
    .start();
}

pub fn start() {
    let mut memory =
        load_data(fs::read_to_string(String::from("data/input-09")).expect("Failed to read file!"));
    Computer {
        memory,
        memory_size: 5000,
        relative_base: 0,
        input_value: 1,
    }
    .start();
}

//let mut data=vec![109, -1, 104, 1, 99];
//let mut data=vec![109, -1, 204, 1, 99];
//let mut data=vec![109, 1, 9, 2, 204, -6, 99];
//let mut data=vec![109, 1, 109, 9, 204, -6, 99];

//let mut data=vec![109, 1, 209, -1, 204, -106, 99] ;
//let mut data=vec![109, 1, 3, 3, 204, 2, 99];

//let mut data=vec!    [109, 1, 203, 2, 204, 2, 99];
