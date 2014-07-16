use libc::c_void;
use math::{Vec2, Transform};
use common::{Color, DrawFlags};
use dynamics::{
    BodyDef, BodyType, FixtureDef,
    JointDefBase, JointType, DistanceJointDef, FrictionJointDef,
    MotorJointDef, PrismaticJointDef, PulleyJointDef, RevoluteJointDef,
    WeldJointDef, WheelJointDef,
    Filter, Profile, Manifold, ContactImpulse
};
use dynamics::joints::LimitState;
use collision::{
    ShapeType, MassData, AABB, RayCastInput, RayCastOutput
};

pub struct DestructionListener;
pub struct CDestructionListener;
pub struct ContactFilter;
pub struct CContactFilter;
pub struct ContactListener;
pub struct CContactListener;
pub struct QueryCallback;
pub struct CQueryCallback;
pub struct RayCastCallback;
pub struct CRayCastCallback;
pub struct Draw;
pub struct CDraw;

pub struct Contact;
pub struct ContactManager;
pub struct JointEdge;
pub struct ContactEdge;

pub struct World;
pub struct Body;
pub struct Fixture;
pub struct Shape;
pub struct ChainShape;
pub struct EdgeShape;
pub struct CircleShape;
pub struct PolygonShape;
pub struct Joint;
pub struct DistanceJoint;
pub struct FrictionJoint;
pub struct GearJoint;
pub struct MotorJoint;
pub struct MouseJoint;
pub struct PrismaticJoint;
pub struct PulleyJoint;
pub struct RevoluteJoint;
pub struct RopeJoint;
pub struct WeldJoint;
pub struct WheelJoint;

pub type Any = *mut c_void;

