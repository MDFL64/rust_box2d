pub mod math;
use std::marker::PhantomData;

pub mod common;
pub mod world;
pub mod body;
pub mod shapes;
pub mod debug_draw;

/// Used to mark our handles as !Send and !Sync for some attempt at thread safety.
type PhantomNoSend = PhantomData<*mut ()>;

pub use world::{World, WorldDef};
pub use body::{Body, BodyDef};
pub use shapes::{Shape, ShapeDef};
