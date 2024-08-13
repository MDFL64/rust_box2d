use crate::{common::HexColor, math::{Transform, Vec2, AABB}, PhantomNoSend};

#[repr(C)]
pub(crate) struct DebugDrawOpaque {
    _data: [u8;0],
    _thread_unsafe: PhantomNoSend
}

#[repr(C)]
pub struct DebugDraw<C>
{
    pub draw_polygon: fn(vertices: *const Vec2, vertex_count: u32, color: HexColor, context: *mut C),
    pub draw_solid_polygon: fn(transform: Transform, vertices: *const Vec2, vertex_count: u32, radius: f32, color: HexColor, context: *mut C),
    
    pub draw_circle: fn(center: Vec2, radius: f32, color: HexColor, context: *mut C),
    pub draw_solid_circle: fn(transform: Transform, radius: f32, color: HexColor, context: *mut C),

    pub draw_capsule: fn(p1: Vec2, p2: Vec2, radius: f32, color: HexColor, context: *mut C),
    pub draw_solid_capsule: fn(p1: Vec2, p2: Vec2, radius: f32, color: HexColor, context: *mut C),

    pub draw_segment: fn(p1: Vec2, p2: Vec2, color: HexColor, context: *mut C),
    pub draw_transform: fn(transform: Transform, context: *mut C),
    pub draw_point: fn(pos: Vec2, size: f32, color: HexColor, context: *mut C),

    pub draw_string: fn(pos: Vec2, string: *const std::ffi::c_char, context: *mut C),

    pub drawing_bounds: AABB,
 
    pub use_drawing_bounds: bool,

    pub draw_shapes: bool,
    pub draw_joints: bool,
    pub draw_joint_extras: bool,
 
    pub draw_aabbs: bool,
    pub draw_mass: bool,
    pub draw_contacts: bool,
 
    pub draw_graph_colors: bool,
    pub draw_contact_normals: bool,
    pub draw_contact_impulses: bool,
    pub draw_friction_impulses: bool,

    pub context: *mut C
}

impl<C> DebugDraw<C> {
    pub(crate) fn as_opaque(&self) -> *const DebugDrawOpaque {
        unsafe {
            std::mem::transmute(self)
        }
    }
}
