//! Bod2D for Rust
//!
//! You won't find a lot of information about Box2D itself here,
//! look at [the official website](http://box2d.org/) instead.
//!
//! # World
//!
//! ```
//! use wrapped2d::b2;
//! use wrapped2d::user_data::NoUserData;
//!
//! let gravity = b2::Vec2 { x: 0., y: -10. };
//! let world = b2::World::<NoUserData>::new(&gravity);
//! ```
//!
//! # Handles
//!
//! Bodies, fixtures and joints are accessed through handles and
//! their borrowing is dynamically checked by `RefCell`s.
//!
//! ```
//! # use wrapped2d::b2;
//! # use wrapped2d::user_data::NoUserData;
//! # let gravity = b2::Vec2 { x: 0., y: -10. };
//! # let mut world = b2::World::<NoUserData>::new(&gravity);
//! let mut def = b2::BodyDef {
//!     body_type: b2::BodyType::Dynamic,
//!     position: b2::Vec2 { x: 10., y: 10. },
//!     .. b2::BodyDef::new()
//! };
//!
//! let handle = world.create_body(&def);
//! let mut body = world.body_mut(handle);
//!
//! let shape = b2::PolygonShape::new_box(0.5, 0.5);
//!
//! let handle = body.create_fast_fixture(&shape, 2.);
//! let fixture = body.fixture(handle);
//! ```
//!
//! # User Data
//!
//! You can provide a unit struct to specify the user data types that will be used for bodies,
//! joints and fixtures. For example:
//!
//! ```
//! use wrapped2d::b2;
//! use wrapped2d::user_data::*;
//!
//! pub type ObjectId = u64;
//!
//! pub struct CustomUserData;
//! impl UserDataTypes for CustomUserData {
//!     type BodyData = Option<ObjectId>;
//!     type JointData = ();
//!     type FixtureData = FixtureKind;
//! }
//!
//! pub type World = b2::World<CustomUserData>;
//!
//! pub enum FixtureKind {
//!     Wood,
//!     Metal,
//! }
//!
//! fn main() {
//!     let mut world = World::new(&b2::Vec2 { x: 0., y: -10. });
//!     
//!     let def = b2::BodyDef {
//!         body_type: b2::BodyType::Dynamic,
//!         position: b2::Vec2 { x: 0., y: -15. },
//!         ..b2::BodyDef::new()
//!     };
//!
//!     let h1 = world.create_body(&def); // will use `Default` user data (`None` here)
//!     let h2 = world.create_body_with(&def, Some(2)); // specifying user data for the body
//!
//!     let user_data = world.body(h2).user_data(); // access the body user data
//! }
//! ```

// disable me after fixing errors
#![allow(non_snake_case)]
#![allow(unused)]

extern crate box2d3;
extern crate libc;
extern crate vec_map;
#[macro_use]
extern crate bitflags;
#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "cgmath")]
extern crate cgmath;
#[cfg(feature = "nalgebra")]
extern crate nalgebra;
#[cfg(feature = "serialize")]
extern crate serde;

mod ffi;
#[doc(hidden)]
#[macro_use]
pub mod wrap;
#[doc(hidden)]
pub mod handle;

pub mod collision;
pub mod common;
pub mod dynamics;
#[cfg(feature = "serialize")]
pub mod serialize;
pub mod user_data;

pub mod b2 {
    pub use collision::shapes::{
        ChainShape, CircleShape, EdgeShape, MassData, PolygonShape, Shape, ShapeType, UnknownShape,
    };
    pub use collision::{
        distance, get_point_states, test_overlap, time_of_impact, ContactFeature,
        ContactFeatureType, ContactId, Manifold, ManifoldPoint, ManifoldType, PointState,
        RayCastInput, RayCastOutput, WorldManifold, AABB,
    };
    pub use common::math::{cross_sv, cross_vs, cross_vv};
    pub use common::math::{Rot, Sweep, Transform, Vec2};
    pub use common::settings::{
        ANGULAR_SLOP, LINEAR_SLOP, MAX_MANIFOLD_POINTS, MAX_POLYGON_VERTICES, PI, POLYGON_RADIUS,
    };
    pub use common::{Color, Draw, DrawFlags};
    pub use dynamics::body::{BodyDef, BodyType, FixtureHandle, MetaBody};
    pub use dynamics::fixture::{Filter, Fixture, FixtureDef, MetaFixture};
    pub use dynamics::world::{BodyHandle, JointHandle, World};
    pub use dynamics::Profile;
}
