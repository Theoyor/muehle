use ggez;
use ggez::{Context};
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::conf;
//use crate::action::action::start as strt;
use std::time::{SystemTime};
mod base;   
use base::base::State;
mod action;
use action::action as act;
use crate::base::base::PlayMode::{Place, Move, Jump};
use crate::base::base::PlayMode;
use num_cpus;


pub fn main() {
    let cpus = num_cpus::get();
    print!("CPUs: {} \n",cpus);
    start().expect("Spiel konnte nicht gestartet werden");
}


struct MainState {
    mouse_down: bool,
    real_state: State,
    real_input: PlayerInput,
    wait_ticks: u8,
    players: u8,
}

#[derive(Clone)]
struct PlayerInput {
    down: usize,
    up: usize,
}



impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            mouse_down: false,
            real_state:State::new(),
            real_input: PlayerInput{down:27, up:27},
            wait_ticks: 0,
            players: 1,
        };
        Ok(s)
    }
}

pub fn field_to_coordinates(fd:(i8, i8, i8)) -> (f32, f32, i8) {
    match fd {
        (1, 1, x) => return (100.0,500.0,x),
        (1, 4, x) => return (100.0,300.0,x),
        (1, 7, x) => return (100.0,100.0,x),

        (2, 2, x) => return (175.0,425.0,x),
        (2, 4, x) => return (175.0,300.0,x),
        (2, 6, x) => return (175.0,175.0,x),

        (3, 3, x) => return (250.0,350.0,x),
        (3, 4, x) => return (250.0,300.0,x),
        (3, 5, x) => return (250.0,250.0,x),

        (4, 1, x) => return (300.0,500.0,x),
        (4, 2, x) => return (300.0,425.0,x),
        (4, 3, x) => return (300.0,350.0,x),

        (4, 5, x) => return (300.0,250.0,x),
        (4, 6, x) => return (300.0,175.0,x),
        (4, 7, x) => return (300.0,100.0,x),
        (5, 3, x) => return (350.0,350.0,x),
        (5, 4, x) => return (350.0,300.0,x),
        (5, 5, x) => return (350.0,250.0,x),

        (6, 2, x) => return (425.0,425.0,x),
        (6, 4, x) => return (425.0,300.0,x),
        (6, 6, x) => return (425.0,175.0,x),

        (7, 1, x) => return (500.0,500.0,x),
        (7, 4, x) => return (500.0,300.0,x),
        (7, 7, x) => return (500.0,100.0,x),
                
        _ => return (-1.0,-1.0,-1)
    }


}

pub fn coords_to_index(fd:(f32, f32)) -> usize {
    let mut fi: (i16, i16) = (0,0);
    let x: i16 = fd.0 as i16;
    let y: i16 = fd.1 as i16;
    fi.0 = x;
    fi.1 = y;
    match fi {
        (82..=118, 482..=518) => return 0,
        (82..=118, 282..=318) => return 1,
        (82..=118, 82..=118) => return 2,

        (157..=193, 407..=443) => return 3,
        (157..=193, 282..=318) => return 4,
        (157..=193, 157..=193) => return 5,

        (232..=268, 332..=368) => return 6,
        (232..=268, 282..=318) => return 7,
        (232..=268, 232..=268) => return 8,

        (282..=318, 482..=518) => return 9,
        (282..=318, 407..=443) => return 10,
        (282..=318, 332..=368) => return 11,

        (282..=318, 232..=268) => return 12,
        (282..=318, 157..=193) => return 13,
        (282..=318, 82..=118) => return 14,

        (332..=368, 332..=368) => return 15,
        (332..=368, 282..=318) => return 16,
        (332..=368, 232..=268) => return 17,

        (407..=443, 407..=443) => return 18,
        (407..=443, 282..=318) => return 19,
        (407..=443, 157..=193) => return 20,

        (482..=518, 482..=518) => return 21,
        (482..=518, 282..=318) => return 22,
        (482..=518, 82..=118) => return 23,
        (630..=750, 160..=200) => return 24,
        (765..=885, 160..=200) => return 25,
        (615..=675, 270..=295) => return 26,




        _ => return 27
    }
}



impl event::EventHandler for MainState {


