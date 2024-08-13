use box2d3::debug_draw::DebugDraw;
use box2d3::math::Rot;
use collision::AABB;
use common::math::Vec2;
use common::{Draw, DrawFlags};
use dynamics::body::{BodyDef, MetaBody};
use dynamics::contacts::Contact;
use dynamics::Profile;
use handle::*;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use user_data::UserDataTypes;
use wrap::*;

use crate::b2::Color;

#[derive(Copy, Clone)]
pub struct BodyHandle(box2d3::Body);

#[derive(Copy, Clone)]
pub struct FixtureHandle(box2d3::Shape);

pub type JointHandle = ();

pub struct World {
    handle: box2d3::World,
    ptr: *mut ffi::World,
}

impl World {
    pub fn new(gravity: &Vec2) -> Self {
        let mut def = box2d3::WorldDef::default();
        def.gravity = *gravity;
        let handle = box2d3::World::new(&def);
        World {
            handle,
            ptr: std::ptr::null_mut()
        }
    }

    pub fn create_body(&mut self, def: &BodyDef) -> BodyHandle
    {
        self.create_body_with(def)
    }

    pub fn create_body_with(&mut self, def: &BodyDef) -> BodyHandle {
        let mut internal_def = box2d3::BodyDef::default();

        internal_def.kind = def.body_type;
        internal_def.position = def.position;
        internal_def.rotation = Rot::from_angle(def.angle);
        internal_def.linear_velocity = def.linear_velocity;
        internal_def.angular_velocity = def.angular_velocity;
        internal_def.linear_damping = def.linear_damping;
        internal_def.angular_damping = def.angular_damping;
        internal_def.enable_sleep = def.allow_sleep;
        internal_def.is_awake = def.awake;
        internal_def.fixed_rotation = def.fixed_rotation;
        internal_def.is_bullet = def.bullet;
        internal_def.is_enabled = def.active;
        internal_def.gravity_scale = def.gravity_scale;

        let body = self.handle.create_body(&internal_def);

        BodyHandle(body)
    }

    pub fn body(&self, handle: BodyHandle) -> MetaBody {
        MetaBody::new(handle.0)
    }

    pub fn body_mut(&self, handle: BodyHandle) -> MetaBody {
        MetaBody::new(handle.0)
    }

    /*pub fn try_body(&self, handle: BodyHandle) -> Option<Ref<MetaBody<U>>> {
        panic!("-");

        //self.bodies.get(handle)
    }

    pub fn try_body_mut(&self, handle: BodyHandle) -> Option<RefMut<MetaBody<U>>> {
        panic!("-");

        //self.bodies.get_mut(handle)
    }*/

    pub fn destroy_body(&mut self, handle: BodyHandle) {
        todo!("destroy body");
        /*let mut body = self.bodies.remove(handle);

        World::remove_body_joint_handles(&mut body, &mut self.joints);
        unsafe {
            ffi::World_destroy_body(self.ptr, body.ptr);
        }*/
    }

    pub fn step(&mut self, time_step: f32, velocity_iterations: i32, position_iterations: i32) {
        self.handle.step(time_step, velocity_iterations as u32);
    }

    pub fn clear_forces(&mut self) {
        unsafe { ffi::World_clear_forces(self.ptr) }
    }

