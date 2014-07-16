pub use self::joints::{
    UNKNOWN_JOINT, UnknownJoint, JointType, JointDefBase, Joint,
    DISTANCE_JOINT, DistanceJointDef, DistanceJoint,
    FRICTION_JOINT, FrictionJointDef, FrictionJoint,
    GEAR_JOINT, GearJointDef, GearJoint,
    MOTOR_JOINT, MotorJointDef, MotorJoint,
    MOUSE_JOINT, MouseJointDef, MouseJoint,
    PRISMATIC_JOINT, PrismaticJointDef, PrismaticJoint,
    PULLEY_JOINT, PulleyJointDef, PulleyJoint,
    REVOLUTE_JOINT, RevoluteJointDef, RevoluteJoint,
    ROPE_JOINT, RopeJointDef, RopeJoint,
    WELD_JOINT, WeldJointDef, WeldJoint,
    WHEEL_JOINT, WheelJointDef, WheelJoint
};

use std::ptr;
use std::mem;
use {ffi, Wrapped, clone_from_ptr, settings};
use common::{Draw, DrawLink};
use common::private::DerivedDraw;
use math::{Vec2, Transform};
use dynamics::joints::private::{WrappedJoint, JointDef};
use collision::{
    RayCastInput, RayCastOutput, AABB,
    Shape, ShapeType, UnknownShape, MassData
};
use collision::shapes::private::WrappedShape;

pub mod joints;

wrap!(ffi::World into World)

#[deriving(Clone)]
pub struct Profile {
    pub step: f32,
    pub collide: f32,
    pub solve: f32,
    pub solve_init: f32,
    pub solve_velocity: f32,
    pub solve_position: f32,
    pub brad_phase: f32,
    pub solve_TOI: f32
}

