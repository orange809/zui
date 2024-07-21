use ggez::{event::MouseButton, glam::Vec2, graphics::{Canvas, DrawParam, Drawable, Rect}, Context};

pub struct Button {
    rect: Rect,
    visible: bool,
}

impl Button {
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}

impl Button {
    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl Button {
    pub fn new(rect: Rect) -> Self {
        Button {
            rect: rect,
            visible: true,
        }
    }

    pub fn is_clicked(&self, ctx: &Context) -> bool {
        if !self.visible {
            return false;
        }

        let x = ctx.mouse.position().x;
        let y = ctx.mouse.position().y;

        let is_mouse_on_click_button_x = x >= (self.rect.x) && x <= (self.rect.x + self.rect.w);
        let is_mouse_on_click_button_y = y >= (self.rect.y) && y <= (self.rect.y + self.rect.h);

        is_mouse_on_click_button_y & is_mouse_on_click_button_x & ctx.mouse.button_just_pressed(MouseButton::Left)
    }
}

pub fn render_botton_with(canvas: &mut Canvas, button: &Button, with: &impl Drawable, draw_param: DrawParam) {
    canvas.draw(with, draw_param.dest(Vec2::new(button.rect.x, button.rect.y)));
}