    fn draw(&mut self ,ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [51.0, 118.0, 29.0, 1.0].into());
    

        //Äußerer Ring:
        let rectsize1 = graphics::Rect::new(100.0, 100.0, 400.0, 400.0);
        let rect1 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            rectsize1, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect1, (na::Point2::new(0.0, 0.0),))?;

        //mittlerer Ring:
        let rectsize2 = graphics::Rect::new(175.0, 175.0, 250.0, 250.0);
        let rect2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            rectsize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect2, (na::Point2::new(0.0, 0.0),))?;


        //innerer Ring:
        let rectsize2 = graphics::Rect::new(250.0, 250.0, 100.0, 100.0);
        let rect2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            rectsize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect2, (na::Point2::new(0.0, 0.0),))?;
        
        //Verbindungslinien:

        let line1size = graphics::Rect::new(100.0, 300.0, 150.0, 0.0);
        let line1 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            line1size, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &line1, (na::Point2::new(0.0, 0.0),))?;

        let line2size = graphics::Rect::new(350.0, 300.0, 150.0, 0.0);
        let line2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            line2size, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &line2, (na::Point2::new(0.0, 0.0),))?;

        let line3size = graphics::Rect::new(300.0, 350.0, 0.0, 150.0);
        let line3 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            line3size, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &line3, (na::Point2::new(0.0, 0.0),))?;

        let line4size = graphics::Rect::new(300.0, 100.0, 0.0, 150.0);
        let line4 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            line4size, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &line4, (na::Point2::new(0.0, 0.0),))?;
        
        
        //Rahmen: 

        let rahmensize = graphics::Rect::new(75.0, 75.0, 450.0, 450.0);
        let rahmen = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(2.0), 
            rahmensize, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rahmen, (na::Point2::new(0.0, 0.0),))?;

        let rahmensize2 = graphics::Rect::new(70.0, 70.0, 460.0, 460.0);
        let rahmen2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(6.0), 
            rahmensize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rahmen2, (na::Point2::new(0.0, 0.0),))?;
        
        //Spielsteine:

        for pos in &self.real_state.board {
            
            let steinkoordinaten = field_to_coordinates(*pos);

            if steinkoordinaten.2 == (-1) {
                let spielstein= graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), na::Point2::new(steinkoordinaten.0, steinkoordinaten.1), 18.0, 0.1, graphics::BLACK).unwrap();
                graphics::draw(ctx, &spielstein, (na::Point2::new(0.0, 0.0),))?;
            }
            else if steinkoordinaten.2 == (1) {
                let spielstein= graphics::Mesh::new_circle(ctx, graphics::DrawMode::stroke(4.0), na::Point2::new(steinkoordinaten.0, steinkoordinaten.1), 18.0, 0.1, graphics::BLACK).unwrap();
                graphics::draw(ctx, &spielstein, (na::Point2::new(0.0, 0.0),))?;
                
                let spielstein= graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), na::Point2::new(steinkoordinaten.0, steinkoordinaten.1), 16.0, 0.1, graphics::WHITE).unwrap();
                graphics::draw(ctx, &spielstein, (na::Point2::new(0.0, 0.0),))?;
            }
            

        }

        //Spielinformationen:
        let inforahmensize = graphics::Rect::new(610.0, 100.0, 285.0, 200.0);
        let inforahmen = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(4.0), 
            inforahmensize, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &inforahmen, (na::Point2::new(0.0, 0.0),))?;

        let inforahmensize2 = graphics::Rect::new(615.0, 105.0, 275.0, 190.0);
        let inforahmen2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(2.0), 
            inforahmensize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &inforahmen2, (na::Point2::new(0.0, 0.0),))?;
        
        
        match &self.real_state.p1_mode {
            Place(0) => {
                        //Auswahlfrage
                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(650.0, 120.0);
                        let text_str = format!("Gegner wählen:");
                        let text_display = graphics::Text::new((text_str, textfont, 32.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                        
                        
                        //Auswahl-Spieler
                        let auswahlrahmensize = graphics::Rect::new(630.0, 160.0, 120.0, 40.0);
                        let auswahlrahmen = graphics::Mesh::new_rectangle(
                            ctx, 
                            graphics::DrawMode::stroke(2.0), 
                            auswahlrahmensize, 
                            graphics::BLACK
                        )?;
                        graphics::draw(ctx, &auswahlrahmen, (na::Point2::new(0.0, 0.0),))?;

                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(650.0, 170.0);
                        let text_str = format!("Spieler");
                        let text_display = graphics::Text::new((text_str, textfont, 24.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;

                        //Auswahl-Computer
                        let auswahlrahmensize2 = graphics::Rect::new(765.0, 160.0, 120.0, 40.0);
                        let auswahlrahmen2 = graphics::Mesh::new_rectangle(
                            ctx, 
                            graphics::DrawMode::stroke(2.0), 
                            auswahlrahmensize2, 
                            graphics::BLACK
                        )?;
                        graphics::draw(ctx, &auswahlrahmen2, (na::Point2::new(0.0, 0.0),))?;

                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(770.0, 170.0);
                        let text_str = format!("Computer");
                        let text_display = graphics::Text::new((text_str, textfont, 24.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                    }
                        
            _ => {
                    if self.real_state.p1_mode == PlayMode::Won {
                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(670.0, 140.0);
                        let text_str = format!("Weiß hat");
                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                
                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(663.0, 180.0);
                        let text_str = format!("gewonnen!");
                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                    }
                    else if self.real_state.p2_mode == PlayMode::Won{
                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(655.0, 140.0);
                        let text_str = format!("Schwarz hat");
                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                        
                        let textfont = graphics::Font::default();
                        let text_dest = na::Point2::new(663.0, 180.0);
                        let text_str = format!("gewonnen!");
                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                    }
                    else {
                        if self.real_state.turn == 1 {
                        
                            if self.real_state.allowed == true {
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(650.0, 140.0);
                                let text_str = format!("Weiß ist dran");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(650.0, 180.0);
                                let text_str = format!("mit Schlagen");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                            }
                            else {
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(650.0, 140.0);
                                let text_str = format!("Weiß ist dran");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
        
                                match &self.real_state.p1_mode {
                                    Place(_) => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(665.0, 180.0);
                                        let text_str = format!("mit Setzen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    Move => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(650.0, 180.0);
                                        let text_str = format!("mit Bewegen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    Jump => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(660.0, 180.0);
                                        let text_str = format!("mit Springen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    
                                    
                                    _ => {}
                                }
                            }
    
    
    
    
                        }
                        else {
    
                            if self.real_state.allowed == true {
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(628.0, 140.0);
                                let text_str = format!("Schwarz ist dran");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(650.0, 180.0);
                                let text_str = format!("mit Schlagen");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                            }
                            else {
                                let textfont = graphics::Font::default();
                                let text_dest = na::Point2::new(628.0, 140.0);
                                let text_str = format!("Schwarz ist dran");
                                let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
        
                                match &self.real_state.p2_mode {
                                    Place(_) => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(665.0, 180.0);
                                        let text_str = format!("mit Setzen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    Move => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(650.0, 180.0);
                                        let text_str = format!("mit Bewegen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    Jump => {
                                        let textfont = graphics::Font::default();
                                        let text_dest = na::Point2::new(660.0, 180.0);
                                        let text_str = format!("mit Springen");
                                        let text_display = graphics::Text::new((text_str, textfont, 35.0));
                                        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;
                                    }
                                    
                                    
                                    _ => {}
                                }
                            }
    
                        }
    
                    }
            }
            
        }


        //Reset
        let resetrahmensize = graphics::Rect::new(615.0, 270.0, 60.0, 25.0);
        let resetrahmen = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(2.0), 
            resetrahmensize, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &resetrahmen, (na::Point2::new(0.0, 0.0),))?;

        let textfont = graphics::Font::default();
        let text_dest = na::Point2::new(620.0, 273.0);
        let text_str = format!("Reset");
        let text_display = graphics::Text::new((text_str, textfont, 20.0));
        graphics::draw(ctx, &text_display, (text_dest, 0.0, graphics::BLACK))?;


        //Standardrückgabe von draw
        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.real_state.p1_mode {
            PlayMode::Won => {return Ok(());},
            _=> { }
        }
        match self.real_state.p2_mode {
            PlayMode::Won => {return Ok(());},
            _=> { }
        }

        if self.real_state.turn == -1 && self.players==1{
            if self.wait_ticks == 0{
                let sys_time = SystemTime::now();
                self.real_state = act::start(6, self.real_state.clone()).1;
                let difference = sys_time.elapsed();
                println!("Berechnungszeit:{:?}", difference);
                println!("acted");
                //println!("P1-Stones:{},P2-Stones:{},allowed:{} \n", self.real_state.p1_stones, self.real_state.p2_stones, self.real_state.allowed);
            }
            else{
                self.wait_ticks -= 1;
            }
        }



        Ok(())
    }

    

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        //println!("Mouse button pressed");
        //println!("{:?}", coords_to_index((x, y)));
        self.real_input.down= coords_to_index((x, y));

    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        //println!("Mouse button released");
        //println!("{:?}", coords_to_index((x, y)));
        //println!("{:?},{:?}", x,y);
        self.real_input.up= coords_to_index((x, y));
        self.real_state = apply_input(self.real_input.clone(), self.real_state.clone(), self.players.clone()).0;
        self.players = apply_input(self.real_input.clone(), self.real_state.clone(), self.players.clone()).1;
        if self.real_state.turn == -1 && self.players==1{
            self.wait_ticks = 2;
        }
        //println!("P1-Stones:{},P2-Stones:{},allowed:{} \n", self.real_state.p1_stones, self.real_state.p2_stones, self.real_state.allowed);
    }

}

fn apply_input(real_input: PlayerInput, mut real_state: State, mut players: u8) -> (State, u8) {
    if real_input.up == 24 && real_input.down == 24{
        players = 2;
        return (real_state, players);
    }
    if real_input.up == 25 && real_input.down ==25{
        players = 1;
        return (real_state, players);
    }
    if real_input.up == 26 && real_input.down ==26{
        return (State::new(), players);
    }
    if real_input.up == 27 || real_input.down == 27 || players==1 && real_state.turn== -1{
        //println!("Did nothing");
    } else {
        let up: (i8,i8,i8) = real_state.board[real_input.up];
        let down: (i8,i8,i8) = real_state.board[real_input.down];
        if real_state.allowed==true{
            if State::remove_control(&real_state, up)==Ok(true)  && up == down{
                println!("removing");
                match State::remove(&real_state, up){
                    Ok(t) => real_state =t,
                    Err(v)=> println!("{:?}", v),
                }

            }
        }
        else{
            if players==2 && real_state.turn==-1{
                match real_state.p2_mode{
                    Place(_)=>{
                        if up == down && State::place_control(&real_state, down)==Ok(true){
                            println!("placing");
                            match State::place(&real_state, down){
                                Ok(t) => real_state =t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    Move | Jump=>{
                        if State::move_control(&real_state, down, up)==Ok(true){
                            println!("moving");
                            match State::mov(&real_state, down, up){
                                Ok(t) => real_state =t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    _=>{

                    }

                }
            }
            else{
                match real_state.p1_mode{
                    Place(_)=>{
                        if up == down && State::place_control(&real_state, down)==Ok(true){
                            println!("placing");
                            match State::place(&real_state, down){
                                Ok(t) => real_state =t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    Move | Jump=>{
                        if State::move_control(&real_state, down, up)==Ok(true){
                            println!("moving");
                            match State::mov(&real_state, down, up){
                                Ok(t) => real_state =t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    _=>{

                    }

                }
            }
        }
        println!("applied");
    }
    return (real_state, players);
}

pub fn start() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("Muehle", "Rust-Atzen")
        .window_setup(conf::WindowSetup::default().title("Muehle"))
        .window_mode(conf::WindowMode::default().dimensions(900.0, 600.0));
    
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
     event::run(ctx, event_loop, state)



}








pub fn max_three(a:i8,b:i8,c:i8)->i8{
    if a>=b && a>=c {
        return a;
    }
    else if b >= a && b >= c {
        return b;
    }
    else {
        return c;
    }
}

pub fn min_three(a:i8,b:i8,c:i8)->i8{
    if a<=b && a<=c {
        return a;
    }
    else if b <= a && b <= c {
        return b;
    }
    else {
        return c;
    }
}
