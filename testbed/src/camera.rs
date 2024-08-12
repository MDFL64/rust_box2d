use wrapped2d::b2;

pub struct Camera {
    pub position: [f64; 2],
    pub scale: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0., 0.],
            scale: 1.0,
        }
    }

    pub fn transform_world_to_gl(&self, window_w: u32, window_h: u32) -> [[f64; 3]; 2] {
        let translate_x = -self.position[0];
        let translate_y = -self.position[1];
        let scale_x = 2. / window_w as f64 * self.scale;
        let scale_y = 2. / window_h as f64 * self.scale;
        [
            [scale_x, 0., translate_x * scale_x],
            [0., scale_y, translate_y * scale_y],
        ]
    }

    pub fn gl_to_world(&self, x: f64, y: f64, window_w: u32, window_h: u32) -> b2::Vec2 {
        let scale_x = 0.5 * window_w as f64 / self.scale;
        let scale_y = 0.5 * window_h as f64 / self.scale;
        let translate_x = self.position[0];
        let translate_y = self.position[1];
        b2::Vec2 {
            x: (x * scale_x + translate_x) as f32,
            y: (y * scale_y + translate_y) as f32,
        }
    }
}