impl World {
    pub fn new(gravity: &Vec2) -> World {
        unsafe {
            Wrapped::from_ptr(ffi::World_new(gravity))
        }
    }
    pub fn set_destruction_listener<T: DestructionListener>(
            &mut self, dll: &mut DestructionListenerLink<T>) {
        unsafe {
            ffi::World_set_destruction_listener(self.mut_ptr(), dll.as_ffi_base())
        }
    }
    pub fn set_contact_filter<T: ContactFilter>(
            &mut self, cfl: &mut ContactFilterLink<T>) {
        unsafe {
            ffi::World_set_contact_filter(self.mut_ptr(), cfl.as_ffi_base())
        }
    }
    pub fn set_contact_listener<T: ContactListener>(
            &mut self, cll: &mut ContactListenerLink<T>) {
        unsafe {
            ffi::World_set_contact_listener(self.mut_ptr(), cll.as_ffi_base())
        }
    }
    pub fn set_debug_draw<T: Draw>(&mut self, dl: &mut DrawLink<T>) {
        unsafe {
            ffi::World_set_debug_draw(self.mut_ptr(), dl.mut_draw_ptr())
        }
    }
    pub fn create_body(&mut self, def: &BodyDef) -> Body {
        unsafe {
            Wrapped::from_ptr(
                ffi::World_create_body(self.mut_ptr(), def)
                )
        }
    }
    pub fn destroy_body(&mut self, body: Body) {
        unsafe {
            let mut body = body;
            ffi::World_destroy_body(self.mut_ptr(), body.mut_ptr())
        }
    }
    pub fn create_joint<J: Joint>(&mut self, def: &JointDef) -> J {
        unsafe {
            let joint: J = WrappedJoint::from_joint_ptr(
                ffi::World_create_joint(self.mut_ptr(), def.joint_def_ptr())
                );
            assert!(
                joint.joint_type() == WrappedJoint::joint_type(None::<*const J>)
                || self::UNKNOWN_JOINT == WrappedJoint::joint_type(None::<*const J>)
                )
            joint
        }
    }
    pub fn destroy_joint<J: Joint>(&mut self, joint: J) {
        unsafe {
            let mut joint = joint;
            ffi::World_destroy_joint(self.mut_ptr(), joint.mut_joint_ptr())
        }
    }
    pub fn step(&mut self,
                time_step: f32,
                velocity_iterations: i32,
                position_iterations: i32) {
        unsafe {
            ffi::World_step(self.mut_ptr(),
                            time_step,
                            velocity_iterations,
                            position_iterations)
        }
    }
    pub fn clear_forces(&mut self) {
        unsafe {
            ffi::World_clear_forces(self.mut_ptr())
        }
    }
    pub fn draw_debug_data(&mut self) {
        unsafe {
            ffi::World_draw_debug_data(self.mut_ptr())
        }
    }
    pub fn query_aabb<T: QueryCallback>(&self, qcl: &mut QueryCallbackLink<T>,
                                        aabb: &AABB) {
        unsafe {
            ffi::World_query_aabb(self.ptr(), qcl.as_ffi_base(), aabb)
        }
    }
    pub fn ray_cast<T: RayCastCallback>(&self, rccl: &mut RayCastCallbackLink<T>,
                                        p1: &Vec2, p2: &Vec2) {
        unsafe {
            ffi::World_ray_cast(self.ptr(), rccl.as_ffi_base(), p1, p2)
        }
    }
    /*pub fn mut_body_list(&mut self) -> Vec<Body> {
        unsafe {
            let mut ptr = ffi::World_get_body_list(self.mut_ptr());
            
            let mut vec = Vec::new();
            while !ptr.is_null() {
                vec.push(Wrapped::from_ptr(ptr));
                ptr = ffi::Body_get_next(ptr);
            }
            vec
        }
    }*/
    pub fn set_sleeping_allowed(&mut self, flag: bool) {
        unsafe {
            ffi::World_set_allow_sleeping(self.mut_ptr(), flag)
        }
    }
    pub fn is_sleeping_allowed(&self) -> bool {
        unsafe {
            ffi::World_get_allow_sleeping(self.ptr())
        }
    }
    pub fn set_warm_starting(&mut self, flag: bool) {
        unsafe {
            ffi::World_set_warm_starting(self.mut_ptr(), flag)
        }
    }
    pub fn is_warm_starting(&self) -> bool {
        unsafe {
            ffi::World_get_warm_starting(self.ptr())
        }
    }
    pub fn set_continuous_physics(&mut self, flag: bool) {
        unsafe {
            ffi::World_set_continuous_physics(self.mut_ptr(), flag)
        }
    }
    pub fn is_continuous_physics(&self) -> bool {
        unsafe {
            ffi::World_get_continuous_physics(self.ptr())
        }
    }
    pub fn set_sub_stepping(&mut self, flag: bool) {
        unsafe {
            ffi::World_set_sub_stepping(self.mut_ptr(), flag)
        }
    }
    pub fn is_sub_stepping(&self) -> bool {
        unsafe {
            ffi::World_get_sub_stepping(self.ptr())
        }
    }
    pub fn proxy_count(&self) -> uint {
        unsafe {
            ffi::World_get_proxy_count(self.ptr()) as uint
        }
    }
    pub fn body_count(&self) -> uint {
        unsafe {
            ffi::World_get_body_count(self.ptr()) as uint
        }
    }
    pub fn joint_count(&self) -> uint {
        unsafe {
            ffi::World_get_joint_count(self.ptr()) as uint
        }
    }
    pub fn contact_count(&self) -> uint {
        unsafe {
            ffi::World_get_contact_count(self.ptr()) as uint
        }
    }
    pub fn tree_height(&self) -> i32 {
        unsafe {
            ffi::World_get_tree_height(self.ptr())
        }
    }
    pub fn tree_balance(&self) -> i32 {
        unsafe {
            ffi::World_get_tree_balance(self.ptr())
        }
    }
    pub fn tree_quality(&self) -> f32 {
        unsafe {
            ffi::World_get_tree_quality(self.ptr())
        }
    }
    pub fn set_gravity(&mut self, gravity: &Vec2) {
        unsafe {
            ffi::World_set_gravity(self.mut_ptr(), gravity)
        }
    }
    pub fn gravity(&self) -> Vec2 {
        unsafe {
            ffi::World_get_gravity(self.ptr())
        }
    }
    pub fn is_locked(&self) -> bool {
        unsafe {
            ffi::World_is_locked(self.ptr())
        }
    }
    pub fn set_auto_clearing_forces(&mut self, flag: bool) {
        unsafe {
            ffi::World_set_auto_clear_forces(self.mut_ptr(), flag)
        }
    }
    pub fn is_auto_clearing_forces(&self) -> bool {
        unsafe {
            ffi::World_get_auto_clear_forces(self.ptr())
        }
    }
    pub fn shift_origin(&mut self, origin: &Vec2) {
        unsafe {
            ffi::World_shift_origin(self.mut_ptr(), origin)
        }
    }
    pub fn profile(&self) -> Profile {
        unsafe {
            clone_from_ptr(ffi::World_get_profile(self.ptr()))
        }
    }
    pub fn dump(&mut self) {
        unsafe {
            ffi::World_dump(self.mut_ptr())
        }
    }
}

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            ffi::World_drop(self.mut_ptr())
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[deriving(PartialEq, Show)]
pub enum BodyType {
    STATIC_BODY = 0,
    KINEMATIC_BODY = 1,
    DYNAMIC_BODY = 2
}

