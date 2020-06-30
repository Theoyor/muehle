
pub mod action{
    use super::super::base::base::State;
    use super::super::base::base::PlayMode;
    use std::cmp;

    
    pub fn start(depth: i8, state: State )->(i8,State){ //möglicherweise zu i16 ändern 
        
        // Wenn jemand im vorerigen Zug gewonnen hat, wird eine hohe Bewertung ausgegeben
        if state.p1_mode == PlayMode::Won{
            return (110, state); //Hardcode ist lit
        }else if state.p2_mode == PlayMode::Won{
            return (-110, state);
        }

        //hier kommt rekursives Absteigen für entweder maximalen Spieler (1) oder minimalen Spieler (-1)
        if state.turn == 1{
            let mut maxeval:i8 = -100;
            let mut do_this:State = State::new();

            match &state.p1_mode{
                PlayMode::Jump => {
                
                    let mut a :(i8,i8,i8) = (0,0,0);
                    let mut b :(i8,i8,i8) = (0,0,0);
                    let mut c :(i8,i8,i8)= (0,0,0);
                    //sucht die 3 Felder mit den eigenen Steinen heraus
                    for field in &state.board{
                        if field.2 == 1{
                            if a == (0,0,0){
                                a = *field;
                            }else if b == (0,0,0){
                                b = *field;
                            }else if c == (0,0,0){
                                c = *field;
                            }else{
                                print!("1 Error Jump in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)
                            }
                        }
                    }
                    // Testet descend mit allen möglichen Feldern und den drei "eigenen" Feldern
                    for field in &state.board{
                        if field.2 == 0{
                            let evala = descend( depth-1 ,state.ki_mov(a, *field),-100,100);
                            let evalb = descend( depth-1 ,state.ki_mov(b, *field),-100,100);
                            let evalc = descend( depth-1 ,state.ki_mov(c, *field),-100,100);

                            if evala >= evalb && evala >= evalc && evala > maxeval{
                                maxeval= evala;
                                do_this = state.ki_mov(a, *field);
                            } else if evalb >= evalc && evalb > maxeval{
                                maxeval = evalb;
                                do_this = state.ki_mov(b, *field);
                            }else if evalc > maxeval{
                                maxeval = evalc;
                                do_this = state.ki_mov(c, *field);
                            }
                            
                        }
                    }
                    },

                PlayMode::Move =>{
                    //Sucht alle leeren Felder und testet auf diesen Feldern mov von allen benachbarten Feldern, die einen eigenen Stein beherbergen 
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == 1{
                                    let eval =descend(depth-1, state.ki_mov(fd, *field),-100,100);
                                    if eval > maxeval{
                                        maxeval = eval;
                                        do_this = state.ki_mov(fd, *field);
                                    }

                                }
                            }
                        }
                    }
                },

                PlayMode::Place(_) =>{
                    //sucht alle leeren Felder und testet ein plazieren auf sie
                    for field in &state.board{
                      if field.2 == 0 {
                        let eval =descend(depth-1, state.ki_place( *field),-100,100);
                        if eval > maxeval{
                            maxeval = eval;
                            do_this = state.ki_place( *field);
                        }
                      }  
                    }
                },
                _=>{print!("2 Error Max in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)}
                    
            }

        return  (maxeval,do_this);   
        }
        else{
            let mut mineval:i8 = 100;
            let mut do_this:State = State::new();

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
                                print!("3 Error Jump in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)
                            }
                        }
                    }
                        for field in &state.board{
                            if field.2 == 0{
                                let evala = descend( depth-1 ,state.ki_mov(a, *field),-100,100);
                                let evalb = descend( depth-1 ,state.ki_mov(b, *field),-100,100);
                                let evalc = descend( depth-1 ,state.ki_mov(c, *field),-100,100);

                                if evala <= evalb && evala <= evalc && evala < mineval{
                                    mineval = evala;
                                    do_this = state.ki_mov(a, *field);
                                } else if evalb <= evalc && evalb < mineval{
                                    mineval = evalb;
                                    do_this = state.ki_mov(b, *field);
                                }else if evalc < mineval{
                                    mineval = evalc;
                                    do_this = state.ki_mov(c, *field);
                                }

                            }
                        }
                    },

                PlayMode::Move =>{
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == -1{
                                    let eval =descend(depth-1, state.ki_mov(fd, *field),-100,100);
                                    if eval < mineval{
                                        mineval = eval;
                                        do_this = state.ki_mov(fd, *field);
                                    }
                                }
                            }
                        }
                    }
                },
                PlayMode::Place(_) =>{
                    for field in &state.board{
                        if field.2 == 0 {
                            let eval =descend(depth-1, state.ki_place( *field),-100,100);
                            if eval < mineval{
                                mineval = eval;
                                do_this = state.ki_place( *field);
                            }
                        }  
                    }
                },
                _=>{print!("4 Error Min in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)}
                    
            }

            return  (mineval,do_this);
        }
    }
    pub fn descend(depth: i8, state: State, alpha: i8, beta: i8 )->i8{ //möglicherweise zu i16 ändern
        let mut alph = alpha;
        let mut bet = beta;
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
                    //sucht die 3 Felder mit den eigenen Steinen heraus
                    for field in &state.board{
                        if field.2 == 1{
                            if a == (0,0,0){
                                a = *field;
                            }else if b == (0,0,0){
                                b = *field;
                            }else if c == (0,0,0){
                                c = *field;
                            }else{
                                print!("5 Error Jump in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)
                            }
                        }
                    }
                    // Testet descend mit allen möglichen Feldern und den drei "eigenen" Feldern
                    for field in &state.board{
                        if field.2 == 0{
                            let eval = super::super::max_three(descend( depth-1 ,state.ki_mov(a, *field),alph,bet),
                            descend( depth-1 ,state.ki_mov(b, *field),alph,bet),
                            descend( depth-1 ,state.ki_mov(c, *field),alph,bet));
                            maxeval = cmp::max(eval,maxeval);
                            alph = cmp::max(alph, eval);
                            if bet <= alph {
                                break;
                            }
                            
                        }
                    }
                    },

                PlayMode::Move =>{
                    //Sucht alle leeren Felder und testet auf diesen Feldern mov von allen benachbarten Feldern, die einen eigenen Stein beherbergen 
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == 1{
                                    // hier wird die rekursion ausgeführt
                                    let eval :i8 = descend(depth-1, state.ki_mov(fd, *field),alph,bet);
                                    maxeval = cmp::max(eval, maxeval);
                                    alph = cmp::max(alph, eval);
                                    if bet <= alph {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },

                PlayMode::Place(_) =>{
                    //sucht alle leeren Felder und testet ein plazieren auf sie
                    for field in &state.board{
                      if field.2 == 0 {
                          let eval :i8 = descend(depth-1,state.ki_place(*field),alph,bet);
                          maxeval = cmp::max(eval, maxeval);
                          alph = cmp::max(alph,eval);
                          if bet <= alph {
                              break;
                          }
                      }  
                    }
                },
                _=>{print!("6 Error Max in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)}
                    
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
                                print!("7 Error Jump in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)
                            }
                        }
                    }
                        for field in &state.board{
                            if field.2 == 0{
                                let eval = super::super::min_three(descend( depth-1 ,state.ki_mov(a, *field),alph,bet),
                                    descend( depth-1 ,state.ki_mov(b, *field),alph,bet),
                                    descend( depth-1 ,state.ki_mov(c, *field),alph,bet));
                                mineval = cmp::min(eval,mineval);
                                bet = cmp::min(bet,eval);
                                if bet <= alph {
                                    break;
                                }
                            }
                        }
                    },

                PlayMode::Move =>{
                    for field in &state.board{
                        if field.2 == 0{
                            for fd in state.get_neighbor(*field){
                                if fd.2 == -1{
                                    let eval :i8 = descend(depth-1, state.ki_mov(fd, *field),alph,bet);
                                    mineval = cmp::min(eval,mineval);
                                    bet = cmp::min(bet,eval);
                                    if bet <= alph {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },
                PlayMode::Place(_) =>{
                    for field in &state.board{
                        if field.2 == 0 {
                            let eval : i8 = descend(depth-1,state.ki_place(*field),alph,bet);
                            mineval = cmp::min(eval, mineval);
                            bet = cmp::min(bet,eval);
                            if bet <= alph {
                                break;
                            }
                        }  
                    }
                },
                _=>{print!("8 Error Jump in Start \n depth: {}, P1-Stones:{},P2-Stones:{}",depth,state.p1_stones,state.p2_stones)}
                    
            }

        return  mineval;

        }

    }

}