use crate::{math::Vec2, PhantomNoSend};

#[repr(C)]
#[derive(Debug)]
pub struct WorldDef {
    /// Gravity vector. Box2D has no up-vector defined.
	pub gravity: Vec2,

	/// Restitution velocity threshold, usually in m/s. Collisions above this
	/// speed have restitution applied (will bounce).
	pub restitution_threshold: f32,

	/// This parameter controls how fast overlap is resolved and has units of meters per second
	pub contact_pushout_velocity: f32,

	/// Threshold velocity for hit events. Usually meters per second.
	pub hit_event_threshold: f32,

	/// Contact stiffness. Cycles per second.
	pub contact_hertz: f32,

	/// Contact bounciness. Non-dimensional.
	pub contact_damping_ratio: f32,

	/// Joint stiffness. Cycles per second.
	pub joint_hertz: f32,

	/// Joint bounciness. Non-dimensional.
	pub joint_damping_ratio: f32,

	/// Maximum linear velocity. Usually meters per second.
	pub maximum_linear_velocity: f32,

	/// Can bodies go to sleep to improve performance
	pub enable_sleep: bool,

	/// Enable continuous collision
	pub enable_continuous: bool,

	/// Number of workers to use with the provided task system. Box2D performs best when using only
	///	performance cores and accessing a single L2 cache. Efficiency cores and hyper-threading provide
	///	little benefit and may even harm performance.
	pub worker_count: u32,

	/// Function to spawn tasks
	enqueue_task: *const std::ffi::c_void,

	/// Function to finish a task
	finish_task: *const std::ffi::c_void,

	/// User context that is provided to enqueueTask and finishTask
	user_task_context: *const std::ffi::c_void,

	/// Used internally to detect a valid definition. DO NOT SET.
	_cookie: u32
}

impl Default for WorldDef {
    fn default() -> Self {
        unsafe {
            b2DefaultWorldDef()
        }
    }
}

#[repr(C)]
pub struct World {
    index: u16,
    revision: u16,
    _thread_unsafe: PhantomNoSend
}

impl World {
    pub fn new(def: &WorldDef) -> Self {
        unsafe {
            b2CreateWorld(def)
        }
    }
}

extern "C" {
    fn b2DefaultWorldDef() -> WorldDef;

    fn b2CreateWorld(def: &WorldDef) -> World;
}
