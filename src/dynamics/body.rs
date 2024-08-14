use box2d3::body::BodyKind;
use box2d3::ShapeDef;
use collision::shapes::{MassData, Shape};
use common::math::{Transform, Vec2};
use dynamics::contacts::{Contact, ContactEdge};
use dynamics::fixture::{Fixture, FixtureDef, MetaFixture};
use dynamics::world::{BodyHandle, JointHandle};
use handle::*;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use user_data::{InternalUserData, RawUserData, RawUserDataMut, UserData, UserDataTypes};
use wrap::*;

use crate::b2::UnknownShape;

pub type BodyType = box2d3::body::BodyKind;

#[repr(C)]
#[derive(Clone)]
pub struct BodyDef {
    pub body_type: BodyType,
    pub position: Vec2,
    pub angle: f32,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub allow_sleep: bool,
    pub awake: bool,
    pub fixed_rotation: bool,
    pub bullet: bool,
    pub active: bool,
    #[doc(hidden)]
    pub user_data: ffi::Any,
    pub gravity_scale: f32,
}

impl BodyDef {
    pub fn new() -> BodyDef {
        BodyDef {
            body_type: BodyType::Static,
            position: Vec2 { x: 0., y: 0. },
            angle: 0.,
            linear_velocity: Vec2 { x: 0., y: 0. },
            angular_velocity: 0.,
            linear_damping: 0.,
            angular_damping: 0.,
            allow_sleep: true,
            awake: true,
            fixed_rotation: false,
            bullet: false,
            active: true,
            user_data: ptr::null_mut(),
            gravity_scale: 1.,
        }
    }
}

pub struct FixtureHandle(box2d3::Shape);

pub struct MetaBody {
    body: box2d3::Body,
}

impl MetaBody {
    pub fn new(body: box2d3::Body) -> Self {
        Self {
            body
        }
    }

    pub fn create_fixture(&mut self, shape: &dyn Shape, def: &mut FixtureDef) -> FixtureHandle
    {
        self.create_fixture_with(shape, def)
    }

    pub fn create_fixture_with(
        &mut self,
        shape: &dyn Shape,
        def: &mut FixtureDef,
    ) -> FixtureHandle {

        let mut shape_def = ShapeDef::default();
        shape_def.friction = def.friction;
        shape_def.restitution = def.restitution;
        shape_def.density = def.density;

        let shape = match shape.to_enum() {
            UnknownShape::Polygon(polygon) => {
                self.body.create_shape_polygon(&shape_def, &polygon.inner)
            }
            _ => panic!("?")
        };

        FixtureHandle(shape)
    }

    pub fn create_fast_fixture(&mut self, shape: &dyn Shape, density: f32) -> FixtureHandle
    {
        self.create_fast_fixture_with(shape, density)
    }

    pub fn create_fast_fixture_with(
        &mut self,
        shape: &dyn Shape,
        density: f32,
    ) -> FixtureHandle {
        // the defaults here aren't consistent but who cares at this stage?
        let mut def = FixtureDef::new();
        def.density = density;

        self.create_fixture_with(shape, &mut def)
    }

