mod state;
use state::state::State;

fn main() {
    println!("Hello, world!");
    let fd = State::new();
    println!("{}",fd.spotMuehle((1,1,1)));
}
