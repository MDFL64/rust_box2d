use std::mem;
use std::ptr;

use super::Shape;
use common::math::Vec2;
use wrap::*;

pub struct EdgeShape;

impl EdgeShape {
    pub fn new() -> Self {
        panic!("edge shape");
    }

    pub fn new_with(v1: &Vec2, v2: &Vec2) -> Self {
        panic!("edge shape");

    }

    pub fn set(&mut self, v1: &Vec2, v2: &Vec2) {
        panic!("edge shape");
    }

    pub fn v1(&self) -> Vec2 {
        panic!("edge shape");
    }

    pub fn set_v1(&mut self, v1: Vec2) {
        panic!("edge shape");
    }

    pub fn v2(&self) -> Vec2 {
        panic!("edge shape");
    }

    pub fn set_v2(&mut self, v2: Vec2) {
        panic!("edge shape");
    }

    pub fn v0(&self) -> Option<Vec2> {
        panic!("edge shape");
    }

    pub fn set_v0(&mut self, v0: Option<Vec2>) {
        panic!("edge shape");
    }

    pub fn v3(&self) -> Option<Vec2> {
        panic!("edge shape");
    }

    pub fn set_v3(&mut self, v3: Option<Vec2>) {
        panic!("edge shape");
    }
}

impl Shape for EdgeShape {
    fn to_enum(&self) -> super::UnknownShape {
        todo!();
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::ffi::Shape;
    use common::math::Vec2;

    pub enum EdgeShape {}

    pub fn EdgeShape_new() -> *mut EdgeShape {
        todo!()
    }
    pub fn EdgeShape_drop(slf: *mut EdgeShape) {
        todo!()
    }
    pub fn EdgeShape_as_shape(slf: *mut EdgeShape) -> *mut Shape {
        todo!()
    }
    pub fn Shape_as_edge_shape(slf: *mut Shape) -> *mut EdgeShape {
        todo!()
    }
    pub fn EdgeShape_set(slf: *mut EdgeShape, v1: *const Vec2, v2: *const Vec2) {
        todo!()
    }
    pub fn EdgeShape_get_v1(slf: *const EdgeShape) -> Vec2 {
        todo!()
    }
    pub fn EdgeShape_set_v1(slf: *mut EdgeShape, v1: Vec2) {
        todo!()
    }
    pub fn EdgeShape_get_v2(slf: *const EdgeShape) -> Vec2 {
        todo!()
    }
    pub fn EdgeShape_set_v2(slf: *mut EdgeShape, v2: Vec2) {
        todo!()
    }
    pub fn EdgeShape_get_v0(slf: *const EdgeShape, v0: &mut Vec2) -> bool {
        todo!()
    }
    pub fn EdgeShape_set_v0(slf: *mut EdgeShape, v0: *const Vec2) {
        todo!()
    }
    pub fn EdgeShape_get_v3(slf: *const EdgeShape, v3: &mut Vec2) -> bool {
        todo!()
    }
    pub fn EdgeShape_set_v3(slf: *mut EdgeShape, v3: *const Vec2) {
        todo!()
    }
}