#[allow(dead_code)]
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
    user_data: ffi::Any,
    pub gravity_scale: f32,
}

impl BodyDef {
    pub fn new() -> BodyDef {
        BodyDef {
            body_type: STATIC_BODY,
            position: Vec2 { x:0., y:0. },
            angle: 0.,
            linear_velocity: Vec2 { x:0., y:0. },
            angular_velocity: 0.,
            linear_damping: 0.,
            angular_damping: 0.,
            allow_sleep: true,
            awake: true,
            fixed_rotation: false,
            bullet: false,
            active: true,
            user_data: ptr::mut_null(),
            gravity_scale: 1.
        }
    }
    pub unsafe fn set_user_data<T>(&mut self, data: *mut T) {
        self.user_data = data as ffi::Any
    }
}

wrap!(ffi::Body into Body)

impl Body {
    pub fn create_fixture(&mut self, def: &FixtureDef) -> Fixture {
        unsafe {
            Wrapped::from_ptr(ffi::Body_create_fixture(self.mut_ptr(), def))
        }
    }
    pub fn create_fast_fixture(&mut self, shape: &Shape, density: f32) -> Fixture {
        unsafe {
            Wrapped::from_ptr(
                ffi::Body_create_fast_fixture(self.mut_ptr(),
                                              shape.shape_ptr(),
                                              density)
                )
        }
    }
    pub fn destroy_fixture(&mut self, fixture: Fixture) {
        unsafe {
            let mut fixture = fixture;
            ffi::Body_destroy_fixture(self.mut_ptr(), fixture.mut_ptr())
        }
    }
    pub fn set_transform(&mut self, pos: &Vec2, angle: f32) {
        unsafe {
            ffi::Body_set_transform(self.mut_ptr(), pos, angle)
        }
    }
    pub fn transform(&self) -> Transform {
        unsafe {
            clone_from_ptr(ffi::Body_get_transform(self.ptr()))
        }
    }
    pub fn position(&self) -> Vec2 {
        unsafe {
            clone_from_ptr(ffi::Body_get_position(self.ptr()))
        }
    }
    pub fn angle(&self) -> f32 {
        unsafe {
            ffi::Body_get_angle(self.ptr())
        }
    }
    pub fn world_center(&self) -> Vec2 {
        unsafe {
            clone_from_ptr(ffi::Body_get_world_center(self.ptr()))
        }
    }
    pub fn local_center(&self) -> Vec2 {
        unsafe {
            clone_from_ptr(ffi::Body_get_local_center(self.ptr()))
        }
    }
    pub fn set_linear_velocity(&mut self, v: &Vec2) {
        unsafe {
            ffi::Body_set_linear_velocity(self.mut_ptr(), v)
        }
    }
    pub fn linear_velocity(&self) -> Vec2 {
        unsafe {
            clone_from_ptr(ffi::Body_get_linear_velocity(self.ptr()))
        }
    }
    pub fn set_angular_velocity(&mut self, v: f32) {
        unsafe {
            ffi::Body_set_angular_velocity(self.mut_ptr(), v)
        }
    }
    pub fn angular_velocity(&self) -> f32 {
        unsafe {
            ffi::Body_get_angular_velocity(self.ptr())
        }
    }
    pub fn apply_force(&mut self, force: &Vec2, point: &Vec2, wake: bool) {
        unsafe {
            ffi::Body_apply_force(self.mut_ptr(), force, point, wake)
        }
    }
    pub fn apply_force_to_center(&mut self, force: &Vec2, wake: bool) {
        unsafe {
            ffi::Body_apply_force_to_center(self.mut_ptr(), force, wake)
        }
    }
    pub fn apply_torque(&mut self, torque: f32, wake: bool) {
        unsafe {
            ffi::Body_apply_torque(self.mut_ptr(), torque, wake)
        }
    }
    pub fn apply_linear_impulse(&mut self, impulse: &Vec2,
                                point: &Vec2, wake: bool) {
        unsafe {
            ffi::Body_apply_linear_impulse(self.mut_ptr(), impulse,
                                           point, wake)
        }
    }
    pub fn apply_angular_impulse(&mut self, impulse: f32, wake: bool) {
        unsafe {
            ffi::Body_apply_angular_impulse(self.mut_ptr(), impulse, wake)
        }
    }
    pub fn mass(&self) -> f32 {
        unsafe {
            ffi::Body_get_mass(self.ptr())
        }
    }
    pub fn inertia(&self) -> f32 {
        unsafe {
            ffi::Body_get_inertia(self.ptr())
        }
    }
    pub fn mass_data(&self) -> MassData {
        unsafe {
            let mut data = MassData::new();
            ffi::Body_get_mass_data(self.ptr(), &mut data);
            data
        }
    }
    pub fn set_mass_data(&mut self, data: &MassData) {
        unsafe {
            ffi::Body_set_mass_data(self.mut_ptr(), data)
        }
    }
    pub fn reset_mass_data(&mut self) {
        unsafe {
            ffi::Body_reset_mass_data(self.mut_ptr())
        }
    }
    pub fn world_point(&self, local: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_world_point(self.ptr(), local)
        }
    }
    pub fn world_vector(&self, local: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_world_vector(self.ptr(), local)
        }
    }
    pub fn local_point(&self, world: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_local_point(self.ptr(), world)
        }
    }
    pub fn local_vector(&self, world: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_local_vector(self.ptr(), world)
        }
    }
    pub fn linear_velocity_from_world_point(&self, world: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_linear_velocity_from_world_point(self.ptr(), world)
        }
    }
    pub fn linear_velocity_from_local_point(&self, local: &Vec2) -> Vec2 {
        unsafe {
            ffi::Body_get_linear_velocity_from_local_point(self.ptr(), local)
        }
    }
    pub fn linear_damping(&self) -> f32 {
        unsafe {
            ffi::Body_get_linear_damping(self.ptr())
        }
    }
    pub fn set_linear_damping(&mut self, damping: f32) {
        unsafe {
            ffi::Body_set_linear_damping(self.mut_ptr(), damping)
        }
    }
    pub fn angular_damping(&self) -> f32 {
        unsafe {
            ffi::Body_get_angular_damping(self.ptr())
        }
    }
    pub fn set_angular_damping(&mut self, damping: f32) {
        unsafe {
            ffi::Body_set_angular_damping(self.mut_ptr(), damping)
        }
    }
    pub fn gravity_scale(&self) -> f32 {
        unsafe {
            ffi::Body_get_gravity_scale(self.ptr())
        }
    }
    pub fn set_gravity_scale(&mut self, scale: f32) {
        unsafe {
            ffi::Body_set_gravity_scale(self.mut_ptr(), scale)
        }
    }
    pub fn set_body_type(&mut self, typ: BodyType) {
        unsafe {
            ffi::Body_set_type(self.mut_ptr(), typ)
        }
    }
    pub fn body_type(&self) -> BodyType {
        unsafe {
            ffi::Body_get_type(self.ptr())
        }
    }
    pub fn set_bullet(&mut self, flag: bool) {
        unsafe {
            ffi::Body_set_bullet(self.mut_ptr(), flag)
        }
    }
    pub fn is_bullet(&self) -> bool {
        unsafe {
            ffi::Body_is_bullet(self.ptr())
        }
    }
    pub fn set_sleeping_allowed(&mut self, flag: bool) {
        unsafe {
            ffi::Body_set_sleeping_allowed(self.mut_ptr(), flag)
        }
    }
    pub fn is_sleeping_allowed(&self) -> bool {
        unsafe {
            ffi::Body_is_sleeping_allowed(self.ptr())
        }
    }
    pub fn set_awake(&mut self, flag: bool) {
        unsafe {
            ffi::Body_set_awake(self.mut_ptr(), flag)
        }
    }
    pub fn is_awake(&self) -> bool {
        unsafe {
            ffi::Body_is_awake(self.ptr())
        }
    }
    pub fn set_active(&mut self, flag: bool) {
        unsafe {
            ffi::Body_set_active(self.mut_ptr(), flag)
        }
    }
    pub fn is_active(&self) -> bool {
        unsafe {
            ffi::Body_is_active(self.ptr())
        }
    }
    pub fn set_rotation_fixed(&mut self, flag: bool) {
        unsafe {
            ffi::Body_set_fixed_rotation(self.mut_ptr(), flag)
        }
    }
    pub fn is_rotation_fixed(&self) -> bool {
        unsafe {
            ffi::Body_is_fixed_rotation(self.ptr())
        }
    }
    /*pub fn mut_fixture_list(&mut self) -> Vec<Fixture> {
        unsafe {
        
        }
    }*/
    pub fn mut_next(&mut self) -> Body {
        unsafe {
            Wrapped::from_ptr(ffi::Body_get_next(self.mut_ptr()))
        }
    }
    pub unsafe fn set_user_data<T>(&mut self, data: *mut T) {
        ffi::Body_set_user_data(self.mut_ptr(), data as ffi::Any)
    }
    pub unsafe fn user_data<T>(&self) -> *mut T {
        ffi::Body_get_user_data(self.ptr()) as *mut T
    }
    pub fn mut_world(&mut self) -> World {
        unsafe {
            Wrapped::from_ptr(ffi::Body_get_world(self.mut_ptr()))
        }
    }
    pub fn dump(&mut self) {
        unsafe {
            ffi::Body_dump(self.mut_ptr())
        }
    }
}

