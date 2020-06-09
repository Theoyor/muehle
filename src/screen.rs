use ggez;
use ggez::{Context};
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::conf;


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
        //self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [51.0, 118.0, 29.0, 1.0].into());
    

        //Äußerer Ring:
        let rectsize1 = graphics::Rect::new(100.0, 100.0, 400.0, 400.0);
        let rect1 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(3.0), 
            rectsize1, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect1, (na::Point2::new(0.0, 0.0),))?;

        //mittlerer Ring:
        let rectsize2 = graphics::Rect::new(175.0, 175.0, 250.0, 250.0);
        let rect2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(3.0), 
            rectsize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect2, (na::Point2::new(0.0, 0.0),))?;


        //innerer Ring:
        let rectsize2 = graphics::Rect::new(250.0, 250.0, 100.0, 100.0);
        let rect2 = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(3.0), 
            rectsize2, 
            graphics::BLACK
        )?;
        graphics::draw(ctx, &rect2, (na::Point2::new(0.0, 0.0),))?;
        
        //Verbindungslinien:
        
        //let line1= graphics::Mesh::new_line(ctx, &[na::Point2::new(10.0, 10.0), na::Point2::new(10.0, 20.0)], 3.0, graphics::BLACK);
        
        //graphics::draw(ctx, &line1, (na::Point2::new(0.0,0.0),))?;
        

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