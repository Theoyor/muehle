mod base;   
use base::base::State;
mod screen;
use screen::start;

fn main() {
    println!("Hello, world!");
    let fd = State::new();
    fd.printm();

    println!("{}",fd.spot_muehle((1,1,1)));
    let fd = fd.change(4, 1, 1).unwrap();
    let fd = fd.change(4, 2, 1).unwrap();
    let fd = fd.change(4, 7, 1).unwrap();
    fd.printm();
    println!("{}",fd.spot_muehle((4,1,1)));
    screen::start();
}
