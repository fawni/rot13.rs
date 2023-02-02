use rot13::Rot13;

fn main() {
    println!("{}", std::env::args().nth(1).expect("nothing to rot").rot13());
}
