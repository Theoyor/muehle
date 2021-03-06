

pub mod base{
    use std::fmt;
    use heapless::Vec;
    use heapless::consts::U24;
    


    #[derive(Clone, Debug, PartialEq)]
    pub enum PlayMode{
        Place(u8),
        Move,
        Jump,
        Won,
    }

    #[derive(Clone)]
    pub struct State{
        //* (x,y,p) ist ein field mit x: x-Achse des Feldes, y: y-Achse des Feldes, und bei p = 0 ist Field unbesetzt
        //* p = 1 von Spieler 1 besetzt und bei p = -1 von Spieler 2 (bzw. dem Computer) besetzt

        pub board: Vec<(i8,i8,i8), U24>,
        pub p1_mode: PlayMode,
        pub p2_mode: PlayMode,
        pub p1_stones: i8,
        pub p2_stones: i8,
        pub allowed: bool,
        pub turn: i8,
    }

    impl fmt::Debug for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("State")
             .field("p1_mode", &self.p1_mode)
             .field("p2_mode", &self.p2_mode)
             .field("p1_stones", &self.p1_stones)
             .field("p2_stones", &self.p2_stones)
             .field("p1_turn", &self.turn)
             .finish()
        }
    }
    


    impl State{


        pub fn new() -> State{
            //* Gibt einen State mit "leerem" board zurück. Sortiert nach Spalten.
            //! Niemals die Reihenfolge ändern
            let mut xs: Vec<(i8,i8,i8), U24> = Vec::new();
            xs.extend_from_slice(&[(1, 1, 0), (1, 4, 0), (1, 7, 0), (2, 2, 0), (2, 4, 0), (2, 6, 0), (3, 3, 0), (3, 4, 0), (3, 5, 0), (4, 1, 0), (4, 2, 0), (4, 3, 0), (4, 5, 0), (4, 6, 0), (4, 7, 0), (5, 3, 0), (5, 4, 0), (5, 5, 0), (6, 2, 0), (6, 4, 0), (6, 6, 0), (7, 1, 0), (7, 4, 0), (7, 7, 0)]).unwrap();

            State{
                board: xs,
                p1_mode: PlayMode::Place(0),
                p2_mode: PlayMode::Place(0),
                p1_stones: 9,
                p2_stones: 9,
                allowed: false,
                turn: 1, //TODO evtl. Seiten auswaehlen
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

        pub fn spot_muehle(&self,fd:(i8,i8,i8))->i8{
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
                        if field.0 == fd.0 && (field.1 > 4 && fd.1 > 4 || field.1 < 4 && fd.1 < 4) {
                            x_counter += 1;
                        }

                    }
                    // y Koordinate == 4 ist ein Sonderfall
                    else if fd.1 == 4 {
                        if field.0 == fd.0 {
                            x_counter += 1;
                        }
                        if field.1 == fd.1 && (field.0 > 4 && fd.0 > 4 || field.0 < 4 && fd.0 < 4) {
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
            if x_counter>2 && y_counter>2 {
                return 2;
            } else if x_counter > 2 {
                return 1;
            } else if y_counter > 2 {
                return 1;
            } else {
                return 0;
            }
        }

        //

        pub fn spot_pot_muehle(&self, field: (i8,i8,i8)) -> i8{
            //let field = self.coords_to_field(ffield.0, ffield.1).unwrap();
            let mut ret  = 0;
            let neighbors = self.get_neighbor(field);

            if field.2 == 0{
                // Falls das Feld selbst die Lücke ist, muss nur geschaut werden ob mehr als drei Steine mit gleicher Farbe um sie rumstehen 
                let mut neighbor_count = 0;
                for n in neighbors{
                    if n.0 == field.0 && n.2 == field.1{
                        neighbor_count += 1;
                    }
                    if n.1 == field.1 && n.2 == field.1{
                        neighbor_count += 1;
                    }
                }
                if neighbor_count > 2{
                    ret += 1;
                }

            }else{ 
                let mut own_x:u8 = 0;
                let mut free_x = (0,0,0);
                let mut own_y:u8 = 0;
                let mut free_y = (0,0,0);
                //Schaut wieviele eigene Steine neben ihm liegen und setzt, falls es gibt, ein freies Feld auf free_x/free_y
                for n in neighbors{
                    if n.0 == field.0 && n.2 == field.2{
                        own_x += 1;
                    }else if n.0 == field.0 && n.2 == 0{
                        free_x = n;
                    }
                    if n.1 == field.1 && n.2 == field.2{
                        own_y += 1;
                    }else if n.1 == field.1 && n.2 == 0{
                        free_y = n;
                    }
                }

                //println!("{}  {:?}",ownx ,free_x);
                //println!("{}  {:?}",owny ,free_y);

                if own_x == 1 && free_x != (0,0,0){
                    //Falls field genau einen Nachbar auf der x-Achse hat und der andere kein gegner sondern leer ist
                    let mut nb_count = 0;
                    let nb = self.get_neighbor(free_x);
                    for n in nb{
                        if n.1 == free_x.1 || n.0 == free_x.0 && n.2 == field.2{
                        nb_count += 1;
                    }
                    }
                    if nb_count >= 2{
                        ret += 1;
                    }
                }
                //kein else
                if own_y == 1 && free_y != (0,0,0){
                    let mut nb_count = 0;
                    let nb = self.get_neighbor(free_y);
                    for n in nb{
                        if n.1 == free_y.1 || n.0 == free_y.0 && n.2 == field.2{
                            nb_count += 1;
                        }
                    }
                    if nb_count >= 2{
                        ret += 1;
                    }
                }
            }
            return ret;
        }

        //gibt aus, wieviele Steine eines Spielers sich bewegen können
        pub fn movable(&self, player: i8) -> i8{
            let mut ret = 0;
            for field in &self.board{
                if field.2 != player{
                    continue;
                }
                let neighbors = self.get_neighbor(*field);
                for n in neighbors{
                    if n.2 == 0{
                        ret += 1;
                        break;
                    }
                }
            }
            return ret;
        }

        pub fn place_control(&self, plz: (i8,i8,i8)) ->Result<bool,&str>{
            if plz == (0,0,0){
                return Err("Feld existert nicht")
            }
            //Supi 
            if plz.2 == 0{
                return Ok(true);
            //falls das Feld besetzt ist 
            }else {
                return Err("Feld ist besetzt");
            }
        }


        pub fn remove_control(&self, rem: (i8,i8,i8)) ->Result<bool,&str>{
            if rem == (0,0,0){
                return Err("Feld existert nicht")
            }
            // falls rem Teil einer Mühle ist und es freihe Steine gibt
            if self.spot_muehle(rem)> 0{
                // return Error, sobald ein Stein vom gleichen Typ nicht in einer Mühle ist 
                for field in &self.board {
                    if field.2 == rem.2 && self.spot_muehle(*field) == 0{
                        return Err("Stein ist Teil einer Mühle");
                    }
                    
                }


                
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
            
            if from == (0,0,0) || to == (0,0,0){
                return Err("Ein Feld existert nicht")
            }
            // wenn das Feld besetzt ist
            if to.2 != 0 {
                return Err("Feld ist besetzt");
            }
            // wenn ein falscher Stein bewegt werden soll
            if self.turn != from.2 {
                return Err("Das ist nicht dein Stein/Das Feld ist leer!!!");
            }
            // Wenn der Stein nicht bewegt wird
            if from.0 == to.0 && from.1 == to.1{
                return Err("Du musst den Stein schon bewegen")
            }
            // wenn das Zielfeld das Ursprungsfeld berührt
            else if from.0 == to.0 || from.1 == to.1 {
                return Ok(true);
            }
                
            // wenn im Jumpmode true sonst false
            if self.turn == 1  {
                match self.p1_mode {
                    PlayMode::Jump => return Ok(true),
                    _ => return Err("Das Feld ist zuweit weg")
                }
            }
            if self.turn == -1 {
                match self.p2_mode {
                    PlayMode::Jump => return Ok(true),
                    _ => return Err("Das Feld ist zuweit weg")
                }
            }
            return Err("Unbekannt move_control");
        }


        pub fn change(&mut self, tupel:(i8,i8,i8)){
            //gibt Ok(State) mit verändertem State zurück oder einen String-Error
            for field in &mut self.board{
                if field.0 == tupel.0 && field.1 == tupel.1{                
                    *field = (tupel.0, tupel.1, tupel.2);
                }
            }
        }

        // Die angegegbenen Felder sind so, wie sie im Array stehen
        pub fn mov(&self, from: (i8,i8,i8), to: (i8,i8,i8))->Result<State, &str>{
            //* Gibt wenn möglich einen State nach einem move-Zug aus 
            //TODO testen ob gewonnen
            let mut st = self.clone();
            //moven:
            st.change( (from.0,from.1, 0) );
            st.change( (to.0,to.1,st.turn) );
            
            //Falls sich der gegner nicht mehr bewegen kann, hat man gewonnen
            if 0 == self.movable(st.turn*-1){
                if st.turn == 1{
                    st.p1_mode = PlayMode::Won;
                }else{   
                    st.p2_mode = PlayMode::Won;
                }
            }

            //falls Mühle entstanden ist den Zug nicht beenden und in schlagenden Zustand gehen
            if st.spot_muehle((to.0,to.1,st.turn))>0{
                if st.turn == 1{
                    st.allowed = true;
                }else{   
                    st.allowed = true;
                }
            //Andernfalls Zug beenden und Zustand beibehalten
            }else{
                st.turn *= -1;
            }
            return Ok(st);
                
            
        }
        //Bekommt das zu verändernde Feld angegeben
        pub fn remove(&self, field: (i8,i8,i8))->Result<State, &str>{
            //* Gibt wenn möglich einen State nach einem remove-Zug aus
            let mut st = self.clone();
            
            //removen
            st.change((field.0,field.1, 0));
            //schlagen nicht mehr erlauben

            st.allowed = false; 
            if st.turn == 1{

                //Schauen ob p2 in einen anderen Zustand wechseln muss
                st.p2_stones -= 1;
                if st.p2_stones == 3{
                    st.p2_mode = PlayMode::Jump;
                }else if st.p2_stones < 3{
                    st.p1_mode = PlayMode::Won;
                }
            }
            if st.turn == -1{
                
                //schauen ob p1 in einen anderen Zustand muss
                st.p1_stones -= 1;
                if st.p1_stones == 3{
                    st.p1_mode = PlayMode::Jump;
                }else if st.p1_stones < 3{
                    st.p2_mode = PlayMode::Won;
                }  
            }

            //Falls sich der gegner nicht mehr bewegen kann, hat man gewonnen
            if 0 == self.movable(st.turn*-1){
                if st.turn == 1{
                    st.p1_mode = PlayMode::Won;
                }else{   
                    st.p2_mode = PlayMode::Won;
                }
            }
            // Zug beenden
            st.turn *= -1;

            return Ok(st);
        
        }

        //Bekommt das zu verändernde Feld eingegeben, Man gewinnt aktuell leider noch nicht, wenn man nach dem letzten place den Gegner eingeschlossen hat
        pub fn place(&self, field:(i8,i8,i8))->Result<State,&str>{ 
            let mut st = self.clone();

            st.change( (field.0,field.1,st.turn) );

            //Falls Mühle gelegt wurde
            if st.spot_muehle((field.0,field.1,st.turn))>0{
                st.allowed = true;
                if st.turn == 1{
                    // Hier wird gemoved
                    match st.p1_mode {
                        PlayMode::Place( 8) =>st.p1_mode = PlayMode::Move,
                        PlayMode::Place(n) =>st.p1_mode = PlayMode::Place(n+1),
                        _ => {}
                    }   
                }else{
                    // Hier wird gemoved
                    match st.p2_mode {
                        PlayMode::Place(8) =>st.p2_mode = PlayMode::Move,
                        PlayMode::Place(n) =>st.p2_mode = PlayMode::Place(n+1),
                        _ => {}
                    }   
                }
            //Falls keine Mühle gebildet wurde
            }else{
                //Wen 8 Steine gesetzt worden sind, in den Move-Zustand wechseln, sonst den Steine-Counter erhöhen
                if st.turn == 1{
                    match st.p1_mode {
                        PlayMode::Place(8) => st.p1_mode = PlayMode::Move,
                        PlayMode::Place(n) =>st.p1_mode = PlayMode::Place(n+1),
                        _ => {}
                    }   
                }else{
                    match st.p2_mode {
                        PlayMode::Place(8) =>st.p2_mode = PlayMode::Move,
                        PlayMode::Place(n) =>st.p2_mode = PlayMode::Place(n+1),
                        _ => {}
                    }   
                }

                //Zug beenden
                st.turn *= -1;
            }
            return Ok(st);
        
        }


        pub fn spielstandbewertung(&self)->i8 { //möglicherweise zu i16 ändern
            // anzahle der steine verrechnen
            let r: i8 = self.p1_stones - self.p2_stones;

            // anzahl der beweglichen steine verrechnen
            let x = self.movable(1)-self.movable(-1);

            // anzahl der mühlen verrechnen
            let mut y: i8 = 0;
            y += self.spot_muehle(self.board[0])*self.board[0].2;
            y += self.spot_muehle(self.board[3])*self.board[3].2;
            y += self.spot_muehle(self.board[6])*self.board[6].2;
            y += self.spot_muehle(self.board[17])*self.board[17].2;
            y += self.spot_muehle(self.board[20])*self.board[20].2;
            y += self.spot_muehle(self.board[23])*self.board[23].2;
            y += self.spot_muehle(self.board[4])*self.board[4].2;
            y += self.spot_muehle(self.board[10])*self.board[10].2;
            y += self.spot_muehle(self.board[13])*self.board[13].2;
            y += self.spot_muehle(self.board[19])*self.board[19].2;

            // anzahl der pot_mühlen verrechnen
            let mut z: i8 = 0;
            z += self.spot_pot_muehle(self.board[1])*self.board[1].2;
            z += self.spot_pot_muehle(self.board[4])*self.board[4].2;
            z += self.spot_pot_muehle(self.board[7])*self.board[7].2;
            z += self.spot_pot_muehle(self.board[16])*self.board[16].2;
            z += self.spot_pot_muehle(self.board[19])*self.board[19].2;
            z += self.spot_pot_muehle(self.board[22])*self.board[22].2;
            z += self.spot_pot_muehle(self.board[9])*self.board[9].2;
            z += self.spot_pot_muehle(self.board[10])*self.board[10].2;
            z += self.spot_pot_muehle(self.board[11])*self.board[11].2;
            z += self.spot_pot_muehle(self.board[12])*self.board[12].2;
            z += self.spot_pot_muehle(self.board[13])*self.board[13].2;
            z += self.spot_pot_muehle(self.board[14])*self.board[14].2;


            /*println!("{}", x);
            println!("{}", y);
            println!("{}", z); */
            return 2*x+y+z+6*r;
        }


        // Die angegegbenen Felder sind so, wie sie im Array stehen
        pub fn ki_mov(&self, from: (i8,i8,i8), to: (i8,i8,i8))->State{
            //* Gibt wenn möglich einen State nach einem move-Zug aus 
            //TODO testen ob gewonnen
            let mut st = self.clone();
            //moven:
            st.change( (from.0,from.1, 0) );
            st.change( (to.0,to.1,st.turn) );
            
            //Falls sich der gegner nicht mehr bewegen kann, hat man gewonnen
            if 0 == self.movable(st.turn*-1){
                if st.turn == 1{
                    st.p1_mode = PlayMode::Won;
                }else{   
                    st.p2_mode = PlayMode::Won;
                }
            }

            //falls Mühle entstanden ist den "besten"Stein schlagen
            if st.spot_muehle((to.0,to.1,st.turn))>0{
                st.allowed = true;
            }
            if st.turn == 1{

                //Schauen ob p2 in einen anderen Zustand muss
                //st.p2_stones -= 1;
                if st.p2_stones == 3{
                    st.p2_mode = PlayMode::Jump;
                }else if st.p2_stones < 3{
                    st.p1_mode = PlayMode::Won;
                }
            }
            if st.turn == -1{
                
                //schauen ob p1 in einen anderen Zustand muss
                //st.p1_stones -= 1;
                if st.p1_stones == 3{
                    st.p1_mode = PlayMode::Jump;
                }else if st.p1_stones < 3{
                    st.p2_mode = PlayMode::Won;
                }  
            }
            if !st.allowed {
                st.turn *= -1;
            }
            return st;
        }

        pub fn ki_remove(&self, field:(i8,i8,i8))->State{
            let mut st = self.clone();
            st.change( (field.0,field.1,0) );
            st.allowed = false;

             //Falls sich der gegner nicht mehr bewegen kann, hat man gewonnen
             if 0 == self.movable(st.turn*-1){
                if st.turn == 1{
                    st.p1_mode = PlayMode::Won;
                }else{   
                    st.p2_mode = PlayMode::Won;
                }
            }

            if st.turn == -1 {
                st.p1_stones = st.p1_stones - 1;
            }if st.turn == 1 {
                st.p2_stones = st.p2_stones -1;
            }
            st.turn *= -1;
            return st;
        }
            //Bekommt das zu verändernde Feld eingegeben, Man gewinnt aktuell leider noch nicht, wenn man nach dem letzten place den Gegner eingeschlossen hat
        pub fn ki_place(&self, field:(i8,i8,i8))->State{ 
            let mut st = self.clone();
            st.change( (field.0,field.1,st.turn) );

            //Falls Mühle gelegt wurde
            if st.spot_muehle((field.0,field.1,st.turn))>0{
                st.allowed=true;
            }

            //Wen 8 Steine gesetzt worden sind, in den Move-Zustand wechseln, sonst den Steine-Counter erhöhen
            if st.turn == 1{
                match st.p1_mode {
                    PlayMode::Place(8) => st.p1_mode = PlayMode::Move,
                    PlayMode::Place(n) =>st.p1_mode = PlayMode::Place(n+1),
                    _ => {}
                }   
            }else{
                match st.p2_mode {
                    PlayMode::Place(8) =>st.p2_mode = PlayMode::Move,
                    PlayMode::Place( n) =>st.p2_mode = PlayMode::Place(n+1),
                    _ => {}
                }   
            }

            //Zug beenden
            if !st.allowed {
                st.turn *= -1;
            }
            return st;
        
        }

        pub fn steine_schlagen(&self) ->(i8, i8, i8){
            let mut best_stone: (i8, i8, i8) = (0, 0, 0);
            let mut best_stone_value: i8 = 0;
            let mut best_stone_muehle: (i8, i8, i8) = (0, 0, 0);
            let mut best_stone_value_muehle: i8 = 0;
            for field in &self.board {
                let mut stone_value: i8 = 1;
                let mut stone_value_muehle: i8 = 1;
                if  field.2 == 0 || field.2 == -1 {
                    continue;
                }
                if self.spot_muehle(*field)>0 {
                    if self.spot_pot_muehle(*field)>0 {
                        stone_value_muehle = stone_value_muehle + self.spot_pot_muehle(*field);
                    }

                }
                else if self.spot_pot_muehle(*field)>0 {
                    stone_value = stone_value + self.spot_pot_muehle(*field);
                }
                if stone_value > best_stone_value {
                    best_stone = *field;
                    best_stone_value = stone_value;
                }
                if stone_value_muehle > best_stone_value_muehle {
                    best_stone_muehle = *field;
                    best_stone_value_muehle = stone_value_muehle;
                }

            }
            // Nicht unbedingt sehr intelligent fehlt mindestens noch sowas wie nicht bewegliche Steine
            // guckt nur ob teil einer potenziellen mühle bzw ob er überhaupt geschlagen werden darf
            if best_stone != (0, 0, 0) {
                return best_stone;
            }
            else {
                return best_stone_muehle;
            }
        }



        
    }


}