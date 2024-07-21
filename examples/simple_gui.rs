use ggez::{event::{self, EventHandler}, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text}, ContextBuilder, GameResult};
use zui::button::{render_botton_with, Button};

struct Main {
    mesh: Mesh
}

impl EventHandler for Main {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let test_button = Button::new(Rect::new(50.0, 50.0, 200.0, 100.0));
        let quit_button = Button::new(Rect::new(50.0, 100.0, 200.0, 100.0));
        
        render_botton_with(&mut canvas, &test_button, &self.mesh, DrawParam::new());
        render_botton_with(&mut canvas, &quit_button, &self.mesh, DrawParam::new());
        render_botton_with(&mut canvas, &test_button, Text::new("test").set_scale(100.0), DrawParam::new());
        render_botton_with(&mut canvas, &quit_button, Text::new("quit").set_scale(100.0), DrawParam::new());
        if test_button.is_clicked(ctx) {
            println!("Test!");
        }
        if quit_button.is_clicked(ctx) {
            ctx.request_quit();
        }

        let _ = canvas.finish(ctx);
        Ok(())
    }
}

fn main() -> GameResult{
    let (ctx, event_loop) = ContextBuilder::new("simple_gui", "orange809").build()?;

    let mesh = Mesh::new_rectangle(
        &ctx.gfx,
        DrawMode::fill(),
        Rect::new(0.0, 0.0, 200.0, 100.0),
        Color::GREEN
    )?;

    let main = Main {
        mesh
    };

    event::run(ctx, event_loop, main)
}