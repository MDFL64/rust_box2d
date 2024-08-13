use super::Shape;
use common::math::Vec2;
use wrap::*;

pub struct CircleShape {
    radius: f32,
    offset: Vec2
}

impl CircleShape {
    #[deprecated = "Consider using a different constructor that gives you what you want."]
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            offset: Vec2 { x: 0.0, y: 0.0 }
        }
    }

    pub fn new_with_radius(radius: f32) -> Self {
        assert!(radius > 0.0);
        Self {
            radius,
            offset: Vec2 { x: 0.0, y: 0.0 }
        }
    }

    pub fn new_with(position: Vec2, radius: f32) -> Self {
        assert!(radius > 0.0);
        Self {
            radius,
            offset: position
        }
    }

    pub fn support(&self, dir: &Vec2) -> i32 {
        panic!("circle");
    }

    pub fn support_vertex<'a>(&'a self, dir: &Vec2) -> &'a Vec2 {
        panic!("circle");
    }

    pub fn vertex_count(&self) -> i32 {
        panic!("circle");
    }

    pub fn vertex<'a>(&'a self, index: i32) -> &'a Vec2 {
        panic!("circle");
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        assert!(radius > 0.0);
        self.radius = radius;
    }

    pub fn position(&self) -> Vec2 {
        self.offset
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.offset = pos;
    }
}

impl Shape for CircleShape {}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::ffi::Shape;
    pub use collision::shapes::ffi::{Shape_get_radius, Shape_set_radius};
    use common::math::Vec2;

    pub enum CircleShape {}

    pub fn CircleShape_new() -> *mut CircleShape {
        todo!()
    }
    pub fn CircleShape_drop(slf: *mut CircleShape) {
        todo!()
    }
    pub fn CircleShape_as_shape(slf: *mut CircleShape) -> *mut Shape {
        todo!()
    }
    pub fn Shape_as_circle_shape(slf: *mut Shape) -> *mut CircleShape {
        todo!()
    }
    pub fn CircleShape_get_support(slf: *const CircleShape, d: *const Vec2) -> i32 {
        todo!()
    }
    pub fn CircleShape_get_support_vertex(slf: *const CircleShape, d: *const Vec2) -> *const Vec2 {
        todo!()
    }
    pub fn CircleShape_get_vertex_count(slf: *const CircleShape) -> i32 {
        todo!()
    }
    pub fn CircleShape_get_vertex(slf: *const CircleShape, index: i32) -> *const Vec2 {
        todo!()
    }
    pub fn CircleShape_get_pos(slf: *const CircleShape) -> Vec2 {
        todo!()
    }
    pub fn CircleShape_set_pos(slf: *mut CircleShape, pos: Vec2) {
        todo!()
    }
}
