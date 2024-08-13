use std::mem;
use std::ptr;
use std::slice;

use super::{EdgeShape, Shape};
use common::math::Vec2;
use wrap::*;

pub struct ChainShape;

impl ChainShape {
    pub fn new() -> Self {
        panic!("chain shape");
    }

    pub fn new_loop(vertices: &[Vec2]) -> Self {
        panic!("chain shape");
    }

    pub fn new_chain(vertices: &[Vec2]) -> Self {
        panic!("chain shape");
    }

    pub fn clear(&mut self) {
        panic!("chain shape");
    }

    pub fn create_loop(&mut self, vertices: &[Vec2]) {
        panic!("chain shape");
    }

    pub fn create_chain(&mut self, vertices: &[Vec2]) {
        panic!("chain shape");
    }

    pub fn vertices(&self) -> &[Vec2] {
        panic!("chain shape");
    }

    pub fn prev_vertex(&self) -> Option<Vec2> {
        panic!("chain shape");
    }

    pub fn set_prev_vertex(&mut self, v: Option<Vec2>) {
        panic!("chain shape");
    }

    pub fn next_vertex(&self) -> Option<Vec2> {
        panic!("chain shape");
    }

    pub fn set_next_vertex(&mut self, v: Option<Vec2>) {
        panic!("chain shape");
    }

    pub fn child_edge(&self, index: i32) -> EdgeShape {
        panic!("chain shape");
    }
}

impl Shape for ChainShape {}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::edge::ffi::EdgeShape;
    pub use collision::shapes::ffi::Shape;
    use common::math::Vec2;

    pub enum ChainShape {}

    pub fn ChainShape_new() -> *mut ChainShape {
        todo!()
    }
    pub fn ChainShape_drop(slf: *mut ChainShape) {
        todo!()
    }
    pub fn ChainShape_as_shape(slf: *mut ChainShape) -> *mut Shape {
        todo!()
    }
    pub fn Shape_as_chain_shape(slf: *mut Shape) -> *mut ChainShape {
        todo!()
    }
    pub fn ChainShape_clear(slf: *mut ChainShape) {
        todo!()
    }
    pub fn ChainShape_create_loop(slf: *mut ChainShape, vertices: *const Vec2, count: i32) {
        todo!()
    }
    pub fn ChainShape_create_chain(slf: *mut ChainShape, vertices: *const Vec2, count: i32) {
        todo!()
    }
    pub fn ChainShape_get_vertices_const(slf: *const ChainShape) -> *const Vec2 {
        todo!()
    }
    pub fn ChainShape_get_vertex_count(slf: *const ChainShape) -> i32 {
        todo!()
    }
    pub fn ChainShape_get_prev_vertex(slf: *const ChainShape, prev: &mut Vec2) -> bool {
        todo!()
    }
    pub fn ChainShape_set_prev_vertex(slf: *mut ChainShape, vertex: *const Vec2) {
        todo!()
    }
    pub fn ChainShape_get_next_vertex(slf: *const ChainShape, next: &mut Vec2) -> bool {
        todo!()
    }
    pub fn ChainShape_set_next_vertex(slf: *mut ChainShape, vertex: *const Vec2) {
        todo!()
    }
    pub fn ChainShape_get_child_edge(slf: *const ChainShape, edge: *mut EdgeShape, index: i32) {
        todo!()
    }
}
