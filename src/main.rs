use ggez;
use ggez::{Context};
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::conf;
use crate::action::action::start as strt;
use std::time::{Duration,SystemTime};
mod base;   
use base::base::State;
mod action;
use action::action as act;
use crate::base::base::PlayMode::{Place, Move, Jump};
use crate::base::base::PlayMode;


pub fn main() {
    let sys_time = SystemTime::now();
    //let mut fd = State::new();

    //fd = place_tst(fd);
    //fd = mov_test(fd);
    //let i = fd.spot_pot_muehle((4,2,1));
    //println!("{}", i);
    //println!("{:?}", fd);
    //fd.spielstandbewertung();
    //println!("{:?}",fd.steineSchlagen());
    start();

    //let tup = strt(6,fd);
    //println!("Bewertung: {}",tup.0);
    //println!("{:?}", tup.1);

    let difference = sys_time.elapsed();
    println!("{:?}", difference);
}


struct MainState {
    mouse_down: bool,
    realState: State,
    realInput: PlayerInput,
    waitTicks: u8,
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
            realState:place_tst(State::new()),
            realInput: PlayerInput{down:24, up:24},
            waitTicks: 0,
            players: 2,
        };
        Ok(s)
    }
}

pub fn fieldToCoordinates(fd:(i8,i8,i8))-> (f32,f32, i8) {
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

pub fn coordsToIndex(fd:(f32,f32))-> usize {
    match fd {
        (82.0..=118.0, 482.0..=518.0) => return 0,
        (82.0..=118.0, 282.0..=318.0) => return 1,
        (82.0..=118.0, 82.0..=118.0) => return 2,

        (157.0..=193.0, 407.0..=443.0) => return 3,
        (157.0..=193.0, 282.0..=318.0) => return 4,
        (157.0..=193.0, 157.0..=193.0) => return 5,

        (232.0..=268.0, 332.0..=368.0) => return 6,
        (232.0..=268.0, 282.0..=318.0) => return 7,
        (232.0..=268.0, 232.0..=268.0) => return 8,

        (282.0..=318.0, 482.0..=518.0) => return 9,
        (282.0..=318.0, 407.0..=443.0) => return 10,
        (282.0..=318.0, 332.0..=368.0) => return 11,

        (282.0..=318.0, 232.0..=268.0) => return 12,
        (282.0..=318.0, 157.0..=193.0) => return 13,
        (282.0..=318.0, 82.0..=118.0) => return 14,

        (332.0..=368.0, 332.0..=368.0) => return 15,
        (332.0..=368.0, 282.0..=318.0) => return 16,
        (332.0..=368.0, 232.0..=268.0) => return 17,

        (407.0..=443.0, 407.0..=443.0) => return 18,
        (407.0..=443.0, 282.0..=318.0) => return 19,
        (407.0..=443.0, 157.0..=193.0) => return 20,

        (482.0..=518.0, 482.0..=518.0) => return 21,
        (482.0..=518.0, 282.0..=318.0) => return 22,
        (482.0..=518.0, 82.0..=118.0) => return 23,

        _ => return 24
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

        for pos in &self.realState.board {
            
            let mut steinkoordinaten = fieldToCoordinates(*pos);

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

        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {

        if self.realState.turn == -1 && self.players==1{
            if self.waitTicks == 0{
                self.realState = act::start(5,self.realState.clone()).1;
            }
            else{
                self.waitTicks -= 1;
            }
        }



        Ok(())
    }

    

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("Mouse button pressed");
        println!("{:?}", coordsToIndex((x,y)));
        self.realInput.down=coordsToIndex((x,y));

    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        println!("Mouse button released");
        println!("{:?}", coordsToIndex((x,y)));
        self.realInput.up=coordsToIndex((x,y));
        self.realState = apply_input(self.realInput.clone(), self.realState.clone(), self.players.clone());
        if self.realState.turn == -1 && self.players==1{
            self.waitTicks = 2;
        }
        println!("P1-Stones:{},P2-Stones:{} \n",self.realState.p1_stones,self.realState.p2_stones,);
    }

}

fn apply_input(realInput: PlayerInput, mut realState: State, players: u8) -> State {
    if realInput.up == 24 || realInput.down == 24 || players==1 && realState.turn== -1{
        println!("Did nothing");
    } else {
        let up: (i8,i8,i8) = realState.board[realInput.up];
        let down: (i8,i8,i8) = realState.board[realInput.down];
        if realState.allowed==true && up == down{
            if State::remove_control(&realState, up)==Ok(true){
                println!("removing");
                match State::remove(&realState, up){
                    Ok(t) => realState=t,
                    Err(v)=> println!("{:?}", v),
                }

            }
        }
        else{
            if players==2 && realState.turn==-1{
                match realState.p2_mode{
                    Place(_)=>{
                        if up == down {
                            println!("placing");
                            match State::place(&realState, down){
                                Ok(t) => realState=t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    Move | Jump=>{
                        if State::move_control(&realState, down, up)==Ok(true){
                            println!("moving");
                            match State::mov(&realState, down, up){
                                Ok(t) => realState=t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    _=>{

                    }

                }
            }
            else{
                match realState.p1_mode{
                    Place(_)=>{
                        if up == down {
                            println!("placing");
                            match State::place(&realState, down){
                                Ok(t) => realState=t,
                                Err(v)=> println!("{:?}", v),
                            }
                        }
                    }
                    Move | Jump=>{
                        if State::move_control(&realState, down, up)==Ok(true){
                            println!("moving");
                            match State::mov(&realState, down, up){
                                Ok(t) => realState=t,
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
    return realState;
}

pub fn start() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("Muehle", "Rust-Atzen")
        .window_setup(conf::WindowSetup::default().title("Muehle"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0));
    
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
     event::run(ctx, event_loop, state)



}









fn mov_test(mut fd:State)->State{
    fd = fd.mov((4,2,1), (4,1,0)).unwrap();
    fd = fd.mov((4,6,-1), (6,6,0)).unwrap();
    
    fd = fd.mov((2,2,1), (4,2,0)).unwrap();
    fd = fd.remove((6,4,-1)).unwrap();
    return fd;
}


fn place_tst(mut fd:State)->State{
    fd = fd.place((4,2,0)).unwrap();
    fd = fd.place((7,4,0)).unwrap();
    
    fd = fd.place((1,1,0)).unwrap();
    fd = fd.place((7,1,0)).unwrap();
    
    fd = fd.place((4,3,0)).unwrap();
    fd = fd.place((7,7,0)).unwrap();
    fd = fd.remove((1,1,1)).unwrap();
    
    fd = fd.place((1,4,0)).unwrap();
    fd = fd.place((3,3,0)).unwrap();
    
    fd = fd.place((5,5,0)).unwrap();
    fd = fd.place((5,3,0)).unwrap();
    
    fd = fd.place((2,6,0)).unwrap();
    fd = fd.place((4,6,0)).unwrap();

    fd = fd.place((1,7,0)).unwrap();
    fd = fd.place((4,7,0)).unwrap();
    
    fd = fd.place((3,5,0)).unwrap();
    fd = fd.place((6,4,0)).unwrap();
    
    fd = fd.place(( 2,2,0)).unwrap();
    fd = fd.place((3,4,0)).unwrap();

    return fd;
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
