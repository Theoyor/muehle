mod base;   
use base::base::State;
mod screen;
use screen::start;

fn main() {
    println!("Hello, world!");
    let mut fd = State::new();

    fd = fd.place((1,1,0)).unwrap();
    fd = fd.place((1,4,0)).unwrap();
    fd = fd.place((4,1,0)).unwrap();
    fd = fd.place((2,2,0)).unwrap();
    fd = fd.place((7,1,0)).unwrap();
    fd = fd.remove((1,4,-1)).unwrap();
    println!("{:?}", fd);
    screen::start();
}