#[link(name = "box2d_frontend", kind = "static")]
extern {
    pub fn World_new(gravity: *const Vec2) -> *mut World;
    pub fn World_drop(slf: *mut World);
    pub fn World_set_destruction_listener(slf: *mut World,
                                          dl: *mut DestructionListener);
    pub fn World_set_contact_filter(slf: *mut World, cf: *mut ContactFilter);
    pub fn World_set_contact_listener(slf: *mut World, cl: *mut ContactListener);
    pub fn World_set_debug_draw(slf: *mut World, dd: *mut Draw);
    pub fn World_create_body(slf: *mut World, def: *const BodyDef) -> *mut Body;
    pub fn World_destroy_body(slf: *mut World, body: *mut Body);
    pub fn World_create_joint(slf: *mut World, def: *const JointDefBase
                              ) -> *mut Joint;
    pub fn World_destroy_joint(slf: *mut World, joint: *mut Joint);
    pub fn World_step(slf: *mut World,
                      time_step: f32,
                      velocity_iterations: i32,
                      position_iterations: i32);
    pub fn World_clear_forces(slf: *mut World);
    pub fn World_draw_debug_data(slf: *mut World);
    pub fn World_query_aabb(slf: *const World, qc: *mut QueryCallback,
                            aabb: *const AABB);
    pub fn World_ray_cast(slf: *const World, rcc: *mut RayCastCallback,
                          p1: *const Vec2, p2: *const Vec2);
    pub fn World_get_body_list(slf: *mut World) -> *mut Body;
    pub fn World_get_body_list_const(slf: *const World) -> *const Body;
    pub fn World_get_joint_list(slf: *mut World) -> *mut Joint;
    pub fn World_get_joint_list_const(slf: *const World) -> *const Joint;
    pub fn World_get_contact_list(slf: *mut World) -> *mut Contact;
    pub fn World_get_contact_list_const(slf: *const World) -> *const Contact;
    pub fn World_set_allow_sleeping(slf: *mut World, flag: bool);
    pub fn World_get_allow_sleeping(slf: *const World) -> bool;
    pub fn World_set_warm_starting(slf: *mut World, flag: bool);
    pub fn World_get_warm_starting(slf: *const World) -> bool;
    pub fn World_set_continuous_physics(slf: *mut World, flag: bool);
    pub fn World_get_continuous_physics(slf: *const World) -> bool;
    pub fn World_set_sub_stepping(slf: *mut World, flag: bool);
    pub fn World_get_sub_stepping(slf: *const World) -> bool;
    pub fn World_get_proxy_count(slf: *const World) -> i32;
    pub fn World_get_body_count(slf: *const World) -> i32;
    pub fn World_get_joint_count(slf: *const World) -> i32;
    pub fn World_get_contact_count(slf: *const World) -> i32;
    pub fn World_get_tree_height(slf: *const World) -> i32;
    pub fn World_get_tree_balance(slf: *const World) -> i32;
    pub fn World_get_tree_quality(slf: *const World)-> f32;
    pub fn World_set_gravity(slf: *mut World, gravity: *const Vec2);
    pub fn World_get_gravity(slf: *const World) -> Vec2;
    pub fn World_is_locked(slf: *const World) -> bool;
    pub fn World_set_auto_clear_forces(slf: *mut World, flag: bool);
    pub fn World_get_auto_clear_forces(slf: *const World) -> bool;
    pub fn World_shift_origin(slf: *mut World, origin: *const Vec2);
    //pub fn World_get_contact_manager(slf: *const World) -> *const ContactManager;
    pub fn World_get_profile(slf: *const World) -> *const Profile;
    pub fn World_dump(slf: *mut World);
    
    pub fn CDestructionListener_new(
        object: Any,
        goodbye_joint: unsafe extern fn(Any, *mut Joint),
        goodbye_fixture: unsafe extern fn(Any, *mut Fixture),
        ) -> *mut CDestructionListener;
    pub fn CDestructionListener_as_base(slf: *mut CDestructionListener
                                        ) -> *mut DestructionListener;
    pub fn CDestructionListener_drop(slf: *mut CDestructionListener);
    pub fn CContactFilter_new(
        object: Any,
        should_collide: unsafe extern fn(Any, *mut Fixture, *mut Fixture
                                         ) -> bool
        ) -> *mut CContactFilter;
    pub fn CContactFilter_as_base(slf: *mut CContactFilter
                                  ) -> *mut ContactFilter;
    pub fn CContactFilter_drop(slf: *mut CContactFilter);
    pub fn CContactListener_new(
        object: Any,
        begin_contact: unsafe extern fn(Any, *mut Contact),
        end_contact: unsafe extern fn(Any, *mut Contact),
        pre_solve: unsafe extern fn(Any, *mut Contact, *const Manifold),
        post_solve: unsafe extern fn(Any, *mut Contact, *const ContactImpulse),
        ) -> *mut CContactListener;
    pub fn CContactListener_as_base(slf: *mut CContactListener
                                    ) -> *mut ContactListener;
    pub fn CContactListener_drop(slf: *mut CContactListener);
    pub fn CQueryCallback_new(
        object: Any,
        report_fixture: unsafe extern fn(Any, *mut Fixture) -> bool,
        ) -> *mut CQueryCallback;
    pub fn CQueryCallback_as_base(slf: *mut CQueryCallback
                                  ) -> *mut QueryCallback;
    pub fn CQueryCallback_drop(slf: *mut CQueryCallback);
    pub fn CRayCastCallback_new(
        object: Any,
        hit_fixture: unsafe extern fn(Any, *mut Fixture, *const Vec2,
                                      *const Vec2, f32) -> f32,
        ) -> *mut CRayCastCallback;
    pub fn CRayCastCallback_as_base(slf: *mut CRayCastCallback
                                    ) -> *mut RayCastCallback;
    pub fn CRayCastCallback_drop(slf: *mut CRayCastCallback);
    
    pub fn CDraw_new(
        object: Any,
        draw_polygon: unsafe extern fn(Any, *const Vec2, i32, *const Color),
        draw_solid_polygon: unsafe extern fn(Any, *const Vec2, i32, *const Color),
        draw_circle: unsafe extern fn(Any, *const Vec2, f32, *const Color),
        draw_solid_circle: unsafe extern fn(Any, *const Vec2, f32, *const Vec2,
                                            *const Color),
        draw_segment: unsafe extern fn(Any, *const Vec2, *const Vec2,
                                       *const Color),
        draw_transform: unsafe extern fn(Any, *const Transform),
        ) -> *mut CDraw;
    pub fn CDraw_as_base(slf: *mut CDraw) -> *mut Draw;
    pub fn CDraw_drop(slf: *mut CDraw);
    pub fn CDraw_set_flags(slf: *mut CDraw, flags: DrawFlags);
    pub fn CDraw_get_flags(slf: *const CDraw) -> DrawFlags;
    pub fn CDraw_append_flags(slf: *mut CDraw, flags: DrawFlags);
    pub fn CDraw_clear_flags(slf: *mut CDraw, flags: DrawFlags);
    
    pub fn Body_create_fixture(slf: *mut Body, def: *const FixtureDef
                               ) -> *mut Fixture;
    pub fn Body_create_fast_fixture(slf: *mut Body, shape: *const Shape,
                                    density: f32) -> *mut Fixture;
    pub fn Body_destroy_fixture(slf: *mut Body, fixture: *mut Fixture);
    pub fn Body_set_transform(slf: *mut Body, pos: *const Vec2, angle: f32);
    pub fn Body_get_transform(slf: *const Body) -> *const Transform;
    pub fn Body_get_position(slf: *const Body) -> *const Vec2;
    pub fn Body_get_angle(slf: *const Body) -> f32;
    pub fn Body_get_world_center(slf: *const Body) -> *const Vec2;
    pub fn Body_get_local_center(slf: *const Body) -> *const Vec2;
    pub fn Body_set_linear_velocity(slf: *mut Body, v: *const Vec2);
    pub fn Body_get_linear_velocity(slf: *const Body) -> *const Vec2;
    pub fn Body_set_angular_velocity(slf: *mut Body, omega: f32);
    pub fn Body_get_angular_velocity(slf: *const Body) -> f32;
    pub fn Body_apply_force(slf: *mut Body, force: *const Vec2,
                            point: *const Vec2, wake: bool);
    pub fn Body_apply_force_to_center(slf: *mut Body, force: *const Vec2,
                                      wake: bool);
    pub fn Body_apply_torque(slf: *mut Body, torque: f32, wake: bool);
    pub fn Body_apply_linear_impulse(slf: *mut Body, impulse: *const Vec2,
                                     point: *const Vec2, wake: bool);
    pub fn Body_apply_angular_impulse(slf: *mut Body, impulse: f32,
                                      wake: bool);
    pub fn Body_get_mass(slf: *const Body) -> f32;
    pub fn Body_get_inertia(slf: *const Body) -> f32;
    pub fn Body_get_mass_data(slf: *const Body, data: *mut MassData);
    pub fn Body_set_mass_data(slf: *mut Body, data: *const MassData);
    pub fn Body_reset_mass_data(slf: *mut Body);
    pub fn Body_get_world_point(slf: *const Body, local: *const Vec2) -> Vec2;
    pub fn Body_get_world_vector(slf: *const Body, local: *const Vec2) -> Vec2;
    pub fn Body_get_local_point(slf: *const Body, world: *const Vec2) -> Vec2;
    pub fn Body_get_local_vector(slf: *const Body, world: *const Vec2) -> Vec2;
    pub fn Body_get_linear_velocity_from_world_point(slf: *const Body,
                                                     point: *const Vec2
                                                     ) -> Vec2;
    pub fn Body_get_linear_velocity_from_local_point(slf: *const Body,
                                                     point: *const Vec2
                                                     ) -> Vec2;
    pub fn Body_get_linear_damping(slf: *const Body) -> f32;
    pub fn Body_set_linear_damping(slf: *mut Body, damping: f32);
    pub fn Body_get_angular_damping(slf: *const Body) -> f32;
    pub fn Body_set_angular_damping(slf: *mut Body, damping: f32);
    pub fn Body_get_gravity_scale(slf: *const Body) -> f32;
    pub fn Body_set_gravity_scale(slf: *mut Body, scale: f32);
    pub fn Body_set_type(slf: *mut Body, typ: BodyType);
    pub fn Body_get_type(slf: *const Body) -> BodyType;
    pub fn Body_set_bullet(slf: *mut Body, flag: bool);
    pub fn Body_is_bullet(slf: *const Body) -> bool;
    pub fn Body_set_sleeping_allowed(slf: *mut Body, flag: bool);
    pub fn Body_is_sleeping_allowed(slf: *const Body) -> bool;
    pub fn Body_set_awake(slf: *mut Body, flag: bool);
    pub fn Body_is_awake(slf: *const Body) -> bool;
    pub fn Body_set_active(slf: *mut Body, flag: bool);
    pub fn Body_is_active(slf: *const Body) -> bool;
    pub fn Body_set_fixed_rotation(slf: *mut Body, flag: bool);
    pub fn Body_is_fixed_rotation(slf: *const Body) -> bool; 
    pub fn Body_get_fixture_list(slf: *mut Body) -> *mut Fixture;
    pub fn Body_get_fixture_list_const(slf: *const Body) -> *const Fixture;
    pub fn Body_get_joint_list(slf: *mut Body) -> *mut JointEdge;
    pub fn Body_get_joint_list_const(slf: *const Body) -> *const JointEdge;
    pub fn Body_get_contact_list(slf: *mut Body) -> *mut ContactEdge;
    pub fn Body_get_contact_list_const(slf: *const Body) -> *const ContactEdge;
    pub fn Body_get_next(slf: *mut Body) -> *mut Body;
    pub fn Body_get_next_const(slf: *const Body) -> *const Body;
    pub fn Body_get_user_data(slf: *const Body) -> Any;
    pub fn Body_set_user_data(slf: *mut Body, data: Any);
    pub fn Body_get_world(slf: *mut Body) -> *mut World;
    pub fn Body_get_world_const(slf: *const Body) -> *const World;
    pub fn Body_dump(slf: *mut Body);                 
    
    pub fn Fixture_get_type(slf: *const Fixture) -> ShapeType;
    pub fn Fixture_get_shape(slf: *mut Fixture) -> *mut Shape;
    pub fn Fixture_get_shape_const(slf: *const Fixture) -> *const Shape;
    pub fn Fixture_set_sensor(slf: *mut Fixture, flag: bool);
    pub fn Fixture_is_sensor(slf: *const Fixture) -> bool;
    pub fn Fixture_set_filter_data(slf: *mut Fixture, filter: *const Filter);
    pub fn Fixture_get_filter_data(slf: *const Fixture) -> *const Filter;
    pub fn Fixture_refilter(slf: *mut Fixture);
    pub fn Fixture_get_body(slf: *mut Fixture) -> *mut Body;
    pub fn Fixture_get_body_const(slf: *const Fixture) -> *const Body;
    pub fn Fixture_get_next(slf: *mut Fixture) -> *mut Fixture;
    pub fn Fixture_get_next_const(slf: *const Fixture) -> *const Fixture;
    pub fn Fixture_get_user_data(slf: *const Fixture) -> Any;
    pub fn Fixture_set_user_data(slf: *mut Fixture, data: Any);
    pub fn Fixture_test_point(slf: *const Fixture, p: *const Vec2) -> bool;
    pub fn Fixture_ray_cast(slf: *const Fixture,
                            output: *mut RayCastOutput,
                            input: *const RayCastInput,
                            child_index: i32) -> bool;
    pub fn Fixture_get_mass_data(slf: *const Fixture, data: *mut MassData);
    pub fn Fixture_set_density(slf: *mut Fixture, density: f32);
    pub fn Fixture_get_density(slf: *const Fixture) -> f32;
    pub fn Fixture_get_friction(slf: *const Fixture) -> f32;
    pub fn Fixture_set_friction(slf: *mut Fixture, friction: f32);
    pub fn Fixture_get_restitution(slf: *const Fixture) -> f32;
    pub fn Fixture_set_restitution(slf: *mut Fixture, restitution: f32);
    pub fn Fixture_get_aabb(slf: *const Fixture, child_id: i32) -> *const AABB;
    pub fn Fixture_dump(slf: *mut Fixture, body_id: i32);
    
    pub fn Shape_drop_virtual(slf: *mut Shape);
    //pub fn Shape_clone_virtual(slf: *const Shape,
    //                           alloc: *mut BlockAllocator) -> *mut Shape;
    pub fn Shape_get_type(slf: *const Shape) -> ShapeType;
    pub fn Shape_get_child_count_virtual(slf: *const Shape) -> i32;
    pub fn Shape_test_point_virtual(slf: *const Shape,
                                    xf: *const Transform,
                                    p: *const Vec2) -> bool;
    pub fn Shape_ray_cast_virtual(slf: *const Shape,
                                  output: *mut RayCastOutput,
                                  input: *const RayCastInput,
                                  transform: *const Transform,
                                  child_index: i32) -> bool;
    pub fn Shape_compute_aabb_virtual(slf: *const Shape,
                                      aabb: *mut AABB,
                                      xf: *const Transform,
                                      child_id: i32);
    pub fn Shape_compute_mass_virtual(slf: *const Shape,
                                      data: *mut MassData,
                                      density: f32);
                                      
    pub fn ChainShape_new() -> *mut ChainShape;
    pub fn ChainShape_drop(slf: *mut ChainShape);
    pub fn ChainShape_as_shape(slf: *mut ChainShape) -> *mut Shape;
    pub fn Shape_as_chain_shape(slf: *mut Shape) -> *mut ChainShape;
    pub fn ChainShape_clear(slf: *mut ChainShape);
    pub fn ChainShape_create_loop(slf: *mut ChainShape,
                                  vertices: *const Vec2,
                                  count: i32);
    pub fn ChainShape_create_chain(slf: *mut ChainShape,
                                   vertices: *const Vec2,
                                   count: i32);
    pub fn ChainShape_set_prev_vertex(slf: *mut ChainShape, vertex: *const Vec2);
    pub fn ChainShape_set_next_vertex(slf: *mut ChainShape, vertex: *const Vec2);
    pub fn ChainShape_get_child_edge(slf: *const ChainShape,
                                     edge: *mut EdgeShape,
                                     index: i32);
                                     
    pub fn EdgeShape_new() -> *mut EdgeShape;
    pub fn EdgeShape_drop(slf: *mut EdgeShape);
    pub fn EdgeShape_as_shape(slf: *mut EdgeShape) -> *mut Shape;
    pub fn Shape_as_edge_shape(slf: *mut Shape) -> *mut EdgeShape;
    pub fn EdgeShape_set(slf: *mut EdgeShape, v1: *const Vec2, v2: *const Vec2);

    pub fn CircleShape_new() -> *mut CircleShape;
    pub fn CircleShape_drop(slf: *mut CircleShape);
    pub fn CircleShape_as_shape(slf: *mut CircleShape) -> *mut Shape;
    pub fn Shape_as_circle_shape(slf: *mut Shape) -> *mut CircleShape;
    pub fn CircleShape_get_support(slf: *const CircleShape, d: *const Vec2) -> i32;
    pub fn CircleShape_get_support_vertex(slf: *const CircleShape,
                                          d: *const Vec2) -> *const Vec2;
    pub fn CircleShape_get_vertex_count(slf: *const CircleShape) -> i32;
    pub fn CircleShape_get_vertex(slf: *const CircleShape, index: i32
                                 ) -> *const Vec2;
    
    pub fn PolygonShape_new() -> *mut PolygonShape;
    pub fn PolygonShape_drop(slf: *mut PolygonShape);
    pub fn PolygonShape_as_shape(slf: *mut PolygonShape) -> *mut Shape;
    pub fn Shape_as_polygon_shape(slf: *mut Shape) -> *mut PolygonShape;
    pub fn PolygonShape_set(slf: *mut PolygonShape, points: *const Vec2, count: i32);
    pub fn PolygonShape_set_as_box(slf: *mut PolygonShape, hw: f32, hh: f32);
    pub fn PolygonShape_set_as_oriented_box(slf: *mut PolygonShape,
                                            hw: f32, hh: f32,
                                            center: *const Vec2,
                                            angle: f32);
    pub fn PolygonShape_get_vertex_count(slf: *const PolygonShape) -> i32;
    pub fn PolygonShape_get_vertex(slf: *const PolygonShape, index: i32) -> *const Vec2;
    pub fn PolygonShape_validate(slf: *const PolygonShape) -> bool;
    
    pub fn Joint_get_type(slf: *const Joint) -> JointType;
    pub fn Joint_get_body_a(slf: *mut Joint) -> *mut Body;
    pub fn Joint_get_body_b(slf: *mut Joint) -> *mut Body;
    pub fn Joint_get_anchor_a_virtual(slf: *const Joint) -> Vec2;
    pub fn Joint_get_anchor_b_virtual(slf: *const Joint) -> Vec2;
    pub fn Joint_get_reaction_force_virtual(slf: *const Joint) -> Vec2;
    pub fn Joint_get_reaction_torque_virtual(slf: *const Joint) -> f32;
    pub fn Joint_get_next(slf: *mut Joint) -> *mut Joint;
    pub fn Joint_get_next_const(slf: *const Joint) -> *const Joint;
    pub fn Joint_get_user_data(slf: *const Joint) -> Any;
    pub fn Joint_set_user_data(slf: *mut Joint, data: Any);
    pub fn Joint_is_active(slf: *const Joint) -> bool;
    pub fn Joint_dump_virtual(slf: *mut Joint);
    pub fn Joint_shift_origin_virtual(slf: *mut Joint, origin: *const Vec2);
    
    pub fn DistanceJointDef_initialize(slf: *mut DistanceJointDef,
                                       body_a: *mut Body,
                                       body_b: *mut Body,
                                       anchor_a: *const Vec2,
                                       anchor_b: *const Vec2);
    pub fn DistanceJoint_as_joint(slf: *mut DistanceJoint) -> *mut Joint;
    pub fn Joint_as_distance_joint(slf: *mut Joint) -> *mut DistanceJoint;
    pub fn DistanceJoint_get_local_anchor_a(slf: *const DistanceJoint) -> *const Vec2;
    pub fn DistanceJoint_get_local_anchor_b(slf: *const DistanceJoint) -> *const Vec2;
    pub fn DistanceJoint_set_length(slf: *mut DistanceJoint, length: f32);
    pub fn DistanceJoint_get_length(slf: *const DistanceJoint) -> f32;
    pub fn DistanceJoint_set_frequency(slf: *mut DistanceJoint, hz: f32);
    pub fn DistanceJoint_get_frequency(slf: *const DistanceJoint) -> f32;
    pub fn DistanceJoint_set_damping_ratio(slf: *mut DistanceJoint, ratio: f32);
    pub fn DistanceJoint_get_damping_ratio(slf: *const DistanceJoint) -> f32;
    
    pub fn FrictionJointDef_initialize(slf: *mut FrictionJointDef,
                                       body_a: *mut Body,
                                       body_b: *mut Body,
                                       anchor: *const Vec2);
    pub fn FrictionJoint_as_joint(slf: *mut FrictionJoint) -> *mut Joint;
    pub fn Joint_as_friction_joint(slf: *mut Joint) -> *mut FrictionJoint;
    pub fn FrictionJoint_get_local_anchor_a(slf: *const FrictionJoint) -> *const Vec2;
    pub fn FrictionJoint_get_local_anchor_b(slf: *const FrictionJoint) -> *const Vec2;
    pub fn FrictionJoint_set_max_force(slf: *mut FrictionJoint, force: f32);
    pub fn FrictionJoint_get_max_force(slf: *const FrictionJoint) -> f32;
    pub fn FrictionJoint_set_max_torque(slf: *mut FrictionJoint, torque: f32);
    pub fn FrictionJoint_get_max_torque(slf: *const FrictionJoint) -> f32;
    
    pub fn GearJoint_as_joint(slf: *mut GearJoint) -> *mut Joint;
    pub fn Joint_as_gear_joint(slf: *mut Joint) -> *mut GearJoint;
    pub fn GearJoint_get_joint_1(slf: *mut GearJoint) -> *const Joint;
    pub fn GearJoint_get_joint_2(slf: *mut GearJoint) -> *const Joint;
    pub fn GearJoint_set_ratio(slf: *mut GearJoint, ratio: f32);
    pub fn GearJoint_get_ratio(slf: *const GearJoint) -> f32;
    
    pub fn MotorJointDef_initialize(slf: *mut MotorJointDef,
                                    body_a: *mut Body,
                                    body_b: *mut Body);
    pub fn MotorJoint_as_joint(slf: *mut MotorJoint) -> *mut Joint;
    pub fn Joint_as_motor_joint(slf: *mut Joint) -> *mut MotorJoint;
    pub fn MotorJoint_set_linear_offset(slf: *mut MotorJoint, offset: *const Vec2);
    pub fn MotorJoint_get_linear_offset(slf: *const MotorJoint) -> *const Vec2;
    pub fn MotorJoint_set_angular_offset(slf: *mut MotorJoint, offset: f32);
    pub fn MotorJoint_get_angular_offset(slf: *const MotorJoint) -> f32;
    pub fn MotorJoint_set_max_force(slf: *mut MotorJoint, force: f32);
    pub fn MotorJoint_get_max_force(slf: *const MotorJoint) -> f32;
    pub fn MotorJoint_set_max_torque(slf: *mut MotorJoint, torque: f32);
    pub fn MotorJoint_get_max_torque(slf: *const MotorJoint) -> f32;
    pub fn MotorJoint_set_correction_factor(slf: *mut MotorJoint, factor: f32);
    pub fn MotorJoint_get_correction_factor(slf: *const MotorJoint) -> f32;
    
    pub fn MouseJoint_as_joint(slf: *mut MouseJoint) -> *mut Joint;
    pub fn Joint_as_mouse_joint(slf: *mut Joint) -> *mut MouseJoint;
    pub fn MouseJoint_set_target(slf: *mut MouseJoint, target: *const Vec2);
    pub fn MouseJoint_get_target(slf: *const MouseJoint) -> *const Vec2;
    pub fn MouseJoint_set_max_force(slf: *mut MouseJoint, force: f32);
    pub fn MouseJoint_get_max_force(slf: *const MouseJoint) -> f32;
    pub fn MouseJoint_set_frequency(slf: *mut MouseJoint, hz: f32);
    pub fn MouseJoint_get_frequency(slf: *const MouseJoint) -> f32;
    pub fn MouseJoint_set_damping_ratio(slf: *mut MouseJoint, ratio: f32);
    pub fn MouseJoint_get_damping_ratio(slf: *const MouseJoint) -> f32; 
    
    pub fn PrismaticJointDef_initialize(slf: *mut PrismaticJointDef,
                                        body_a: *mut Body,
                                        body_b: *mut Body,
                                        anchor: *const Vec2,
                                        axis: *const Vec2);
    pub fn PrismaticJoint_as_joint(slf: *mut PrismaticJoint) -> *mut Joint;
    pub fn Joint_as_prismatic_joint(slf: *mut Joint) -> *mut PrismaticJoint;
    pub fn PrismaticJoint_get_local_anchor_a(slf: *const PrismaticJoint) -> *const Vec2;
    pub fn PrismaticJoint_get_local_anchor_b(slf: *const PrismaticJoint) -> *const Vec2;
    pub fn PrismaticJoint_get_local_axis_a(slf: *const PrismaticJoint) -> *const Vec2;
    pub fn PrismaticJoint_get_reference_angle(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_get_joint_translation(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_get_joint_speed(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_is_limit_enabled(slf: *const PrismaticJoint) -> bool;
    pub fn PrismaticJoint_enable_limit(slf: *mut PrismaticJoint, flag: bool);
    pub fn PrismaticJoint_get_lower_limit(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_get_upper_limit(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_set_limits(slf: *mut PrismaticJoint,
                                     lower: f32, upper: f32);
    pub fn PrismaticJoint_is_motor_enabled(slf: *const PrismaticJoint) -> bool;
    pub fn PrismaticJoint_enable_motor(slf: *mut PrismaticJoint, flag: bool);
    pub fn PrismaticJoint_set_motor_speed(slf: *mut PrismaticJoint, speed: f32);
    pub fn PrismaticJoint_get_motor_speed(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_set_max_motor_force(slf: *mut PrismaticJoint,
                                              force: f32);
    pub fn PrismaticJoint_get_max_motor_force(slf: *const PrismaticJoint) -> f32;
    pub fn PrismaticJoint_get_motor_force(slf: *const PrismaticJoint,
                                          inv_dt: f32) -> f32;
                                         
    pub fn PulleyJointDef_initialize(slf: *mut PulleyJointDef,
                                     body_a: *mut Body,
                                     body_b: *mut Body,
                                     ground_anchor_a: *const Vec2,
                                     ground_anchor_b: *const Vec2,
                                     anchor_a: *const Vec2,
                                     anchor_b: *const Vec2,
                                     ratio: f32);
    pub fn PulleyJoint_as_joint(slf: *mut PulleyJoint) -> *mut Joint;
    pub fn Joint_as_pulley_joint(slf: *mut Joint) -> *mut PulleyJoint;
    pub fn PulleyJoint_get_ground_anchor_a(slf: *const PulleyJoint) -> Vec2;
    pub fn PulleyJoint_get_ground_anchor_b(slf: *const PulleyJoint) -> Vec2;
    pub fn PulleyJoint_get_length_a(slf: *const PulleyJoint) -> f32;
    pub fn PulleyJoint_get_length_b(slf: *const PulleyJoint) -> f32;
    pub fn PulleyJoint_get_ratio(slf: *const PulleyJoint) -> f32;
    pub fn PulleyJoint_get_current_length_a(slf: *const PulleyJoint) -> f32;
    pub fn PulleyJoint_get_current_length_b(slf: *const PulleyJoint) -> f32;
    
    pub fn RevoluteJointDef_initialize(slf: *mut RevoluteJointDef,
                                       body_a: *mut Body,
                                       body_b: *mut Body,
                                       anchor: *const Vec2);
    pub fn RevoluteJoint_as_joint(slf: *mut RevoluteJoint) -> *mut Joint;
    pub fn Joint_as_revolute_joint(slf: *mut Joint) -> *mut RevoluteJoint;
    pub fn RevoluteJoint_get_local_anchor_a(slf: *const RevoluteJoint) -> *const Vec2;
    pub fn RevoluteJoint_get_local_anchor_b(slf: *const RevoluteJoint) -> *const Vec2;
    pub fn RevoluteJoint_get_reference_angle(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_get_joint_angle(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_get_joint_speed(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_is_limit_enabled(slf: *const RevoluteJoint) -> bool;
    pub fn RevoluteJoint_enable_limit(slf: *mut RevoluteJoint, flag: bool);
    pub fn RevoluteJoint_get_lower_limit(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_get_upper_limit(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_set_limits(slf: *mut RevoluteJoint,
                                    lower: f32, upper: f32);
    pub fn RevoluteJoint_is_motor_enabled(slf: *const RevoluteJoint) -> bool;
    pub fn RevoluteJoint_enable_motor(slf: *mut RevoluteJoint, flag: bool);
    pub fn RevoluteJoint_set_motor_speed(slf: *mut RevoluteJoint, speed: f32);
    pub fn RevoluteJoint_get_motor_speed(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_set_max_motor_torque(slf: *mut RevoluteJoint,
                                              torque: f32);
    pub fn RevoluteJoint_get_max_motor_torque(slf: *const RevoluteJoint) -> f32;
    pub fn RevoluteJoint_get_motor_torque(slf: *const RevoluteJoint) -> f32;
    
    pub fn RopeJoint_as_joint(slf: *mut RopeJoint) -> *mut Joint;
    pub fn Joint_as_rope_joint(slf: *mut Joint) -> *mut RopeJoint;
    pub fn RopeJoint_get_local_anchor_a(slf: *const RopeJoint) -> *const Vec2;
    pub fn RopeJoint_get_local_anchor_b(slf: *const RopeJoint) -> *const Vec2;
    pub fn RopeJoint_set_max_length(slf: *mut RopeJoint, length: f32);
    pub fn RopeJoint_get_max_length(slf: *const RopeJoint) -> f32;
    pub fn RopeJoint_get_limit_state(slf: *const RopeJoint) -> LimitState;
    
    pub fn WeldJointDef_initialize(slf: *mut WeldJointDef,
                                   body_a: *mut Body,
                                   body_b: *mut Body,
                                   anchor: *const Vec2);
    pub fn WeldJoint_as_joint(slf: *mut WeldJoint) -> *mut Joint;
    pub fn Joint_as_weld_joint(slf: *mut Joint) -> *mut WeldJoint;
    pub fn WeldJoint_get_local_anchor_a(slf: *const WeldJoint) -> *const Vec2;
    pub fn WeldJoint_get_local_anchor_b(slf: *const WeldJoint) -> *const Vec2;
    pub fn WeldJoint_get_reference_angle(slf: *const WeldJoint) -> f32;
    pub fn WeldJoint_set_frequency(slf: *mut WeldJoint, frequency: f32);
    pub fn WeldJoint_get_frequency(slf: *const WeldJoint) -> f32;
    pub fn WeldJoint_set_damping_ratio(slf: *mut WeldJoint, ratio: f32);
    pub fn WeldJoint_get_damping_ratio(slf: *const WeldJoint) -> f32;

    pub fn WheelJointDef_initialize(slf: *mut WheelJointDef,
                                    body_a: *mut Body,
                                    body_b: *mut Body,
                                    anchor: *const Vec2,
                                    axis: *const Vec2);
    pub fn WheelJoint_as_joint(slf: *mut WheelJoint) -> *mut Joint;
    pub fn Joint_as_wheel_joint(slf: *mut Joint) -> *mut WheelJoint;
    pub fn WheelJoint_get_local_anchor_a(slf: *const WheelJoint) -> *const Vec2;
    pub fn WheelJoint_get_local_anchor_b(slf: *const WheelJoint) -> *const Vec2;
    pub fn WheelJoint_get_local_axis_a(slf: *const WheelJoint) -> *const Vec2;
    pub fn WheelJoint_get_joint_translation(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_get_joint_speed(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_is_motor_enabled(slf: *const WheelJoint) -> bool;
    pub fn WheelJoint_enable_motor(slf: *mut WheelJoint, flag: bool);
    pub fn WheelJoint_set_motor_speed(slf: *mut WheelJoint, speed: f32);
    pub fn WheelJoint_get_motor_speed(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_set_max_motor_torque(slf: *mut WheelJoint, torque: f32);
    pub fn WheelJoint_get_max_motor_torque(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_get_motor_torque(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_set_spring_frequency(slf: *mut WheelJoint, frequency: f32);
    pub fn WheelJoint_get_spring_frequency(slf: *const WheelJoint) -> f32;
    pub fn WheelJoint_set_spring_damping_ratio(slf: *mut WheelJoint, ratio: f32);
    pub fn WheelJoint_get_spring_damping_ratio(slf: *const WheelJoint) -> f32;
}