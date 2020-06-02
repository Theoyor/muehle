

pub mod base{
    use heapless::Vec;
    use heapless::consts::U24;

    pub enum PlayMode{
        Place,
        Move,
        Jump,
    }

    pub struct State{
        //* (x,y,p) ist ein field mit x: x-Achse des Feldes, y: y-Achse des Feldes, und bei p = 0 ist Field unbesetzt
        //* p = 1 von Spieler 1 besetzt und bei p = -1 von Spieler 2 (bzw. dem Computer) besetzt

        pub board: Vec<(i8,i8,i8), U24>,
        pub p1_mode: PlayMode,
        pub p2_mode: PlayMode,
        pub turn: i8,
    }

    impl State{
        pub fn new() -> State{
            //* Gibt einen State mit "leerem" board zurück. Sortiert nach Spalten.
            let mut xs: Vec<(i8,i8,i8), U24> = Vec::new();
            xs.extend_from_slice(&[(1, 1, 0), (1, 4, 0), (1, 7, 0), (2, 2, 0), (2, 4, 0), (2, 6, 0), (3, 3, 0), (3, 4, 0), (3, 5, 0), (4, 1, 0), (4, 2, 0), (4, 3, 0), (4, 5, 0), (4, 6, 0), (4, 7, 0), (5, 3, 0), (5, 4, 0), (5, 5, 0), (6, 2, 0), (6, 4, 0), (6, 6, 0), (7, 1, 0), (7, 4, 0), (7, 7, 0)]).unwrap();


            State{
                board: xs,
                p1_mode: PlayMode::Place,
                p2_mode: PlayMode::Place,
                turn: 1, //TODO evtl. Seiten auswaehlen
            }
        }

        pub fn sort_by_lines(&self) -> Vec<(i8,i8,i8), U24>{
            let mut ret: Vec<(i8,i8,i8), U24> = Vec::new();
            for i in 1..8{
                for field in &self.board {
                    if field.2 == i{
                        ret.push(*field);
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
                    println!("{:?} ", field.2);
                    i = 0;
                }
            }

        }

        
        pub fn change(&mut self, x:i8, y:i8, player:i8){
            for field in &mut self.board{
                if field.0 == x && field.1 == y{

                }
            }
        }

        pub fn spot_muehle(&self,fd:(i8,i8,i8))->bool{
            let mut x_counter:i8= 0;
            let mut y_counter:i8= 0;
            //* Zählt die Felder mit den gleichen x oder y Koordinaten und dem selben Spieler. Wenn der Counter 3 ist ist der Stein Teil einer Mühle.
            for field in &self.board {
                if field.2 == fd.2 {
                    if field.0 == fd.0 {
                        x_counter += 1;
                    }
                    if field.1 == fd.1 {
                        y_counter += 1;
                    }
                }
            }
            if x_counter>2 || y_counter>2 {
                return true
            } else {
                return false
            }

        }

        pub fn move_control(&self,from:(i8,i8,i8),to:(i8,i8,i8)) -> bool{
            // wenn das Feld besetzt ist
            if to.2 != 0 {
                return false
            }
            // wenn ein falscher Stein bewegt werden soll
            if self.turn != from.2 {
                return false
            }
            // wenn das Zielfeld das Ursprungsfeld berührt
            if from.0 == to.0 || from.1 == to.1 {
                return true
            }
            // wenn im Jumpmode true sonst false
            if self.turn == 1  {
                match self.p1_mode {
                    PlayMode::Place => return false,
                    PlayMode::Move => return false,
                    PlayMode::Jump => return true
                }
            }
            if self.turn == -1 {
                match self.p2_mode {
                    PlayMode::Place => return false,
                    PlayMode::Move => return false,
                    PlayMode::Jump => return true
                }
            } else {
                return false
            }
        }



    }


}