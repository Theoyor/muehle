mod state;
use state::state::State;

fn main() {
    println!("Hello, world!");
    let fd = State::new();
    println!("{}",State::spotMuehle(fd,(1,1,1)));
}
