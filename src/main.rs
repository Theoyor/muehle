mod base;   
use base::base::State;
mod screen;
use screen::start;

fn main() {
    let mut fd = State::new();

    fd = place_tst(fd);
    fd = mov_test(fd);
    let i = fd.spot_pot_muehle((4,2,1));
    println!("{}", i);
    println!("{:?}", fd);
    fd.spielstandbewertung();
    //screen::start();
}

fn mov_test(mut fd:State)->State{
    fd = fd.mov((4,2,1), (4,1,0)).unwrap();
    fd = fd.mov((4,6,-1), (6,6,0)).unwrap();
    
    fd = fd.mov((2,2,1), (4,2,0)).unwrap();
    fd = fd.remove((6,4,-1)).unwrap();
    return fd;
}


fn place_tst(mut fd:State)->State{
    fd = fd.place((4,2,0)).unwrap();
    fd = fd.place((7,4,0)).unwrap();
    
    fd = fd.place((1,1,0)).unwrap();
    fd = fd.place((7,1,0)).unwrap();
    
    fd = fd.place((4,3,0)).unwrap();
    fd = fd.place((7,7,0)).unwrap();
    fd = fd.remove((1,1,1)).unwrap();
    
    fd = fd.place((1,4,0)).unwrap();
    fd = fd.place((3,3,0)).unwrap();
    
    fd = fd.place((5,5,0)).unwrap();
    fd = fd.place((5,3,0)).unwrap();
    
    fd = fd.place((2,6,0)).unwrap();
    fd = fd.place((4,6,0)).unwrap();

    fd = fd.place((1,7,0)).unwrap();
    fd = fd.place((4,7,0)).unwrap();
    
    fd = fd.place((3,5,0)).unwrap();
    fd = fd.place((6,4,0)).unwrap();
    
    fd = fd.place(( 2,2,0)).unwrap();
    fd = fd.place((3,4,0)).unwrap();

    return fd;
}

