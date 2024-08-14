use crate::{math::{Rot, Vec2}, shapes::Polygon, PhantomNoSend, Shape, ShapeDef, World};

#[derive(Debug)]
#[repr(C)]
pub struct BodyDef
{
	/// The body type: static, kinematic, or dynamic.
	pub kind: BodyKind,

	/// The initial world position of the body. Bodies should be created with the desired position.
	/// @note Creating bodies at the origin and then moving them nearly doubles the cost of body creation, especially
	///	if the body is moved after shapes have been added.
	pub position: Vec2,

	/// The initial world rotation of the body. Use b2MakeRot() if you have an angle.
	pub rotation: Rot,

	/// The initial linear velocity of the body's origin. Typically in meters per second.
	pub linear_velocity: Vec2,

	/// The initial angular velocity of the body. Radians per second.
	pub angular_velocity: f32,

	/// Linear damping is use to reduce the linear velocity. The damping parameter
	/// can be larger than 1 but the damping effect becomes sensitive to the
	/// time step when the damping parameter is large.
	///	Generally linear damping is undesirable because it makes objects move slowly
	///	as if they are floating.
	pub linear_damping: f32,

	/// Angular damping is use to reduce the angular velocity. The damping parameter
	/// can be larger than 1.0f but the damping effect becomes sensitive to the
	/// time step when the damping parameter is large.
	///	Angular damping can be use slow down rotating bodies.
	pub angular_damping: f32,

	/// Scale the gravity applied to this body. Non-dimensional.
	pub gravity_scale: f32,

	/// Sleep velocity threshold, default is 0.05 meter per second
	pub sleep_threshold: f32,

	/// Use this to store application specific body data.
	pub user_data: *const std::ffi::c_void,

	/// Set this flag to false if this body should never fall asleep.
	pub enable_sleep: bool,

	/// Is this body initially awake or sleeping?
	pub is_awake: bool,

	/// Should this body be prevented from rotating? Useful for characters.
	pub fixed_rotation: bool,

	/// Treat this body as high speed object that performs continuous collision detection
	/// against dynamic and kinematic bodies, but not other bullet bodies.
	///	@warning Bullets should be used sparingly. They are not a solution for general dynamic-versus-dynamic
	///	continuous collision. They may interfere with joint constraints.
	pub is_bullet: bool,

	/// Used to disable a body. A disabled body does not move or collide.
	pub is_enabled: bool,

	/// Automatically compute mass and related properties on this body from shapes.
	/// Triggers whenever a shape is add/removed/changed. Default is true.
	pub automatic_mass: bool,

	/// This allows this body to bypass rotational speed limits. Should only be used
	///	for circular objects, like wheels.
	pub allow_fast_rotation: bool,

	/// Used internally to detect a valid definition. DO NOT SET.
	_cookie: u32,
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct Body {
    index: u32,
    world: u16,
    revision: u16,
    _thread_unsafe: PhantomNoSend
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum BodyKind {
    Static = 0,
    Kinematic = 1,
    Dynamic = 2
}

impl Default for BodyDef {
    fn default() -> Self {
        unsafe {
            b2DefaultBodyDef()
        }
    }
}

impl Body {
	pub fn create_shape_polygon(&self, shape_def: &ShapeDef, polygon: &Polygon) -> Shape {
		unsafe {
			b2CreatePolygonShape(*self, shape_def, polygon)
		}
	}

	pub fn set_angular_velocity(&self, ang_vel: f32) {
		unsafe {
			b2Body_SetAngularVelocity(*self, ang_vel);
		}
	}
}

extern "C" {
	fn b2CreatePolygonShape(body: Body, shape_def: &ShapeDef, polygon: &Polygon) -> Shape;

	fn b2Body_SetAngularVelocity(body: Body, ang_vel: f32);

    fn b2DefaultBodyDef() -> BodyDef;
}