    pub fn draw_debug_data<D: Draw>(&mut self, draw: &mut D, flags: DrawFlags) {
        /*unsafe {
            let ptr = self.draw_link.use_with(draw, flags);
            ffi::World_set_debug_draw(self.ptr, ptr);

            ffi::World_draw_debug_data(self.ptr);

            ffi::World_set_debug_draw(self.ptr, ptr::null_mut());
        }*/
        //println!("TODO DRAW");

        let draw_opts = DebugDraw::<D> {
            draw_polygon: |_, _, _, _| {
                println!("draw_polygon")
            },
            draw_solid_polygon: |xform, verts, vert_count, radius, color, draw| {
                let vert_count = vert_count as usize;
                let mut vert_buffer = [Vec2{x: 0.0, y: 0.0};8];
                assert!(vert_count < vert_buffer.len());

                unsafe {
                    for i in 0..vert_count {
                        let v = verts.add(i).read();
                        vert_buffer[i] = &xform * v;
                    }

                    let color = color.to_floats();

                    (*draw).draw_solid_polygon(&vert_buffer[0..vert_count], &Color{
                        a: 1.0,
                        r: color[0],
                        g: color[1],
                        b: color[2]
                    });
                }
            },
            draw_circle: |_, _, _, _| {
                println!("draw_circle")
            },
            draw_solid_circle: |_, _, _, _| {
                println!("draw_solid_circle")
            },
            draw_capsule: |_, _, _, _, _| {
                println!("draw_capsule")
            },
            draw_solid_capsule: |_, _, _, _, _| {
                println!("draw_solid_capsule")
            },
            draw_segment: |_, _, _, _| {
                println!("draw_segment")
            },
            draw_transform: |_, _| {
                println!("draw_transform")
            },
            draw_point: |_, _, _, _| {
                println!("draw_point")
            },
            draw_string: |_, _, _| {
                println!("draw_string")
            },
            drawing_bounds: box2d3::math::AABB {
                lower_bound: Vec2{x: 0.0, y: 0.0},
                upper_bound: Vec2{x: 0.0, y: 0.0},
            },
            use_drawing_bounds: false,
            draw_shapes: true,
            draw_joints: false,
            draw_joint_extras: false,
            draw_aabbs: false,
            draw_mass: false,
            draw_contacts: false,
            draw_graph_colors: false,
            draw_contact_normals: false,
            draw_contact_impulses: false,
            draw_friction_impulses: false,
            context: draw,
        };

        self.handle.debug_draw(&draw_opts);

    }

    pub fn contacts_mut(&mut self) -> ContactIterMut {
        ContactIterMut {
            ptr: unsafe { ffi::World_get_contact_list(self.ptr) },
            phantom: PhantomData,
        }
    }

    pub fn contacts(&self) -> ContactIter {
        ContactIter {
            ptr: unsafe { ffi::World_get_contact_list_const(self.ptr) },
            phantom: PhantomData,
        }
    }

    pub fn set_sleeping_allowed(&mut self, flag: bool) {
        unsafe { ffi::World_set_allow_sleeping(self.ptr, flag) }
    }

    pub fn is_sleeping_allowed(&self) -> bool {
        unsafe { ffi::World_get_allow_sleeping(self.ptr) }
    }

    pub fn set_warm_starting(&mut self, flag: bool) {
        unsafe { ffi::World_set_warm_starting(self.ptr, flag) }
    }

    pub fn is_warm_starting(&self) -> bool {
        unsafe { ffi::World_get_warm_starting(self.ptr) }
    }

    pub fn set_continuous_physics(&mut self, flag: bool) {
        unsafe { ffi::World_set_continuous_physics(self.ptr, flag) }
    }

    pub fn is_continuous_physics(&self) -> bool {
        unsafe { ffi::World_get_continuous_physics(self.ptr) }
    }

    pub fn set_sub_stepping(&mut self, flag: bool) {
        unsafe { ffi::World_set_sub_stepping(self.ptr, flag) }
    }

    pub fn is_sub_stepping(&self) -> bool {
        unsafe { ffi::World_get_sub_stepping(self.ptr) }
    }

    pub fn proxy_count(&self) -> i32 {
        unsafe { ffi::World_get_proxy_count(self.ptr) }
    }

    pub fn body_count(&self) -> i32 {
        unsafe { ffi::World_get_body_count(self.ptr) }
    }

    pub fn joint_count(&self) -> i32 {
        unsafe { ffi::World_get_joint_count(self.ptr) }
    }

    pub fn contact_count(&self) -> i32 {
        unsafe { ffi::World_get_contact_count(self.ptr) }
    }

    pub fn tree_height(&self) -> i32 {
        unsafe { ffi::World_get_tree_height(self.ptr) }
    }

    pub fn tree_balance(&self) -> i32 {
        unsafe { ffi::World_get_tree_balance(self.ptr) }
    }

    pub fn tree_quality(&self) -> f32 {
        unsafe { ffi::World_get_tree_quality(self.ptr) }
    }

    pub fn set_gravity(&mut self, gravity: &Vec2) {
        unsafe { ffi::World_set_gravity(self.ptr, gravity) }
    }

    pub fn gravity(&self) -> Vec2 {
        unsafe { ffi::World_get_gravity(self.ptr) }
    }

    pub fn is_locked(&self) -> bool {
        unsafe { ffi::World_is_locked(self.ptr) }
    }

    pub fn set_auto_clearing_forces(&mut self, flag: bool) {
        unsafe { ffi::World_set_auto_clear_forces(self.ptr, flag) }
    }

    pub fn is_auto_clearing_forces(&self) -> bool {
        unsafe { ffi::World_get_auto_clear_forces(self.ptr) }
    }

