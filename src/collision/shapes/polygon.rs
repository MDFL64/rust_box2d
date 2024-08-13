use super::Shape;
use common::math::Vec2;
use wrap::*;

/*wrap_shape! {
    ffi::PolygonShape => PolygonShape
    < ffi::PolygonShape_as_shape
    > ffi::Shape_as_polygon_shape
}*/

#[derive(Debug)]
pub struct PolygonShape {
    inner: box2d3::shapes::Polygon
}

impl PolygonShape {
    #[deprecated = "Consider using a different constructor that gives you what you want."]
    pub fn new() -> Self {
        Self {
            inner: box2d3::shapes::Polygon::new_box(0.5, 0.5)
        }
    }

    pub fn new_box(hw: f32, hh: f32) -> Self {
        Self {
            inner: box2d3::shapes::Polygon::new_box(hw, hh)
        }
    }

    pub fn new_oriented_box(hw: f32, hh: f32, center: &Vec2, angle: f32) -> Self {
        panic!()
    }

    pub fn new_with(points: &[Vec2]) -> Self {
        panic!();
    }

    pub fn set_as_box(&mut self, hw: f32, hh: f32) {
        self.inner = box2d3::shapes::Polygon::new_box(hw, hh);
    }

    pub fn set_as_oriented_box(&mut self, hw: f32, hh: f32, center: &Vec2, angle: f32) {
        panic!()
    }

    pub fn set(&mut self, points: &[Vec2]) {
        panic!()
    }

    pub fn vertex_count(&self) -> i32 {
        self.inner.vertex_count as i32
    }

    pub fn vertex<'a>(&'a self, index: i32) -> &'a Vec2 {
        assert!(index < self.vertex_count());
        &self.inner.vertices[index as usize]
    }

    #[deprecated = "This performs no actual validation."]
    pub fn validate(&self) -> bool {
        true
    }
}

impl Shape for PolygonShape {
    
}
