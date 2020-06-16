
pub mod action{
    use super::super::base::base::State;
    use super::super::base::base::PlayMode;
    use std::cmp;

    pub fn descend(depth: i8, state: State )->i8{ //möglicherweise zu i16 ändern 
        if state.p1_mode == PlayMode::Won{
            return 100; //Hardcode ist lit
        }else if state.p2_mode == PlayMode::Won{
            return -100;
        }

        if depth == 0{
            return state.spielstandbewertung();
        }

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
                                maxeval = super::super::max_three(descend( depth-1 ,state.ki_mov(a, *field)), 
                                descend( depth-1 ,state.ki_mov(b, *field)), 
                                descend( depth-1 ,state.ki_mov(c, *field)));
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
                _=>{}
                    
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
                                mineval = super::super::min_three(descend( depth-1 ,state.ki_mov(a, *field)), 
                                    descend( depth-1 ,state.ki_mov(b, *field)), 
                                    descend( depth-1 ,state.ki_mov(c, *field)));
                            }
                        }
                    },

                PlayMode::Move =>{
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == -1{
                                    mineval = cmp::max(descend(depth-1, state.ki_mov(fd, *field)), mineval);
                                }
                            }
                        }
                    }
                },
                PlayMode::Place(n) =>{
                    for field in &state.board{
                        if field.2 == 0 {
                        mineval = cmp::max(descend(depth-1,state.ki_place(*field)), mineval);
                        }  
                    }
                },
                _=>{}
                    
            }

        return  mineval;

        }
        return 0;

    }

}