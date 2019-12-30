pub mod computer {
    pub struct Computer {
        pub memory: Vec<i64>,
        pub memory_size: usize,
        pub relative_base: isize,
        pub input_value: isize,
    }

    impl Computer {
        fn get_arguments(&mut self, index: &usize, mut opcode: isize) -> Vec<usize> {
            let mut args: Vec<usize> = vec![0; 5];
            args[0] = (opcode % 10) as usize;
            opcode = opcode / 10;
            args[1] = (opcode % 10) as usize;
            opcode = opcode / 10;
            let mut arg_count = 2;
            while opcode >= 1 {
                args[arg_count] = if opcode % 10 == 0 {
                    //Position mode
                    self.memory[index + arg_count - 1] as usize
                } else if opcode % 10 == 1 {
                    //Immediate mode
                    index + arg_count - 1
                } else if opcode % 10 == 2 {
                    //Relative mode
                    (self.memory[index + arg_count - 1] + self.relative_base as i64) as usize
                } else {
                    0
                };
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
            while arg_count < required_number {
                args[arg_count] = self.memory[index + arg_count - 1] as usize;
                arg_count += 1;
            }
            args
        }

        fn execute_program(&mut self) -> isize {
            let mut index: usize = 0;
            let mut output: Vec<isize> = Vec::new();
            loop {
                let args = self.get_arguments(&index, self.memory[index].clone() as isize);
                match args[0] {
                    // ADD
                    1 => {
                        self.memory[args[4]] = self.memory[args[2]] + self.memory[args[3]];
                        index += 4;
                    }
                    // MULTIPLY
                    2 => {
                        //println!("{} x {} ", self.memory[args[2]], self.memory[args[3]]);
                        self.memory[args[4]] = self.memory[args[2]] * self.memory[args[3]];
                        index += 4;
                    }
                    // INPUT
                    3 => {
                        //print!("    INPUT REQUIRED");
                        self.memory[args[2]] = self.input_value as i64;
                        index += 2
                    }
                    // OUTPUT
                    4 => {
                        //println!("    \nOUTPUT: {}\n", self.memory[args[2]]);
                        output.push(self.memory[args[2]] as isize);
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
                            self.relative_base += self.memory[args[2]] as isize;
                            index += 2;
                        }
                    }
                    _ => {
                        println!("UNEXPECTED OP CODE {}", args[0]);
                        panic!();
                    }
                }
            }
            //println!("\n\n\nOutput: {:?}", output);
            //self.memory[0]
            output[0]
        }

        pub fn start(&mut self) -> isize {
            let start = self.memory.len();
            for _x in start..self.memory_size {
                self.memory.push(0);
            }
            self.execute_program()
        }
    }

    pub(crate) fn load_data(raw_contents: String) -> Vec<i64> {
        let contents = raw_contents.trim().split(",");
        contents.map(|x| x.parse().unwrap()).collect()
    }
}

pub fn sum_of_primes() {
    let code = "  3,100,1007,100,2,7,1105,-1,87,1007,100,1,14,1105,-1,27,101,-2,100,100,101,1,101,101,1105,1,9,101,105,101,105,101,2,104,104,101,1,102,102,1,102,102,103,101,1,103,103,7,102,101,52,1106,-1,87,101,105,102,59,1005,-1,65,1,103,104,104,101,105,102,83,1,103,83,83,7,83,105,78,1106,-1,35,1101,0,1,-1,1105,1,69,4,104,99";
    computer::Computer {
        memory: computer::load_data(String::from(code)),
        memory_size: 100000,
        relative_base: 0,
        input_value: 100000,
    }
    .start();
}