    pub fn set_angular_velocity(&mut self, v: f32) {
        self.body.set_angular_velocity(v);
    }
}
/*
impl Body {
    pub fn handle(&self) -> BodyHandle {
        panic!()
        //unsafe { self.ptr().handle() }
    }

    pub fn transform<'a>(&'a self) -> &'a Transform {
        unsafe {
            &*ffi::Body_get_transform(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn position<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::Body_get_position(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn angle(&self) -> f32 {
        unsafe { ffi::Body_get_angle(self.ptr()) }
    }

    pub fn world_center<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::Body_get_world_center(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn local_center<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::Body_get_local_center(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn linear_velocity<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::Body_get_linear_velocity(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn angular_velocity(&self) -> f32 {
        unsafe { ffi::Body_get_angular_velocity(self.ptr()) }
    }

    pub fn mass(&self) -> f32 {
        unsafe { ffi::Body_get_mass(self.ptr()) }
    }

    pub fn inertia(&self) -> f32 {
        unsafe { ffi::Body_get_inertia(self.ptr()) }
    }

    pub fn mass_data(&self) -> MassData {
        unsafe {
            let mut data = mem::zeroed();
            ffi::Body_get_mass_data(self.ptr(), &mut data);
            data
        }
    }

    pub fn world_point(&self, local: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_world_point(self.ptr(), local) }
    }

    pub fn world_vector(&self, local: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_world_vector(self.ptr(), local) }
    }

    pub fn local_point(&self, world: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_local_point(self.ptr(), world) }
    }

    pub fn local_vector(&self, world: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_local_vector(self.ptr(), world) }
    }

    pub fn linear_velocity_from_world_point(&self, world: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_linear_velocity_from_world_point(self.ptr(), world) }
    }

    pub fn linear_velocity_from_local_point(&self, local: &Vec2) -> Vec2 {
        unsafe { ffi::Body_get_linear_velocity_from_local_point(self.ptr(), local) }
    }

    pub fn linear_damping(&self) -> f32 {
        unsafe { ffi::Body_get_linear_damping(self.ptr()) }
    }

    pub fn angular_damping(&self) -> f32 {
        unsafe { ffi::Body_get_angular_damping(self.ptr()) }
    }

    pub fn gravity_scale(&self) -> f32 {
        unsafe { ffi::Body_get_gravity_scale(self.ptr()) }
    }

    pub fn body_type(&self) -> BodyType {
        unsafe { ffi::Body_get_type(self.ptr()) }
    }

    pub fn is_bullet(&self) -> bool {
        unsafe { ffi::Body_is_bullet(self.ptr()) }
    }

    pub fn is_sleeping_allowed(&self) -> bool {
        unsafe { ffi::Body_is_sleeping_allowed(self.ptr()) }
    }

    pub fn is_awake(&self) -> bool {
        unsafe { ffi::Body_is_awake(self.ptr()) }
    }

    pub fn is_active(&self) -> bool {
        unsafe { ffi::Body_is_active(self.ptr()) }
    }

    pub fn is_rotation_fixed(&self) -> bool {
        unsafe { ffi::Body_is_fixed_rotation(self.ptr()) }
    }

    pub fn joints(&self) -> JointIter {
        JointIter {
            edge: unsafe { ffi::Body_get_joint_list_const(self.ptr()) },
            phantom: PhantomData,
        }
    }

    pub fn set_transform(&mut self, pos: &Vec2, angle: f32) {
        unsafe { ffi::Body_set_transform(self.mut_ptr(), pos, angle) }
    }

    pub fn set_linear_velocity(&mut self, v: &Vec2) {
        unsafe { ffi::Body_set_linear_velocity(self.mut_ptr(), v) }
    }

    pub fn apply_force(&mut self, force: &Vec2, point: &Vec2, wake: bool) {
        unsafe { ffi::Body_apply_force(self.mut_ptr(), force, point, wake) }
    }

    pub fn apply_force_to_center(&mut self, force: &Vec2, wake: bool) {
        unsafe { ffi::Body_apply_force_to_center(self.mut_ptr(), force, wake) }
    }

    pub fn apply_torque(&mut self, torque: f32, wake: bool) {
        unsafe { ffi::Body_apply_torque(self.mut_ptr(), torque, wake) }
    }

    pub fn apply_linear_impulse(&mut self, impulse: &Vec2, point: &Vec2, wake: bool) {
        unsafe { ffi::Body_apply_linear_impulse(self.mut_ptr(), impulse, point, wake) }
    }

    pub fn apply_angular_impulse(&mut self, impulse: f32, wake: bool) {
        unsafe { ffi::Body_apply_angular_impulse(self.mut_ptr(), impulse, wake) }
    }

    pub fn set_mass_data(&mut self, data: &MassData) {
        unsafe { ffi::Body_set_mass_data(self.mut_ptr(), data) }
    }

    pub fn reset_mass_data(&mut self) {
        unsafe { ffi::Body_reset_mass_data(self.mut_ptr()) }
    }

    pub fn set_linear_damping(&mut self, damping: f32) {
        unsafe { ffi::Body_set_linear_damping(self.mut_ptr(), damping) }
    }

    pub fn set_angular_damping(&mut self, damping: f32) {
        unsafe { ffi::Body_set_angular_damping(self.mut_ptr(), damping) }
    }

    pub fn set_gravity_scale(&mut self, scale: f32) {
        unsafe { ffi::Body_set_gravity_scale(self.mut_ptr(), scale) }
    }

    pub fn set_body_type(&mut self, typ: BodyType) {
        unsafe { ffi::Body_set_type(self.mut_ptr(), typ) }
    }

    pub fn set_bullet(&mut self, flag: bool) {
        unsafe { ffi::Body_set_bullet(self.mut_ptr(), flag) }
    }

    pub fn set_sleeping_allowed(&mut self, flag: bool) {
        unsafe { ffi::Body_set_sleeping_allowed(self.mut_ptr(), flag) }
    }

    pub fn set_awake(&mut self, flag: bool) {
        unsafe { ffi::Body_set_awake(self.mut_ptr(), flag) }
    }

    pub fn set_active(&mut self, flag: bool) {
        unsafe { ffi::Body_set_active(self.mut_ptr(), flag) }
    }

    pub fn set_rotation_fixed(&mut self, flag: bool) {
        unsafe { ffi::Body_set_fixed_rotation(self.mut_ptr(), flag) }
    }

    pub fn dump(&mut self) {
        unsafe { ffi::Body_dump(self.mut_ptr()) }
    }
}
*/

