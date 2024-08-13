extern crate gfx;
extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate wrapped2d;

pub mod camera;
pub use camera::Camera;
mod debug_draw;
use debug_draw::debug_draw;

use piston::event_loop::EventLoop;
use piston::input::{Key, MouseButton};
use piston::window::WindowSettings;
use piston_window::*;
use wrapped2d::b2;
use wrapped2d::user_data::UserDataTypes;

pub const UPDATES_PER_SECOND: u64 = 60;
pub const VELOCITY_ITERATIONS: i32 = 8;
pub const POSITION_ITERATIONS: i32 = 3;

pub trait Test {
    fn process_input(&mut self, _input: &Input, _data: &mut Data) {}

    fn step(&mut self, data: &mut Data, dt: f32) {
        step(data, dt);
    }
}

pub fn step(data: &mut Data, dt: f32) {
    data.world
        .step(dt, VELOCITY_ITERATIONS, POSITION_ITERATIONS);
}

impl Test for () {}

impl<F> Test for F
where
    F: FnMut(&Input, &mut Data),
{
    fn process_input(&mut self, i: &Input, d: &mut Data) {
        self(i, d);
    }
}

pub struct Data {
    pub world: b2::World,
    pub camera: Camera,
    pub draw_flags: b2::DrawFlags,
}

pub fn run<T>(mut test: T, mut data: Data, name: &str, mut width: u32, mut height: u32)
where
    T: Test,
{
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(format!("{} Test", name), [width, height])
        .maybe_graphics_api(Some(opengl))
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();
    window.set_max_fps(UPDATES_PER_SECOND);
    window.set_ups(UPDATES_PER_SECOND);

    let mut running = true;
    let mut mouse_position = b2::Vec2 { x: 0., y: 0. };

    let dummy = data.world.create_body(&b2::BodyDef::new());

    while let Some(evnt) = window.next() {
        /*let window_to_gl = |w, h, x, y| {
            let scale_x = 2. / w as f64;
            let scale_y = 2. / h as f64;
            (x * scale_x - 1., -1. * (y * scale_y - 1.))
        };
        let window_to_world = |w, h, camera: &Camera, x, y| {
            let (x, y) = window_to_gl(w, h, x, y);
            camera.gl_to_world(x, y, w, h)
        };*/

        match &evnt {
            Event::Input(inpt, _) => {
                match inpt {
                    Input::Button(button) => match (button.state, button.button) {
                        (ButtonState::Press, Button::Keyboard(Key::Return)) => running = !running,
                        _ => {}
                    },
                    Input::Resize(resize) => {
                        width = resize.draw_size[0];
                        height = resize.draw_size[1];
                    }
                    _ => {}
                }
                test.process_input(&inpt, &mut data);
            }
            Event::Loop(Loop::Update(UpdateArgs { dt })) => {
                if running {
                    test.step(&mut data, *dt as f32);
                }
            }
            Event::Loop(Loop::Render(_)) => {
                let transform = data.camera.transform_world_to_gl(width, height);

                window.draw_2d(&evnt, |c, g, _dev| {
                    graphics::clear([0.1, 0.1, 0.1, 1.0], g);
                    debug_draw(&mut data.world, data.draw_flags, transform, c, g);
                });
            }
            _ => {}
        }
    }
}
