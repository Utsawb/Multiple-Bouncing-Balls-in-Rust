use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Ball {
    pub pos_x: f32,
    pub pos_y: f32,

    pub vel_x: f32,
    pub vel_y: f32,

    pub gravity: f32,
    pub radius: f32,
    pub color: Srgb<u8>,
}

impl Ball {
    pub fn new(pos_x: f32, pos_y: f32, vel_x: f32, vel_y: f32, gravity: f32, radius: f32, color: Srgb<u8>) -> Self {
        Ball { pos_x, pos_y, vel_x, vel_y, gravity, radius, color }
    }

    pub fn key_logic(&mut self, _key: Key) {
        if _key == Key::Up { self.gravity = self.gravity + 50.0}
        if _key == Key::Down { self.gravity = self.gravity - 50.0}
    
        if _key == Key::Right { self.vel_x = self.vel_x + 50.0}
        if _key == Key::Left { self.vel_x = self.vel_x - 50.0}
    }

    pub fn logic(&mut self, _app: &App, _update: Update) {
        let delta_time = _update.since_last.as_secs_f32();
        let win = _app.window_rect().pad(self.radius);

        if self.pos_x >= win.right() || self.pos_x <= win.left() { self.vel_x = -1.0 * self.vel_x; }
        if self.pos_y >= win.top() || self.pos_y <= win.bottom() { self.vel_y = -1.0 * self.vel_y }
        else { self.vel_y = self.vel_y + (self.gravity * delta_time) }

        self.pos_x = self.pos_x + (self.vel_x * delta_time);
        self.pos_y = self.pos_y + (self.vel_y * delta_time);
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos_x, self.pos_y)
            .radius(self.radius)
            .color(self.color);
    }
}