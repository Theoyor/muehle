mod base;   
use base::base::State;
mod screen;
use screen::start;

fn main() {
    let mut fd = State::new();

    fd = fd.place((4,2,0)).unwrap();
    fd = fd.place((7,4,0)).unwrap();
    
    fd = fd.place((1,1,0)).unwrap();
    fd = fd.place((7,1,0)).unwrap();
    fd = fd.place((4,3,0)).unwrap();
    fd = fd.place((7,7,0)).unwrap();
    fd = fd.remove((1,1,1)).unwrap();
    let i = fd.spot_pot_muehle((4,2,1));
    println!("{}", i);
    println!("{:?}", fd);
    //screen::start();
}
