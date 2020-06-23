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


pub fn main() {
    let sys_time = SystemTime::now();
    let mut fd = State::new();
    
    fd = place_tst(fd);
    fd = mov_test(fd);
    let i = fd.spot_pot_muehle((4,2,1));
    println!("{}", i);
    println!("{:?}", fd);
    fd.spielstandbewertung();
    //println!("{:?}",fd.steineSchlagen());
    start();
    let tup = strt(6,fd);
    println!("Bewertung: {}",tup.0);
    println!("{:?}", tup.1);

    let difference = sys_time.elapsed();
    println!("{:?}", difference);
}


struct MainState {
    mouse_down: bool,
    
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            mouse_down: false,
            
        };
        Ok(s)
    }
}

pub fn fieldToCoordinates(fd:(i8,i8,i8))-> (i16,i16, i8) {
    match fd {
        (1, 1, x) => return (100,500,x),
        (1, 4, x) => return (100,300,x),
        (1, 7, x) => return (100,100,x),

        (2, 2, x) => return (175,425,x),
        (2, 4, x) => return (175,300,x),
        (2, 6, x) => return (175,175,x),

        (3, 3, x) => return (250,350,x),
        (3, 4, x) => return (250,300,x),
        (3, 5, x) => return (250,250,x),

        (4, 1, x) => return (300, 500,x),
        (4, 2, x) => return (300, 425,x),
        (4, 3, x) => return (300, 350,x),

        (4, 5, x) => return (300, 250,x),
        (4, 6, x) => return (300, 175,x),
        (4, 7, x) => return (300, 100,x),
        (5, 3, x) => return (350,350,x),
        (5, 4, x) => return (350,300,x),
        (5, 5, x) => return (350,250,x),

        (6, 2, x) => return (425,425,x),
        (6, 4, x) => return (425,300,x),
        (6, 6, x) => return (425,175,x),

        (7, 1, x) => return (500,500,x),
        (7, 4, x) => return (500,300,x),
        (7, 7, x) => return (500,100,x),
                
        _ => return (-1,-1,-1)
    }

    
}



impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {

    


        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
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

        for pos in main::fd {
            
        }

        /*let spielstein= graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), na::Point2::new(100.0, 100.0), 14.0, 0.1, graphics::BLACK).unwrap();
        graphics::draw(ctx, &spielstein, (na::Point2::new(0.0, 0.0),))?;


        fn drawspielstein( fd:(i8,i8,i8)) {
            if fd.2 == 1 {
                let spielstein= graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), na::Point2::new(100.0, 100.0), 14.0, 0.1, graphics::BLACK).unwrap();
            }

        }
        */
        graphics::present(ctx)?;
        Ok(())
    }
    


    

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

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
