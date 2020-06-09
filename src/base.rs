

pub mod base{
    use heapless::Vec;
    use heapless::consts::U24;
    


    #[derive(Clone)]
    pub enum PlayMode{
        // Der bool ist true: Schlagen ist erlaubt, andernfalls nicht
        Place(bool),
        Move(bool),
        Jump(bool),
        Won,
        Lost,
    }

    #[derive(Clone)]
    pub struct State{
        //* (x,y,p) ist ein field mit x: x-Achse des Feldes, y: y-Achse des Feldes, und bei p = 0 ist Field unbesetzt
        //* p = 1 von Spieler 1 besetzt und bei p = -1 von Spieler 2 (bzw. dem Computer) besetzt

        pub board: Vec<(i8,i8,i8), U24>,
        pub p1_mode: PlayMode,
        pub p2_mode: PlayMode,
        pub p1_stones: u8,
        pub p2_stones: u8,
        pub turn: i8,
    }

    impl State{


        pub fn new() -> State{
            //* Gibt einen State mit "leerem" board zurück. Sortiert nach Spalten.
            //! Niemals die Reihenfolge ändern
            let mut xs: Vec<(i8,i8,i8), U24> = Vec::new();
            xs.extend_from_slice(&[(1, 1, 0), (1, 4, 0), (1, 7, 0), (2, 2, 0), (2, 4, 0), (2, 6, 0), (3, 3, 0), (3, 4, 0), (3, 5, 0), (4, 1, 0), (4, 2, 0), (4, 3, 0), (4, 5, 0), (4, 6, 0), (4, 7, 0), (5, 3, 0), (5, 4, 0), (5, 5, 0), (6, 2, 0), (6, 4, 0), (6, 6, 0), (7, 1, 0), (7, 4, 0), (7, 7, 0)]).unwrap();

            State{
                board: xs,
                p1_mode: PlayMode::Place(false),
                p2_mode: PlayMode::Place(false),
                p1_stones: 9,
                p2_stones: 9,
                turn: 1, //TODO evtl. Seiten auswaehlen
            }
        }
        /* Unwichtig geworden, falls von jemand anderem benötigt, kann es wieder auskommentiert werden

        pub fn sort_by_lines(&self) -> Vec<(i8,i8,i8), U24>{
            let mut ret: Vec<(i8,i8,i8), U24> = Vec::new();
            for i in 1..8{
                for field in &self.board {
                    if field.1 == i{
                        match ret.push(*field){
                            Ok(_) =>{},
                            Err(n) => println!("{:?} konnte nicht angehängt werden", n)
                        }; 
                    } 
                }
            }
            return ret;
        }
        */
        //*evtl nicht nötig
        pub fn coords_to_field(&self, x:i8, y:i8 )->Result<(i8,i8,i8),&str>{
            for field in &self.board{
                if field.0 == x && field.1 == y{
                    return Ok(*field);
                }
            }
            return Err("Existiert nicht")
        }
        

        pub fn printm(&self){
            
            for x in 1..8{
                for y in 1..8 {
                    let mut cont = false;
                    for field in &self.board{
                        if field.0 == y && field.1 ==x{
                            print!("{}", field.2);
                            cont = true;
                            break;
                        }
                    }
                    if !cont{
                        print!(" ")
                    }
                }
                println!{""};
            }
        }


        pub fn spot_muehle(&self,fd:(i8,i8,i8))->bool{
            let mut x_counter:i8= 0;
            let mut y_counter:i8= 0;
            //* Zählt die Felder mit den gleichen x oder y Koordinaten und dem selben Spieler. 
            //* Wenn der Counter 3 ist ist der Stein Teil einer Mühle.
            for field in &self.board {
                // Felder mit selbem Stein besetzt
                if field.2 == fd.2 {
                    // x Koordinate == 4 ist ein Sonderfall
                    if fd.0 == 4 {
                        if field.1 == fd.1 {
                            y_counter += 1;
                        }
                        if field.0 == fd.0 && (field.1 - fd.1).abs() < 3 {
                            x_counter += 1;
                        }
                    }
                    // y Koordinate == 4 ist ein Sonderfall
                    else if fd.1 == 4 {
                        if field.0 == fd.0 {
                            x_counter += 1;
                        }
                        if field.1 == fd.1 && (field.0 - fd.0).abs() < 3 {
                            y_counter += 1;
                        }
                    }
                        // wenn es kein Sonderfall ist
                    else {
                        if field.0 == fd.0 {
                            x_counter += 1;
                        }
                        if field.1 == fd.1 {
                            y_counter += 1;
                        }
                    }
                }
            }
            if x_counter>2 || y_counter>2 {
                return true
            } else {
                return false
            }
        }


