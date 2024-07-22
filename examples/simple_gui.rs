use ggez::{event::{self, EventHandler}, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text}, ContextBuilder, GameResult};
use zui::button::{render_botton_with, Button};

struct Main {
    mesh: Mesh,
    test_button: Button,
    quit_button: Button
}

impl EventHandler for Main {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        println!("{}", ctx.time.fps());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        
        render_botton_with(&mut canvas, &self.test_button, &self.mesh, DrawParam::new());
        render_botton_with(&mut canvas, &self.quit_button, &self.mesh, DrawParam::new());
        render_botton_with(&mut canvas, &self.test_button, Text::new("test").set_scale(25.0), DrawParam::new());
        render_botton_with(&mut canvas, &self.quit_button, Text::new("quit").set_scale(25.0), DrawParam::new());
        if self.test_button.is_clicked(ctx) {
            println!("Test!");
        }
        if self.quit_button.is_clicked(ctx) {
            ctx.request_quit();
        }

        let _ = canvas.finish(ctx);
        Ok(())
    }
}

fn main() -> GameResult{
    let (ctx, event_loop) = ContextBuilder::new("simple_gui", "orange809").build()?;

    let test_button = Button::new(Rect::new(50.0, 50.0, 100.0, 50.0));
    let quit_button = Button::new(Rect::new(50.0, 130.0, 100.0, 50.0));
    let mesh = Mesh::new_rectangle(
        &ctx.gfx,
        DrawMode::fill(),
        Rect::new(0.0, 0.0, 100.0, 50.0),
        Color::GREEN
    )?;

    let main = Main {
        mesh,
        test_button,
        quit_button
    };

    event::run(ctx, event_loop, main)
}