pub fn prime_factor_small() {
    let code = "  3,1,109,583,108,0,1,9,1106,-1,14,4,1,99,107,0,1,19,1105,-1,27,104,-1,102,-1,1,1,21101,0,38,0,20101,0,1,1,1105,1,138,2101,1,1,41,101,596,41,45,1101,1,596,77,1101,0,1,53,101,1,77,77,101,1,53,53,7,45,77,67,1105,-1,128,108,1,1,74,1105,-1,128,1005,-1,54,1,53,77,93,7,45,93,88,1105,-1,101,1101,0,1,-1,1,53,93,93,1105,1,83,21101,0,116,0,20101,0,1,1,20101,0,53,2,1105,1,235,1205,2,54,4,53,2101,0,1,1,1105,1,101,108,1,1,133,1105,-1,137,4,1,99,22101,0,1,2,22101,0,1,1,21101,0,163,3,22101,0,1,4,22101,0,2,5,109,3,1105,1,198,109,-3,22102,-1,1,1,22201,1,4,3,22102,-1,1,1,1208,3,0,182,2105,-1,0,1208,3,1,189,2105,-1,0,22101,0,4,1,1105,1,146,1207,1,1,203,2105,-1,0,21101,0,222,3,22101,0,2,4,22101,0,1,5,109,3,1105,1,235,109,-3,22201,1,4,1,21101,0,2,2,1105,1,235,1105,0,280,101,383,236,243,1107,-1,583,247,1106,-1,276,101,383,236,256,102,1,275,-1,102,2,275,275,1007,275,0,266,1105,-1,280,101,1,236,236,1105,1,238,1,101,-1,236,236,101,383,236,286,207,1,-1,289,1106,-1,-1,22101,0,1,3,2102,1,2,363,2102,-1,2,369,22102,0,1,1,22102,0,2,2,101,1,236,320,101,-1,320,320,1107,-1,0,324,2105,-1,0,22102,2,2,2,101,383,320,336,207,3,-1,339,1105,-1,361,22101,1,2,2,22102,-1,3,3,101,383,320,354,22001,-1,3,3,22102,-1,3,3,1207,2,-1,366,1105,-1,315,22101,-1,2,2,101,383,320,377,22001,-1,1,1,1105,1,315";
    computer::Computer {
        memory: computer::load_data(String::from(code)),
        memory_size: 50340,
        relative_base: 0,
        input_value: 2147483647,
    }
    .start();
}

pub fn prime_factor_big() {
    let code = "3,1,109,583,108,0,1,9,1106,-1,14,4,1,99,107,0,1,19,1105,-1,27,104,-1,102,-1,1,1,21101,0,38,0,20101,0,1,1,1105,1,138,2101,1,1,41,101,596,41,45,1101,1,596,77,1101,0,1,53,101,1,77,77,101,1,53,53,7,45,77,67,1105,-1,128,108,1,1,74,1105,-1,128,1005,-1,54,1,53,77,93,7,45,93,88,1105,-1,101,1101,0,1,-1,1,53,93,93,1105,1,83,21101,0,116,0,20101,0,1,1,20101,0,53,2,1105,1,235,1205,2,54,4,53,2101,0,1,1,1105,1,101,108,1,1,133,1105,-1,137,4,1,99,22101,0,1,2,22101,0,1,1,21101,0,163,3,22101,0,1,4,22101,0,2,5,109,3,1105,1,198,109,-3,22102,-1,1,1,22201,1,4,3,22102,-1,1,1,1208,3,0,182,2105,-1,0,1208,3,1,189,2105,-1,0,22101,0,4,1,1105,1,146,1207,1,1,203,2105,-1,0,21101,0,222,3,22101,0,2,4,22101,0,1,5,109,3,1105,1,235,109,-3,22201,1,4,1,21101,0,2,2,1105,1,235,1105,0,280,101,383,236,243,1107,-1,583,247,1106,-1,276,101,383,236,256,102,1,275,-1,102,2,275,275,1007,275,0,266,1105,-1,280,101,1,236,236,1105,1,238,1,101,-1,236,236,101,383,236,286,207,1,-1,289,1106,-1,-1,22101,0,1,3,2102,1,2,363,2102,-1,2,369,22102,0,1,1,22102,0,2,2,101,1,236,320,101,-1,320,320,1107,-1,0,324,2105,-1,0,22102,2,2,2,101,383,320,336,207,3,-1,339,1105,-1,361,22101,1,2,2,22102,-1,3,3,101,383,320,354,22001,-1,3,3,22102,-1,3,3,1207,2,-1,366,1105,-1,315,22101,-1,2,2,101,383,320,377,22001,-1,1,1,1105,1,315";
    computer::Computer {
        memory: computer::load_data(String::from(code)),
        memory_size: 140569,
        relative_base: 0,
        input_value: 19201644899,
    }
    .start();
}
