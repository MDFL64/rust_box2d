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

//////////////////////

#[no_mangle]
extern "C" fn assert(x: i32) {
    if x==0 {
        panic!("assert failed");
    }
}

#[no_mangle]
unsafe extern "C" fn malloc(size: i32) -> i32 {
    let layout = std::alloc::Layout::from_size_align_unchecked(size as usize,0);
    return std::alloc::alloc_zeroed( layout ) as i32;
}

#[no_mangle]
unsafe extern "C" fn _Znwm(size: i32) -> i32 {
    malloc(size)
}

#[no_mangle]
unsafe extern "C" fn free(addr: i32) {
    let layout = std::alloc::Layout::from_size_align_unchecked(0,0);
    std::alloc::dealloc(addr as *mut u8, layout);
}

#[no_mangle]
unsafe extern "C" fn _ZdlPv(addr: i32) {
    free(addr);
}

#[no_mangle]
extern "C" fn __cxa_pure_virtual() {
    panic!("pure virtual call");
}


extern "C" {
    fn debug_info(x: i32) -> ();
}

struct B2Pair {
    id_a: i32,
    id_b: i32
}

type PairCompare = fn(*const B2Pair, *const B2Pair) -> bool;

#[no_mangle]
unsafe extern "C" fn box2d_sort_pairs(buffer: *mut B2Pair, count: i32, compare: PairCompare) {
    let array = std::slice::from_raw_parts_mut(buffer,count as usize);
    array.sort_unstable_by(|a,b| {
        let less = compare(a,b);
        if less {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    });
    /*for entry in array {
        //debug_info(-2);
        debug_info(entry.id_b + entry.id_a*1000);
        //debug_info(entry.id_b);
    }*/
}
