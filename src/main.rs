mod base;   
use base::base::State;

fn main() {
    println!("Hello, world!");
    let fd = State::new();
    println!("{}",fd.spot_muehle((1,1,1)));
}
