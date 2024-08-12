extern crate piston_window;
extern crate testbed;
extern crate wrapped2d;

use std::time::{Duration, Instant};

use piston_window::*;
use wrapped2d::b2;
use wrapped2d::dynamics::world::BodyHandle;
use wrapped2d::user_data::NoUserData;

type World = b2::World<NoUserData>;

const COUNT: usize = 2000;
const SIZE: f32 = 30.0;
const TUMBLE_RATE: f32 = 0.5;
const TOTAL_STEPS: u32 = 1000;

fn main() {
    let mut world = World::new(&b2::Vec2 { x: 0., y: -10. });

    let b_def = b2::BodyDef {
        body_type: b2::BodyType::Kinematic,
        position: b2::Vec2 { x: -0., y: 0. },
        ..b2::BodyDef::new()
    };

    let tumbler_handle = world.create_body(&b_def);
    {
        let mut tumbler = world.body_mut(tumbler_handle);

        tumbler.set_angular_velocity(TUMBLE_RATE);

        let sz = SIZE + 2.0;

        let shape = b2::PolygonShape::new_oriented_box(sz, 1.0, &b2::Vec2 { x: 0.0, y: -sz }, 0.0);
        tumbler.create_fast_fixture(&shape, 0.);

        let shape = b2::PolygonShape::new_oriented_box(1.0, sz, &b2::Vec2 { x: -sz, y: 0.0 }, 0.0);
        tumbler.create_fast_fixture(&shape, 0.);

        let shape = b2::PolygonShape::new_oriented_box(sz, 1.0, &b2::Vec2 { x: 0.0, y: sz }, 0.0);
        tumbler.create_fast_fixture(&shape, 0.);

        let shape = b2::PolygonShape::new_oriented_box(1.0, sz, &b2::Vec2 { x: sz, y: 0.0 }, 0.0);
        tumbler.create_fast_fixture(&shape, 0.);
    }

    let cube_shape = b2::PolygonShape::new_box(0.5, 0.5);
    let mut circle_shape = b2::CircleShape::new();
    circle_shape.set_radius(1.);

    // make pyramid
    {
        let mut f_def = b2::FixtureDef::new();
        f_def.density = 1.0;

        let mut b_def = b2::BodyDef::new();
        b_def.position = b2::Vec2 { x: -SIZE, y: -SIZE };
        b_def.body_type = b2::BodyType::Dynamic;

        for i in 0..COUNT {
            b_def.position.x += 1.0;
            if b_def.position.x > SIZE {
                b_def.position.x = -SIZE;
                b_def.position.y += 1.0;
            }
            let handle = world.create_body(&b_def);
            world
                .body_mut(handle)
                .create_fixture(&cube_shape, &mut f_def);
        }
    }

    let mut b_def = b2::BodyDef {
        body_type: b2::BodyType::Dynamic,
        position: b2::Vec2 { x: -20., y: 20. },
        ..b2::BodyDef::new()
    };

    let mut f_def = b2::FixtureDef {
        density: 1.,
        restitution: 0.2,
        friction: 0.3,
        ..b2::FixtureDef::new()
    };

    let data = testbed::Data {
        world,
        camera: testbed::Camera {
            position: [0.0, 0.0],
            scale: 5.0,
        },
        draw_flags: b2::DrawFlags::DRAW_SHAPE
            | b2::DrawFlags::DRAW_JOINT
            | b2::DrawFlags::DRAW_PAIR
            | b2::DrawFlags::DRAW_CENTER_OF_MASS,
    };

    testbed::run(Tumbler{
        steps: 0,
        total_time: Duration::default(),
    }, data, "Tumbler", 800, 800);
}

struct Tumbler {
    steps: u32,
    total_time: Duration
}

impl testbed::Test<NoUserData> for Tumbler {
    fn step(&mut self, data: &mut testbed::Data<NoUserData>, dt: f32) {
        let t = Instant::now();
        testbed::step(data, dt);
        self.total_time += t.elapsed();
        self.steps += 1;
        if self.steps > TOTAL_STEPS {
            println!("{:?}", self.total_time);
            std::process::exit(0);
        }
    }
}
