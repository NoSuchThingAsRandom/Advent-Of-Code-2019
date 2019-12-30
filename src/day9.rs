pub mod intcode;
pub fn start() {
    intcode::sum_of_primes();
    //intcode::sum_of_primes();
    /*let mut memory = computer::load_data(
        fs::read_to_string(String::from("data/input-09")).expect("Failed to read file!"),
    );
    computer {
        memory,
        memory_size: 5000,
        relative_base: 0,
        input_value: 1,
    }
    .start();*/
}

//let mut data=vec![109, -1, 104, 1, 99];
//let mut data=vec![109, -1, 204, 1, 99];
//let mut data=vec![109, 1, 9, 2, 204, -6, 99];
//let mut data=vec![109, 1, 109, 9, 204, -6, 99];

//let mut data=vec![109, 1, 209, -1, 204, -106, 99] ;
//let mut data=vec![109, 1, 3, 3, 204, 2, 99];

//let mut data=vec!    [109, 1, 203, 2, 204, 2, 99];
