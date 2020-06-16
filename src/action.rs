
pub mod action{
    use super::super::base::base::State;
    use super::super::base::base::PlayMode;

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
            let mut maxeval:i8 = 100;
            

            if state.p1_mode == PlayMode::Jump{
                
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
                        descend( depth-1 ,state.mov(a, *field).unwrap());
                        descend( depth-1 ,state.mov(b, *field).unwrap());
                        descend( depth-1 ,state.mov(c, *field).unwrap());
                    }
                }
            } 


        }else{

        }
        return 0;

    }

}