pub struct ContactIter<'a> {
    edge: *const ContactEdge,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for ContactIter<'a> {
    type Item = (BodyHandle, WrappedRef<'a, Contact>);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.edge.as_ref() {
                None => None,
                Some(edge) => {
                    self.edge = edge.next;
                    Some((
                        panic!(), //edge.other.handle(),
                        WrappedRef::new(Contact::from_ffi(edge.contact)),
                    ))
                }
            }
        }
    }
}

pub struct ContactIterMut<'a> {
    edge: *mut ContactEdge,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for ContactIterMut<'a> {
    type Item = (BodyHandle, WrappedRefMut<'a, Contact>);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.edge.as_mut() {
                None => None,
                Some(ref mut edge) => {
                    self.edge = edge.next;
                    Some((
                        todo!(), //edge.other.handle(),
                        WrappedRefMut::new(Contact::from_ffi(edge.contact)),
                    ))
                }
            }
        }
    }
}

#[doc(hidden)]
pub mod ffi {
    use super::BodyType;
    pub use collision::shapes::ffi::Shape;
    use collision::shapes::MassData;
    use common::math::{Transform, Vec2};
    use dynamics::contacts::ContactEdge;
    pub use dynamics::fixture::ffi::Fixture;
    use dynamics::fixture::FixtureDef;
    pub use ffi::Any;

    pub enum Body {}

