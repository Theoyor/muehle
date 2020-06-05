mod base;   
use base::base::State;
mod screen;
use screen::start;

fn main() {
    println!("Hello, world!");
    let fd = State::new();
    fd.printm();

    println!("{}",fd.spot_muehle((1,1,1)));
    fd.printm();
    println!("{}",fd.spot_muehle((4,1,1)));
    screen::start();
}