        pub fn place_control(&self, plz: (i8,i8,i8)) ->Result<bool,&str>{
            //Supi 
            if plz.2 == 0{
                return Ok(true);
            //falls das Feld besetzt ist 
            }else {
                return Err("Feld ist besetzt");
            }
        }


        pub fn remove_control(&self, rem: (i8,i8,i8)) ->Result<bool,&str>{
            // falls rem Teil einer Mühle ist
            if self.spot_muehle(rem){
                return Err("Stein ist Teil einer Mühle");
            }
            //falls das Feld leer ist 
            if rem.2 == 0{
                return Err("Feld ist leer");
            //falls versucht wird den gleichen zu schlagen 
            }else if rem.2 == self.turn{
                return Err("Willst du wirklich deienen eigenen Stein schlagen? Du Horst");
            //passt
            }else{
                return Ok(true);
            }
        } 


        pub fn move_control(&self,from:(i8,i8,i8),to:(i8,i8,i8)) ->Result<bool,&str>{
            // wenn das Feld besetzt ist
            if to.2 != 0 {
                return Err("Feld ist besetzt");
            }
            // wenn ein falscher Stein bewegt werden soll
            if self.turn != from.2 {
                return Err("Das ist nicht dein Stein !!!");
            }
            // wenn das Zielfeld das Ursprungsfeld berührt
            if from.0 == to.0 || from.1 == to.1 {
                return Ok(true);
            }
            // wenn im Jumpmode true sonst false
            if self.turn == 1  {
                match self.p1_mode {
                    PlayMode::Jump(b) => return Ok(true),
                    _ => return Err("Das Feld ist zuweit weg")
                }
            }
            if self.turn == -1 {
                match self.p2_mode {
                    PlayMode::Jump(b) => return Ok(true),
                    _ => return Err("Das Feld ist zuweit weg")
                }
            }
            return Err("Unbekannt move_control")
        }


        pub fn change(&mut self, tupel:(i8,i8,i8)){
            //gibt Ok(State) mit verändertem State zurück oder einen String-Error
            for field in &mut self.board{
                if field.0 == tupel.0 && field.1 == tupel.1{                
                    *field = (tupel.0, tupel.1, tupel.2);
                }
            }
        }

    
        pub fn mov(&self, from: (i8,i8,i8), to: (i8,i8,i8))->Result<State, &str>{
            //* Gibt wenn möglich einen State nach einem move-Zug aus 
            //TODO testen ob gewonnen
            let mut st = self.clone();
            match self.move_control(from, to){
                Ok(_) =>{
                    //moven:
                    st.change( (from.0,from.1, 0) );
                    st.change( (to.0,to.1,st.turn) );
                    
                    //falls Mühle entstanden ist den Zug behalten und in schlagenden Zustand gehen
                    if st.spot_muehle((to.0,to.1,st.turn)){
                        if st.turn == 1{
                            st.p1_mode = PlayMode::Move(true);
                        }else{   
                            st.p2_mode = PlayMode::Move(true);
                        }
                    //Andernfalls Zug wechseln und Zustand beibehalten
                    }else{
                        st.turn *= -1;
                    }
                    return Ok(st);
                },
                Err(msg) => return Err(msg)
            }
            
        }
        
        pub fn remove(&self, field: (i8,i8,i8))->Result<State, &str>{
            //* Gibt wenn möglich einen State nacheinem remove-Zug aus
            let mut st = self.clone();
            match self.remove_control(field){
                Ok(_)=>{
                    //removen
                    st.change((field.0,field.1, 0));
                    if st.turn == 1{
                        match &mut st.p1_mode {
                            //Zustand wieder auf nicht-schlagen setzen
                            PlayMode::Jump(true) => st.p1_mode = PlayMode::Jump(false),
                            PlayMode::Place(true) => st.p1_mode = PlayMode::Place(false),
                            PlayMode::Move(true) => st.p1_mode = PlayMode::Move(false), 
                            _ => return Err("Das sollte unmöglich sein, remove")
                        } 
                        //Schauen ob p2 in einen anderen Zustand wechseln muss
                        st.p2_stones -= 1;
                        if st.p2_stones == 3{
                            st.p2_mode = PlayMode::Jump(false);
                        }else if st.p2_stones < 3{
                            st.p1_mode = PlayMode::Won;
                        }
                    }
                    if st.turn == -1{
                        match &mut st.p2_mode {
                            //Zustand auf nicht-schlagen setzen
                            PlayMode::Jump(true) => st.p1_mode = PlayMode::Jump(false),
                            PlayMode::Place(true) => st.p1_mode = PlayMode::Place(false),
                            PlayMode::Move(true) => st.p1_mode = PlayMode::Move(false), 
                            _ => return Err("Das sollte unmöglich sein, remove")
                        } 
                        //schauen ob p1 in einen anderen Zustand muss
                        st.p1_stones -= 1;
                        if st.p1_stones == 3{
                            st.p1_mode = PlayMode::Jump(false);
                        }else if st.p1_stones < 3{
                            st.p2_mode = PlayMode::Won;
                        }
                    }
                    // Der andere Spieler ist am Zug
                    st.turn *= -1;

                    return Ok(st);
                },
                Err(msg) => return Err(msg)
            }
        }

