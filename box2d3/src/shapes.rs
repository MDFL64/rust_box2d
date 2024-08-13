use crate::{math::Vec2, common::Filter};

#[derive(Debug)]
#[repr(C)]
pub struct ShapeDef {
    /// Use this to store application specific shape data.
    pub user_data: *const std::ffi::c_void,

    /// The Coulomb (dry) friction coefficient, usually in the range [0,1].
    pub friction: f32,

    /// The restitution (bounce) usually in the range [0,1].
    pub restitution: f32,

    /// The density, usually in kg/m^2.
    pub density: f32,

    /// Collision filtering data.
    pub filter: Filter,

    /// Custom debug draw color.
    pub custom_color: u32,

    /// A sensor shape generates overlap events but never generates a collision response.
    pub is_sensor: bool,

    /// Enable sensor events for this shape. Only applies to kinematic and dynamic bodies. Ignored for sensors.
    pub enable_sensor_events: bool,

    /// Enable contact events for this shape. Only applies to kinematic and dynamic bodies. Ignored for sensors.
    pub enable_contact_events: bool,

    /// Enable hit events for this shape. Only applies to kinematic and dynamic bodies. Ignored for sensors.
    pub enable_hit_events: bool,

    /// Enable pre-solve contact events for this shape. Only applies to dynamic bodies. These are expensive
    ///	and must be carefully handled due to threading. Ignored for sensors.
    pub enable_pre_solve_events: bool,

    /// Normally shapes on static bodies don't invoke contact creation when they are added to the world. This overrides
    ///	that behavior and causes contact creation. This significantly slows down static body creation which can be important
    ///	when there are many static shapes.
    pub force_contact_creation: bool,

    /// Used internally to detect a valid definition. DO NOT SET.
    _cookie: u32
}

const MAX_POLYGON_VERTICES: usize = 8;

///	DO NOT fill this out manually. Use a constructor.
#[derive(Debug)]
#[repr(C)]
pub struct Polygon {
    /// The polygon vertices
	pub vertices: [Vec2; MAX_POLYGON_VERTICES],

	/// The outward normal vectors of the polygon sides
	pub normals: [Vec2; MAX_POLYGON_VERTICES],

	/// The centroid of the polygon
	pub centroid: Vec2,

	/// The external radius for rounded polygons
	pub radius: f32,

	/// The number of polygon vertices
	pub vertex_count: u32
}

impl Default for ShapeDef {
    fn default() -> Self {
        unsafe {
            b2DefaultShapeDef()
        }
    }
}

impl Polygon {
    pub fn new_box(hx: f32, hy: f32) -> Self {
        unsafe {
            b2MakeBox(hx,hy)
        }
    }
}

extern "C" {
    fn b2DefaultShapeDef() -> ShapeDef;

    fn b2MakeBox(hx: f32, hy: f32) -> Polygon;
}