    pub fn Body_create_fixture(slf: *mut Body, def: *const FixtureDef) -> *mut Fixture {
        todo!()
    }
    pub fn Body_create_fast_fixture(
        slf: *mut Body,
        shape: *const Shape,
        density: f32,
    ) -> *mut Fixture {
        todo!()
    }
    pub fn Body_destroy_fixture(slf: *mut Body, fixture: *mut Fixture) {
        todo!()
    }
    pub fn Body_set_transform(slf: *mut Body, pos: *const Vec2, angle: f32) {
        todo!()
    }
    pub fn Body_get_transform(slf: *const Body) -> *const Transform {
        todo!()
    }
    pub fn Body_get_position(slf: *const Body) -> *const Vec2 {
        todo!()
    }
    pub fn Body_get_angle(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_get_world_center(slf: *const Body) -> *const Vec2 {
        todo!()
    }
    pub fn Body_get_local_center(slf: *const Body) -> *const Vec2 {
        todo!()
    }
    pub fn Body_set_linear_velocity(slf: *mut Body, v: *const Vec2) {
        todo!()
    }
    pub fn Body_get_linear_velocity(slf: *const Body) -> *const Vec2 {
        todo!()
    }
    pub fn Body_set_angular_velocity(slf: *mut Body, omega: f32) {
        todo!()
    }
    pub fn Body_get_angular_velocity(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_apply_force(slf: *mut Body, force: *const Vec2, point: *const Vec2, wake: bool) {
        todo!()
    }
    pub fn Body_apply_force_to_center(slf: *mut Body, force: *const Vec2, wake: bool) {
        todo!()
    }
    pub fn Body_apply_torque(slf: *mut Body, torque: f32, wake: bool) {
        todo!()
    }
    pub fn Body_apply_linear_impulse(
        slf: *mut Body,
        impulse: *const Vec2,
        point: *const Vec2,
        wake: bool,
    ) {
        todo!()
    }
    pub fn Body_apply_angular_impulse(slf: *mut Body, impulse: f32, wake: bool) {
        todo!()
    }
    pub fn Body_get_mass(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_get_inertia(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_get_mass_data(slf: *const Body, data: *mut MassData) {
        todo!()
    }
    pub fn Body_set_mass_data(slf: *mut Body, data: *const MassData) {
        todo!()
    }
    pub fn Body_reset_mass_data(slf: *mut Body) {
        todo!()
    }
    pub fn Body_get_world_point(slf: *const Body, local: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_world_vector(slf: *const Body, local: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_local_point(slf: *const Body, world: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_local_vector(slf: *const Body, world: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_linear_velocity_from_world_point(slf: *const Body, point: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_linear_velocity_from_local_point(slf: *const Body, point: *const Vec2) -> Vec2 {
        todo!()
    }
    pub fn Body_get_linear_damping(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_set_linear_damping(slf: *mut Body, damping: f32) {
        todo!()
    }
    pub fn Body_get_angular_damping(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_set_angular_damping(slf: *mut Body, damping: f32) {
        todo!()
    }
    pub fn Body_get_gravity_scale(slf: *const Body) -> f32 {
        todo!()
    }
    pub fn Body_set_gravity_scale(slf: *mut Body, scale: f32) {
        todo!()
    }
    pub fn Body_set_type(slf: *mut Body, typ: BodyType) {
        todo!()
    }
    pub fn Body_get_type(slf: *const Body) -> BodyType {
        todo!()
    }
    pub fn Body_set_bullet(slf: *mut Body, flag: bool) {
        todo!()
    }
    pub fn Body_is_bullet(slf: *const Body) -> bool {
        todo!()
    }
    pub fn Body_set_sleeping_allowed(slf: *mut Body, flag: bool) {
        todo!()
    }
    pub fn Body_is_sleeping_allowed(slf: *const Body) -> bool {
        todo!()
    }
    pub fn Body_set_awake(slf: *mut Body, flag: bool) {
        todo!()
    }
    pub fn Body_is_awake(slf: *const Body) -> bool {
        todo!()
    }
    pub fn Body_set_active(slf: *mut Body, flag: bool) {
        todo!()
    }
    pub fn Body_is_active(slf: *const Body) -> bool {
        todo!()
    }
    pub fn Body_set_fixed_rotation(slf: *mut Body, flag: bool) {
        todo!()
    }
    pub fn Body_is_fixed_rotation(slf: *const Body) -> bool {
        todo!()
    }
    pub fn Body_get_contact_list(slf: *mut Body) -> *mut ContactEdge {
        todo!()
    }
    pub fn Body_get_contact_list_const(slf: *const Body) -> *const ContactEdge {
        todo!()
    }
    // pub fn Body_get_next(slf: *mut Body) -> *mut Body {todo!()}
    // pub fn Body_get_next_const(slf: *const Body) -> *const Body {todo!()}
    // pub fn Body_get_world(slf: *mut Body) -> *mut World {todo!()}
    // pub fn Body_get_world_const(slf: *const Body) -> *const World {todo!()}
    pub fn Body_dump(slf: *mut Body) {
        todo!()
    }
}
