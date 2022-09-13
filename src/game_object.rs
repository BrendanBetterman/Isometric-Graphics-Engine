use crate::renderer::transform;

use super::mesh;
use super::renderer;
pub mod player;
pub struct GameObject{
    pub mesh:mesh::Mesh,
    pub transform:[f64;3],
    pub angle:[f64;3],
    pub scale:[f64;3],
}
impl GameObject{
    ///should be called on creation
    pub fn init(&mut self){
       
        //self.mesh.verticies = renderer::model::get_cube([0.0,0.0,0.0], self.scale);
        self.mesh.triangles = renderer::model::get_cube_triangles();
    }
    ///should update every tick.
    pub fn update(&mut self){
        self.mesh.update(self.transform,self.angle,1.0);//should pass scale as [f64;3] not f64
        //self.angle[1] += 0.01;
        //self.angle[0] += 0.01;
    }
    ///Sets the mesh
    pub fn set_mesh(&mut self,mesh: mesh::Mesh){
        self.mesh = mesh;
    }
    pub fn set_transform(&mut self, transform:[f64;3],angle:[f64;3],scale:[f64;3]){
        self.transform = transform;
        self.angle = angle;
        self.scale = scale;
    }
    ///input vector to get polygons
    pub fn get_polygons(&mut self , poly_vec:&mut Vec<[[f64;2];3]>){
        self.mesh.get_polygons(poly_vec);
    }
    //should update every frame.
    /*fn fps_update(){

    }*/
}
pub fn default()->GameObject{
    return GameObject{mesh: mesh::default(),transform: [0.0,0.0,0.0],angle: [0.0,0.0,0.0], scale:[0.0,0.0,0.0]};
}