#[allow(dead_code)]
#[deriving(Clone)]
pub struct Filter {
    pub category_bits: u16,
    pub mask_bits: u16,
    pub group_index: i16
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            category_bits: 0x0001,
            mask_bits: 0xFFFF,
            group_index: 0
        }
    }
}

#[allow(dead_code)]
pub struct FixtureDef {
    shape: *const ffi::Shape,
    user_data: ffi::Any,
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub is_sensor: bool,
    pub filter: Filter
}

impl FixtureDef {
    pub fn new(shape: &Shape) -> FixtureDef {
        unsafe {
            FixtureDef {
                shape: shape.shape_ptr(),
                user_data: ptr::mut_null(),
                friction: 0.2,
                restitution: 0.,
                density: 0.,
                is_sensor: false,
                filter: Filter::new()
            }
        }
    }
    pub unsafe fn set_user_data<T>(&mut self, data: *mut T) {
        self.user_data = data as ffi::Any
    }
}

wrap!(ffi::Fixture into Fixture)

impl Fixture {
    pub fn shape_type(&self) -> ShapeType {
        unsafe {
            ffi::Fixture_get_type(self.ptr())
        }
    }
    pub fn shape(&mut self) -> UnknownShape {
        unsafe {
            WrappedShape::from_shape_ptr(
                ffi::Fixture_get_shape(self.mut_ptr())
                )
        }
    }
    pub fn set_sensor(&mut self, flag: bool) {
        unsafe {
            ffi::Fixture_set_sensor(self.mut_ptr(), flag)
        }
    }
    pub fn is_sensor(&self) -> bool {
        unsafe {
            ffi::Fixture_is_sensor(self.ptr())
        }
    }
    pub fn set_filter_data(&mut self, filter: &Filter) {
        unsafe {
            ffi::Fixture_set_filter_data(self.mut_ptr(), filter)
        }
    }
    pub fn filter_data(&self) -> Filter {
        unsafe {
            clone_from_ptr(ffi::Fixture_get_filter_data(self.ptr()))
        }
    }
    pub fn refilter(&mut self) {
        unsafe {
            ffi::Fixture_refilter(self.mut_ptr())
        }
    }
    pub fn mut_body(&mut self) -> Body {
        unsafe {
            Wrapped::from_ptr(ffi::Fixture_get_body(self.mut_ptr()))
        }
    }
    pub fn mut_next(&mut self) -> Fixture {
        unsafe {
            Wrapped::from_ptr(ffi::Fixture_get_next(self.mut_ptr()))
        }
    }
    pub fn test_point(&self, point: &Vec2) -> bool {
        unsafe {
            ffi::Fixture_test_point(self.ptr(), point)
        }
    }
    pub fn ray_cast(&self, input: &RayCastInput, child_index: uint
                    ) -> RayCastOutput {
        unsafe {
            let mut output = RayCastOutput::new();
            ffi::Fixture_ray_cast(self.ptr(), &mut output,
                                  input, child_index as i32);
            output
        }
    }
    pub fn mass_data(&self) -> MassData {
        unsafe {
            let mut data = MassData::new();
            ffi::Fixture_get_mass_data(self.ptr(), &mut data);
            data
        }
    }
    pub fn set_density(&mut self, density: f32) {
        unsafe {
            ffi::Fixture_set_density(self.mut_ptr(), density)
        }
    }
    pub fn density(&self) -> f32 {
        unsafe {
            ffi::Fixture_get_density(self.ptr())
        }
    }
    pub fn friction(&self) -> f32 {
        unsafe {
            ffi::Fixture_get_friction(self.ptr())
        }
    }
    pub fn set_friction(&mut self, friction: f32) {
        unsafe {
            ffi::Fixture_set_friction(self.mut_ptr(), friction)
        }
    }
    pub fn restitution(&self) -> f32 {
        unsafe {
            ffi::Fixture_get_restitution(self.ptr())
        }
    }
    pub fn set_restitution(&mut self, restitution: f32) {
        unsafe {
            ffi::Fixture_set_restitution(self.mut_ptr(), restitution)
        }
    }
    pub fn aabb(&self, child_index: uint) -> AABB {
        unsafe {
            clone_from_ptr(ffi::Fixture_get_aabb(self.ptr(), child_index as i32))
        }
    }
    pub unsafe fn user_data<T>(&self) -> *mut T {
        ffi::Fixture_get_user_data(self.ptr()) as *mut T
    }
    pub unsafe fn set_user_data<T>(&mut self, data: *mut T) {
        ffi::Fixture_set_user_data(self.mut_ptr(), data as ffi::Any)
    }
    pub fn dump(&mut self, child_count: uint) {
        unsafe {
            ffi::Fixture_dump(self.mut_ptr(), child_count as i32)
        }
    }
}

