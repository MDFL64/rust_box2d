use chipmunk_port::space::{cpSpace,cpSpaceAddShape};
use chipmunk_port::{types,body};
use chipmunk_port::poly_shape;
use chipmunk_port::util;
use chipmunk_port::vector::cpVect;


struct TestWorld {
    space: *mut cpSpace,
    bodies: Vec<*mut body::cpBody>
}

#[no_mangle]
unsafe extern "C" fn new_world(box_count: i32) -> i32 {

    let space = Box::into_raw(cpSpace::new());
    (*space).set_gravity(cpVect{x:0.0,y:-10.0});

    let mut bodies = Vec::new();

    // GROUND
    {
        let body = (*space).add_body(chipmunk_port::body::CP_BODY_TYPE_KINEMATIC);

        // bottom
        let shape = poly_shape::cpBoxShapeNew2(body, types::cpBB{t:-20.0,b:-21.0,l:-20.0,r:20.0}, 0.0);
        cpSpaceAddShape(space,shape);

        // top
        let shape = poly_shape::cpBoxShapeNew2(body, types::cpBB{t:21.0,b:20.0,l:-20.0,r:20.0}, 0.0);
        cpSpaceAddShape(space,shape);

        // left
        let shape = poly_shape::cpBoxShapeNew2(body, types::cpBB{t:20.0,b:-20.0,l:-21.0,r:-20.0}, 0.0);
        cpSpaceAddShape(space,shape);

        // right
        let shape = poly_shape::cpBoxShapeNew2(body, types::cpBB{t:20.0,b:-20.0,l:20.0,r:21.0}, 0.0);
        cpSpaceAddShape(space,shape);

        body::cpBodySetAngularVelocity(body, 0.25);

        bodies.push(body);
    }

    // BOXES
    for i in 0..box_count {
        //let body = body::cpBodyNew(1.0, util::cpMomentForBox(1.0, 1.0, 1.0));
        //space::cpSpaceAddBody(space, body);
        let body = (*space).add_body(chipmunk_port::body::CP_BODY_TYPE_DYNAMIC);

        body::cpBodySetPosition(body,cpVect{x: (i as f64) * 10.0 / (box_count as f64), y: 0.0});

        //let shape = shape::cpCircleShapeNew(body,1.0,types::cpVect{x:0.0,y:0.0});
        let shape = poly_shape::cpBoxShapeNew(body, 1.0, 1.0, 0.0);
        chipmunk_port::shape::cpShapeSetMass(shape, 1.0 );

        cpSpaceAddShape(space,shape);

        bodies.push(body);
    }

    let wrapped = Box::new(TestWorld {
        space,
        bodies
    });
    return Box::into_raw(wrapped) as i32;
}

#[no_mangle]
unsafe extern "C" fn world_step(world: i32) {
    let ptr = world as *mut TestWorld;
    chipmunk_port::space_step::cpSpaceStep((*ptr).space, 1.0/60.0);
}

#[no_mangle]
unsafe extern "C" fn get_body_coord(world: i32, n: i32, c: i32) -> f32 {
    let ptr = world as *mut TestWorld;
    let handle = (*ptr).bodies[n as usize];
    if c == 0 {
        return chipmunk_port::body::cpBodyGetPosition(handle).x as f32;
    } else if c == 1 {
        return chipmunk_port::body::cpBodyGetPosition(handle).y as f32;
    } else {
        return chipmunk_port::body::cpBodyGetAngle(handle) as f32;
    }
    /*let ptr = world as *mut TestWorld;
    let handle = (*ptr).bodies[n as usize];
    let body = (*ptr).world.body(handle);

    if c == 2 {
        return body.angle();
    }

    let coord = body.position().as_array()[c as usize];

    return coord;*/
    0.0
}

/*#[wasm_bindgen]
impl ChipmunkTest {
    pub fn step(&self, dt: f64) {
        unsafe {
            space_step::cpSpaceStep(self.space,dt);
        }
    }

    pub fn getBodyCount(&self) -> usize {
        return self.bodies.len();
    }

    pub fn getBodyX(&self, i: usize) -> f64 {
        unsafe {
            let pos = body::cpBodyGetPosition(self.bodies[i]);
            return pos.x;
        }
    }

    pub fn getBodyY(&self, i: usize) -> f64 {
        unsafe {
            let pos = body::cpBodyGetPosition(self.bodies[i]);
            return pos.y;
        }
    }

    pub fn getBodyA(&self, i: usize) -> f64 {
        unsafe {
            let angle = body::cpBodyGetAngle(self.bodies[i]);
            return angle;
        }
    }
}*/