        pub fn place(&self, field:(i8,i8,i8))->Result<State,&str>{
            //! Noch Fehler vorhanden 
            //TODO Testen ob gewonnen
            let mut st = self.clone();
            match self.place_control(field){
                Ok(_) =>{
                    st.change( (field.0,field.1,st.turn) );
                    if st.spot_muehle((field.0,field.1,st.turn)){
                        if st.turn == 1{
                            st.p1_mode = PlayMode::Move(true); //fehler 
                        }else{
                            st.p2_mode = PlayMode::Move(true); //fehler
                        }
                    }else{
                        st.turn *= -1;
                    }
                    return Ok(st);
                },
                Err(msg) => return Err(msg)
            }
        }


        pub fn get_neighbor(&self, fd:(i8,i8,i8))->Vec<(i8,i8,i8), U24>{
            let mut xs: Vec<(i8,i8,i8), U24> = Vec::new();
            match fd {
                (1,1,_) => {xs.extend_from_slice(&[self.board[1],self.board[9]]).unwrap();},
                (1,4,_) => {xs.extend_from_slice(&[self.board[0],self.board[2],self.board[4]]).unwrap();},
                (1,7,_) => {xs.extend_from_slice(&[self.board[1],self.board[14]]).unwrap();},
                (2,2,_) => {xs.extend_from_slice(&[self.board[4],self.board[10]]).unwrap();},
                (2,4,_) => {xs.extend_from_slice(&[self.board[1],self.board[3],self.board[5],self.board[7]]).unwrap();},
                (2,6,_) => {xs.extend_from_slice(&[self.board[4],self.board[13]]).unwrap();},
                (3,3,_) => {xs.extend_from_slice(&[self.board[11],self.board[7]]).unwrap();},
                (3,4,_) => {xs.extend_from_slice(&[self.board[4],self.board[6],self.board[8]]).unwrap();},
                (3,5,_) => {xs.extend_from_slice(&[self.board[7],self.board[12]]).unwrap();},
                (4,1,_) => {xs.extend_from_slice(&[self.board[0],self.board[10],self.board[21]]).unwrap();},
                (4,2,_) => {xs.extend_from_slice(&[self.board[3],self.board[9],self.board[11],self.board[18]]).unwrap();},
                (4,3,_) => {xs.extend_from_slice(&[self.board[6],self.board[10],self.board[15]]).unwrap();},
                (4,5,_) => {xs.extend_from_slice(&[self.board[8],self.board[13],self.board[17]]).unwrap();},
                (4,6,_) => {xs.extend_from_slice(&[self.board[5],self.board[12],self.board[14],self.board[20]]).unwrap();},
                (4,7,_) => {xs.extend_from_slice(&[self.board[2],self.board[13],self.board[23]]).unwrap();},
                (5,3,_) => {xs.extend_from_slice(&[self.board[11],self.board[16]]).unwrap();},
                (5,4,_) => {xs.extend_from_slice(&[self.board[15],self.board[17],self.board[19]]).unwrap();},
                (5,5,_) => {xs.extend_from_slice(&[self.board[12],self.board[16]]).unwrap();},
                (6,2,_) => {xs.extend_from_slice(&[self.board[10],self.board[19]]).unwrap();},
                (6,4,_) => {xs.extend_from_slice(&[self.board[16],self.board[18],self.board[20],self.board[22]]).unwrap();},
                (6,6,_) => {xs.extend_from_slice(&[self.board[13],self.board[19]]).unwrap();},
                (7,1,_) => {xs.extend_from_slice(&[self.board[9],self.board[22]]).unwrap();},
                (7,4,_) => {xs.extend_from_slice(&[self.board[19],self.board[21],self.board[23]]).unwrap();},
                (7,7,_) => {xs.extend_from_slice(&[self.board[14],self.board[22]]).unwrap();}
                _ => {}
            }
            return xs;


        }
    }


}