    pub fn shift_origin(&mut self, origin: &Vec2) {
        unsafe { ffi::World_shift_origin(self.ptr, origin) }
    }

    pub fn profile<'a>(&'a self) -> &'a Profile {
        unsafe {
            &*ffi::World_get_profile(self.ptr) // Comes from a C++ &
        }
    }

    pub fn dump(&mut self) {
        unsafe { ffi::World_dump(self.ptr) }
    }
}

pub struct ContactIterMut<'a> {
    ptr: *mut ffi::Contact,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for ContactIterMut<'a> {
    type Item = WrappedRefMut<'a, Contact>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe {
                let next = ffi::Contact_get_next(self.ptr);
                Some(WrappedRefMut::new(Contact::from_ffi(mem::replace(
                    &mut self.ptr,
                    next,
                ))))
            }
        }
    }
}

pub struct ContactIter<'a> {
    ptr: *const ffi::Contact,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for ContactIter<'a> {
    type Item = WrappedRef<'a, Contact>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe {
                let next = ffi::Contact_get_next_const(self.ptr);
                Some(WrappedRef::new(Contact::from_ffi(
                    mem::replace(&mut self.ptr, next) as *mut ffi::Contact,
                )))
            }
        }
    }
}

#[doc(hidden)]
pub mod ffi {
    use collision::AABB;
    pub use common::ffi::Draw;
    use common::math::Vec2;
    pub use dynamics::body::ffi::Body;
    use dynamics::body::BodyDef;
    pub use dynamics::contacts::ffi::{Contact, Contact_get_next, Contact_get_next_const};
    use dynamics::Profile;

    pub enum World {}

    pub fn World_drop(slf: *mut World) {
        todo!()
    }
    pub fn World_set_debug_draw(slf: *mut World, dd: *mut Draw) {
        todo!()
    }
    pub fn World_create_body(slf: *mut World, def: *const BodyDef) -> *mut Body {
        todo!()
    }
    pub fn World_destroy_body(slf: *mut World, body: *mut Body) {
        todo!()
    }
    pub fn World_clear_forces(slf: *mut World) {
        todo!()
    }
    pub fn World_draw_debug_data(slf: *mut World) {
        todo!()
    }
    // pub fn World_get_body_list(slf: *mut World) -> *mut Body {todo!()}
    // pub fn World_get_body_list_const(slf: *const World) -> *const Body {todo!()}
    // pub fn World_get_joint_list(slf: *mut World) -> *mut Joint {todo!()}
    // pub fn World_get_joint_list_const(slf: *const World) -> *const Joint {todo!()}
    pub fn World_get_contact_list(slf: *mut World) -> *mut Contact {
        todo!()
    }
    pub fn World_get_contact_list_const(slf: *const World) -> *const Contact {
        todo!()
    }
    pub fn World_set_allow_sleeping(slf: *mut World, flag: bool) {
        todo!()
    }
    pub fn World_get_allow_sleeping(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_set_warm_starting(slf: *mut World, flag: bool) {
        todo!()
    }
    pub fn World_get_warm_starting(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_set_continuous_physics(slf: *mut World, flag: bool) {
        todo!()
    }
    pub fn World_get_continuous_physics(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_set_sub_stepping(slf: *mut World, flag: bool) {
        todo!()
    }
    pub fn World_get_sub_stepping(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_get_proxy_count(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_body_count(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_joint_count(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_contact_count(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_tree_height(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_tree_balance(slf: *const World) -> i32 {
        todo!()
    }
    pub fn World_get_tree_quality(slf: *const World) -> f32 {
        todo!()
    }
    pub fn World_set_gravity(slf: *mut World, gravity: *const Vec2) {
        todo!()
    }
    pub fn World_get_gravity(slf: *const World) -> Vec2 {
        todo!()
    }
    pub fn World_is_locked(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_set_auto_clear_forces(slf: *mut World, flag: bool) {
        todo!()
    }
    pub fn World_get_auto_clear_forces(slf: *const World) -> bool {
        todo!()
    }
    pub fn World_shift_origin(slf: *mut World, origin: *const Vec2) {
        todo!()
    }
    // pub fn World_get_contact_manager(slf: *const World) -> *const ContactManager {todo!()}
    pub fn World_get_profile(slf: *const World) -> *const Profile {
        todo!()
    }
    pub fn World_dump(slf: *mut World) {
        todo!()
    }
}
