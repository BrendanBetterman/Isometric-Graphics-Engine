use crate::renderer::transform;

use super::renderer;
use ndarray::{ArrayBase,OwnedRepr,Dim};

pub struct Mesh{
    pub verticies:ArrayBase<OwnedRepr<f64>,Dim<[usize;2]>>,
    pub triangles:[[usize;3];12],
    pub normals:[[f64;3];12],
    pub render:ArrayBase<OwnedRepr<f64>,Dim<[usize;2]>>,
}
pub fn default()->Mesh{
    return Mesh{verticies: renderer::model::get_cube([0.0;3],[100.0 as f64;3]),triangles: [[0;3];12],normals:[[0.0;3];12],render: renderer::model::get_cube([0.0;3],[100.0;3])};
}
pub fn cube(transform:[f64;3],size:[f64;3])->Mesh{
    return Mesh{verticies: renderer::model::get_cube(transform,size),triangles: [[0;3];12],normals:[[0.0;3];12],render: renderer::model::get_cube([0.0;3],[100.0;3])}
}
impl Mesh{
    pub fn update(&mut self,transform:[f64;3],angle:[f64;3],scale:f64){
        self.render = renderer::transform::rotate(&self.verticies, angle);
        self.render = renderer::transform::iso_projection(&self.render, scale);
        self.render = renderer::transform::transform(&self.render, &transform);
    }
    pub fn get_polygons(&mut self,poly_vec:&mut Vec<[[f64;2];3]>,){
        renderer::draw::get_polygons(poly_vec,&self.render,&self.triangles);
    }
}