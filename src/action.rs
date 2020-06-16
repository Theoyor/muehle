
pub mod action{
    use super::super::base::base::State;
    use super::super::base::base::PlayMode;
    use std::cmp;

    pub fn descend(depth: i8, state: State )->i8{ //möglicherweise zu i16 ändern 
        
        // Wenn jemand im vorerigen Zug gewonnen hat, wird eine hohe Bewertung ausgegeben
        if state.p1_mode == PlayMode::Won{
            return 110; //Hardcode ist lit
        }else if state.p2_mode == PlayMode::Won{
            return -110;
        }
        // Wenn Suchtiefe ausgeschöpft, führe Spielstandsbwerzúng durch
        if depth == 0{
            return state.spielstandbewertung();
        }

        //hier kommt rekursives Absteigen für entweder maximalen Spieler (1) oder minimalen Spieler (-1)
        if state.turn == 1{
            let mut maxeval:i8 = -100;
            


            match &state.p1_mode{
                PlayMode::Jump => {
                
                    let mut a :(i8,i8,i8) = (0,0,0);
                    let mut b :(i8,i8,i8) = (0,0,0);
                    let mut c :(i8,i8,i8)= (0,0,0);
                    
                    for field in &state.board{
                        if field.2 == 1{
                            if a == (0,0,0){
                                a = *field;
                            }else if b == (0,0,0){
                                b = *field;
                            }else if c == (0,0,0){
                                c = *field;
                            }else{
                                print!("Hier ist ein Fehler Jump")
                            }
                        }
                    }
                        for field in &state.board{
                            if field.2 == 0{
                                let ma = super::super::max_three(descend( depth-1 ,state.ki_mov(a, *field)), 
                                descend( depth-1 ,state.ki_mov(b, *field)), 
                                descend( depth-1 ,state.ki_mov(c, *field)));
                                maxeval = cmp::max(ma,maxeval);
                                
                            }
                        }
                    },

                PlayMode::Move =>{
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == 1{
                                    maxeval = cmp::max(descend(depth-1, state.ki_mov(fd, *field)), maxeval);
                                }
                            }
                        }
                    }
                },
                PlayMode::Place(n) =>{
                    for field in &state.board{
                      if field.2 == 0 {
                        maxeval = cmp::max(descend(depth-1,state.ki_place(*field)), maxeval);
                      }  
                    }
                },
                _=>{println!("alter was geht max")}
                    
            }

        return  maxeval;   
        }
        else{
            let mut mineval:i8 = 100;

            match &state.p2_mode{
                PlayMode::Jump => {
                
                    let mut a :(i8,i8,i8) = (0,0,0);
                    let mut b :(i8,i8,i8) = (0,0,0);
                    let mut c :(i8,i8,i8)= (0,0,0);
                    
                    for field in &state.board{
                        if field.2 == -1{
                            if a == (0,0,0){
                                a = *field;
                            }else if b == (0,0,0){
                                b = *field;
                            }else if c == (0,0,0){
                                c = *field;
                            }else{
                                print!("Hier ist ein Fehler Jump")
                            }
                        }
                    }
                        for field in &state.board{
                            if field.2 == 0{
                                let mi = super::super::min_three(descend( depth-1 ,state.ki_mov(a, *field)), 
                                    descend( depth-1 ,state.ki_mov(b, *field)), 
                                    descend( depth-1 ,state.ki_mov(c, *field)));
                                mineval = cmp::min(mi,mineval);
                            }
                        }
                    },

                PlayMode::Move =>{
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == -1{
                                    mineval = cmp::min(descend(depth-1, state.ki_mov(fd, *field)), mineval);
                                }
                            }
                        }
                    }
                },
                PlayMode::Place(n) =>{
                    for field in &state.board{
                        if field.2 == 0 {
                        mineval = cmp::min(descend(depth-1,state.ki_place(*field)), mineval);
                        }  
                    }
                },
                _=>{println!("alter was geht min")}
                    
            }

        return  mineval;

        }
        return 0;

    }

}