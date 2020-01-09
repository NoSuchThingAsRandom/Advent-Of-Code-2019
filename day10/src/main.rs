mod lib;

fn main() {
    println!("A");
    lib::start(String::from("../data/test-10-A"));
    lib::start(String::from("../data/test-10-B"));
    lib::start(String::from("../data/test-10-C"));
    lib::start(String::from("../data/test-10-D"));
    lib::start(String::from("../data/test-10-E"));
}