pub struct ContactImpulse {
    pub normal_impulses: [f32, ..settings::MAX_MANIFOLD_POINTS],
    pub tangent_impulses: [f32, ..settings::MAX_MANIFOLD_POINTS],
    pub count: i32
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[deriving(PartialEq, Show)]
pub enum ManifoldType {
    CIRCLES_MANIFOLD = 0,
    FACE_A_MANIFOLD = 1,
    FACE_B_MANIFOLD = 2
}

pub struct Manifold {
    pub points: [ManifoldPoint, ..settings::MAX_MANIFOLD_POINTS],
    pub local_normal: Vec2,
    pub local_point: Vec2,
    pub manifold_type: ManifoldType,
    pub count: i32
}

pub struct ManifoldPoint {
    pub local_point: Vec2,
    pub normal_impulse: f32,
    pub tangent_impulse: f32,
    pub id: u32
}

wrap!(ffi::Contact into Contact)

/*impl Contact {

}*/

pub trait DestructionListener {
    fn goodbye_joint(&mut self, joint: UnknownJoint);
    fn goodbye_fixture(&mut self, fixture: Fixture);
}

pub trait ContactFilter {
    fn should_collide(&mut self, fixture_a: Fixture, fixture_b: Fixture) -> bool;
}

pub trait ContactListener {
    fn begin_contact(&mut self, contact: Contact);
    fn end_contact(&mut self, contact: Contact);
    fn pre_solve(&mut self, contact: Contact, manifold: &Manifold);
    fn post_solve(&mut self, contact: Contact, impulse: &ContactImpulse);
}

pub trait QueryCallback {
    fn report_fixture(&mut self, fixture: Fixture) -> bool;
}

pub trait RayCastCallback {
    fn report_fixture(&mut self, fixture: Fixture, p: &Vec2, normal: &Vec2,
                      fraction: f32) -> f32;
}

unsafe extern fn goodbye_joint(any: ffi::Any, joint: *mut ffi::Joint) {
    assert!(!any.is_null())
    let listener = mem::transmute::<_, *mut &mut DestructionListener>(any);
    (*listener).goodbye_joint(WrappedJoint::from_joint_ptr(joint))
}
unsafe extern fn goodbye_fixture(any: ffi::Any, fixture: *mut ffi::Fixture) {
    assert!(!any.is_null())
    let listener = mem::transmute::<_, *mut &mut DestructionListener>(any);
    (*listener).goodbye_fixture(Wrapped::from_ptr(fixture))
}

unsafe extern fn should_collide(any: ffi::Any, fixture_a: *mut ffi::Fixture,
                                fixture_b: *mut ffi::Fixture) -> bool {
    assert!(!any.is_null())
    let filter = mem::transmute::<_, *mut &mut ContactFilter>(any);
    (*filter).should_collide(Wrapped::from_ptr(fixture_a),
                          Wrapped::from_ptr(fixture_b))
}

unsafe extern fn begin_contact(any: ffi::Any, contact: *mut ffi::Contact) {
    assert!(!any.is_null())
    let listener = mem::transmute::<_, *mut &mut ContactListener>(any);
    (*listener).begin_contact(Wrapped::from_ptr(contact))
}
unsafe extern fn end_contact(any: ffi::Any, contact: *mut ffi::Contact) {
    assert!(!any.is_null())
    let listener = mem::transmute::<_, *mut &mut ContactListener>(any);
    (*listener).end_contact(Wrapped::from_ptr(contact))
}
unsafe extern fn pre_solve(any: ffi::Any, contact: *mut ffi::Contact,
                           old_manifold: *const Manifold) {
    assert!(!any.is_null())
    assert!(!old_manifold.is_null())
    let listener = mem::transmute::<_, *mut &mut ContactListener>(any);
    (*listener).pre_solve(Wrapped::from_ptr(contact), &*old_manifold)
}
unsafe extern fn post_solve(any: ffi::Any, contact: *mut ffi::Contact,
                            impulse: *const ContactImpulse) {
    assert!(!any.is_null())
    assert!(!impulse.is_null())
    let listener = mem::transmute::<_, *mut &mut ContactListener>(any);
    (*listener).post_solve(Wrapped::from_ptr(contact), &*impulse)
}

unsafe extern fn qc_report_fixture(any: ffi::Any, fixture: *mut ffi::Fixture
                                   ) -> bool {
    assert!(!any.is_null())
    let callback = mem::transmute::<_, *mut &mut QueryCallback>(any);
    (*callback).report_fixture(Wrapped::from_ptr(fixture))
}

unsafe extern fn rcc_report_fixture(any: ffi::Any, fixture: *mut ffi::Fixture,
                                    point: *const Vec2, normal: *const Vec2,
                                    fraction: f32) -> f32 {
    assert!(!any.is_null())
    assert!(!point.is_null())
    assert!(!normal.is_null())
    let callback = mem::transmute::<_, *mut &mut RayCastCallback>(any);
    (*callback).report_fixture(Wrapped::from_ptr(fixture), &*point, &*normal,
                            fraction)
}

pub struct DestructionListenerLink<T> {
    t: T,
    c: *mut ffi::CDestructionListener
}

pub struct ContactFilterLink<T> {
    t: T,
    c: *mut ffi::CContactFilter
}

pub struct ContactListenerLink<T> {
    t: T,
    c: *mut ffi::CContactListener
}

pub struct QueryCallbackLink<T> {
    t: T,
    c: *mut ffi::CQueryCallback
}

pub struct RayCastCallbackLink<T> {
    t: T,
    c: *mut ffi::CRayCastCallback
}

impl<T: DestructionListener> DestructionListenerLink<T> {
    pub fn with(t: T) -> DestructionListenerLink<T> {
        unsafe {
            let mut link = DestructionListenerLink {
                t: t,
                c: ptr::mut_null()
            };
            link.c = ffi::CDestructionListener_new(mem::transmute(&mut &mut link.t),
                                                   goodbye_joint,
                                                   goodbye_fixture);
            link
        }
    }
    unsafe fn as_ffi_base(&mut self) -> *mut ffi::DestructionListener {
        ffi::CDestructionListener_as_base(self.c)
    }
}

impl<T: ContactFilter> ContactFilterLink<T> {
    pub fn with(t: T) -> ContactFilterLink<T> {
        unsafe {
            let mut link = ContactFilterLink {
                t: t,
                c: ptr::mut_null()
            };
            link.c = ffi::CContactFilter_new(mem::transmute(&mut &mut link.t),
                                             should_collide);
            link
        }
    }
    unsafe fn as_ffi_base(&mut self) -> *mut ffi::ContactFilter {
        ffi::CContactFilter_as_base(self.c)
    }
}

impl<T: ContactListener> ContactListenerLink<T> {
    pub fn with(t: T) -> ContactListenerLink<T> {
        unsafe {
            let mut link = ContactListenerLink {
                t: t,
                c: ptr::mut_null()
            };
            link.c = ffi::CContactListener_new(mem::transmute(&mut &mut link.t),
                                               begin_contact,
                                               end_contact,
                                               pre_solve,
                                               post_solve);
            link
        }          
    }
    unsafe fn as_ffi_base(&mut self) -> *mut ffi::ContactListener {
        ffi::CContactListener_as_base(self.c)
    }
}

impl<T: QueryCallback> QueryCallbackLink<T> {
    pub fn with(t: T) -> QueryCallbackLink<T> {
        unsafe {
            let mut link = QueryCallbackLink {
                t: t,
                c: ptr::mut_null()
            };
            link.c = ffi::CQueryCallback_new(mem::transmute(&mut (&mut link.t as &mut QueryCallback)),
                                             qc_report_fixture);
            link
        }
    }
    unsafe fn as_ffi_base(&mut self) -> *mut ffi::QueryCallback {
        ffi::CQueryCallback_as_base(self.c)
    }
}

impl<T: RayCastCallback> RayCastCallbackLink<T> {
    pub fn with(t: T) -> RayCastCallbackLink<T> {
        unsafe {
            let mut link = RayCastCallbackLink {
                t: t,
                c: ptr::mut_null()
            };
            link.c = ffi::CRayCastCallback_new(mem::transmute(&mut &mut link.t),
                                               rcc_report_fixture);
            link
        }
    }
    unsafe fn as_ffi_base(&mut self) -> *mut ffi::RayCastCallback {
        ffi::CRayCastCallback_as_base(self.c)
    }
}

#[unsafe_destructor]
impl<T> Drop for DestructionListenerLink<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::CDestructionListener_drop(self.c)
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for ContactFilterLink<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::CContactFilter_drop(self.c)
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for ContactListenerLink<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::CContactListener_drop(self.c)
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for QueryCallbackLink<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::CQueryCallback_drop(self.c)
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for RayCastCallbackLink<T> {
    fn drop(&mut self) {
        unsafe  {
            ffi::CRayCastCallback_drop(self.c)
        }
    }
}