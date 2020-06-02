pub mod state{
    pub struct State{
        //* (x,y,p) ist ein field mit x: x-Achse des Feldes, y: y-Achse des Feldes, und bei p = 0 ist Field unbesetzt
        //* p = 1 von Spieler 1 besetzt und bei p = -1 von Spieler 2 (bzw. dem Computer) besetzt

        pub board: Vec<(i8,i8,i8)>
    }

    impl State{
        pub fn new() -> State{
            //* Gibt einen State mit "leerem" board zurück. Sortiert nach Spalten.
            State{
                board : vec![(1,1,0),(1,4,0),(1,7,0),(2,2,0),(2,4,0),(2,6,0),(3,3,0),(3,4,0),(3,5,0),(4,1,0),(4,2,0),(4,3,0),(4,5,0),(4,6,0),(4,7,0),(5,3,0),(5,4,0),(5,5,0),(6,2,0),(6,4,0),(6,6,0),(7,1,0),(7,4,0),(7,7,0)]
            }
        }

        pub fn sort_by_lines(&self) -> Vec<(i8,i8,i8)>{
            let mut ret: Vec<(i8,i8,i8)> = Vec::new(); 
            for i in 1..8{
                for field in &self.board {
                    if field.2 == i{
                        ret.push(*field)
                    } 
                }
            }
            return ret;
        }

        pub fn printm(&self){
            let lined_board = self.sort_by_lines();
            let mut i = 0;
            for field in lined_board {
                if i<=1{
                    print!("{:?} ",field.2 );
                    i += 1;
                }else if i == 2{
                    println!("{:?}", field.2);
                    i = 0;
                }
            }

        }

        pub fn place(&mut self, x:i8, y:i8)->bool{
            

            return true;
        }

        pub fn spotMuehle(std:State,fd:(i8,i8,i8))->bool{
            let mut xCounter:i8= 0;
            let mut yCounter:i8= 0;
            for field in std.board.iter() {
                if field.2 == fd.2 {
                    if field.0 == fd.0 {
                        xCounter += 1;
                    }
                    if field.1 == fd.1 {
                        yCounter += 1;
                    }
                }
            }
            if xCounter>2 || yCounter>2 {
                return true
            } else {
                return false
            }

        }
    }


}