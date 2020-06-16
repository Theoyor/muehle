use ggez;
use ggez::{Context};
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::conf;
use super::base::base::State;


struct MainState {
    mouse_down: bool,
    pos_x: f32,
    
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            mouse_down: false,
            pos_x: 0.0
        };
        Ok(s)
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