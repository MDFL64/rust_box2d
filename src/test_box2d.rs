use crate::user_data::NoUserData;
use crate::b2;

struct TestWorld {
    world: b2::World::<NoUserData>,
    bodies: Vec<b2::BodyHandle>
}

#[no_mangle]
extern "C" fn new_world(count: i32) -> i32 {
    let mut world: b2::World::<NoUserData> = b2::World::new(&b2::Vec2{x:0.0,y:-10.0});
    let mut bodies = Vec::new();


    // Container
    {
        let mut def = b2::BodyDef::new();
        def.body_type = b2::BodyType::Kinematic;
        def.angular_velocity = 0.25;
    
        let body_h = world.create_body(&def);

        {
            let mut shape = b2::PolygonShape::new();
            shape.set_as_oriented_box(20.0,1.0, &b2::Vec2{x:0.0,y:-21.0}, 0.0 );
            world.body_mut(body_h).create_fast_fixture(&shape,1.0);
        }

        {
            let mut shape = b2::PolygonShape::new();
            shape.set_as_oriented_box(20.0,1.0, &b2::Vec2{x:0.0,y:21.0}, 0.0 );
            world.body_mut(body_h).create_fast_fixture(&shape,1.0);
        }

        {
            let mut shape = b2::PolygonShape::new();
            shape.set_as_oriented_box(1.0,20.0, &b2::Vec2{x:21.0,y:0.0}, 0.0 );
            world.body_mut(body_h).create_fast_fixture(&shape,1.0);
        }

        {
            let mut shape = b2::PolygonShape::new();
            shape.set_as_oriented_box(1.0,20.0, &b2::Vec2{x:-21.0,y:0.0}, 0.0 );
            world.body_mut(body_h).create_fast_fixture(&shape,1.0);
        }

        bodies.push(body_h);
    }

    for i in 0..count {
        let mut def = b2::BodyDef::new();
        def.body_type = b2::BodyType::Dynamic;
    
        let body_h = world.create_body(&def);

        let mut shape = b2::PolygonShape::new();
        shape.set_as_box(0.5,0.5);

        world.body_mut(body_h).create_fast_fixture(&shape,1.0);
        world.body_mut(body_h).set_transform(&b2::Vec2{x:0.01 * (i as f32),y:0.0},0.0);

        bodies.push(body_h);
    }

    let wrapped = Box::new(TestWorld {
        world,
        bodies
    });
    return Box::into_raw(wrapped) as i32;
}

#[no_mangle]
unsafe extern "C" fn world_step(world: i32) {
    let ptr = world as *mut TestWorld;
    (*ptr).world.step(1.0/60.0,6,2);
}

#[no_mangle]
unsafe extern "C" fn get_body_coord(world: i32, n: i32, c: i32) -> f32 {
    let ptr = world as *mut TestWorld;
    let handle = (*ptr).bodies[n as usize];
    let body = (*ptr).world.body(handle);

    if c == 2 {
        return body.angle();
    }

    let coord = body.position().as_array()[c as usize];

    return coord;
}

#[no_mangle]
unsafe extern "C" fn get_profile(world: i32, n: i32) -> f32 {
    let ptr = world as *mut TestWorld;
    let profile = (*ptr).world.profile();
    let array = std::slice::from_raw_parts(profile as (*const b2::Profile) as (*const f32), 8);
    return array[n as usize];
}

/*
pub struct Profile {
    pub step: f32,          Whole Step
    pub collide: f32,           - Query BP for Contacts
    pub solve: f32,             - Solve
    pub solve_init: f32,            - Solve Init
    pub solve_velocity: f32,        - Solve Velocity
    pub solve_position: f32,        - Solve Position
    pub broad_phase: f32,           - (Sync Broadphase, Find New Contacts)
    pub solve_toi: f32,         - SolveTOI
}
*/
