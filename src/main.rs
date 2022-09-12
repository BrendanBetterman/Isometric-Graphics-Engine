extern crate piston_window;
use piston_window::*;

mod mesh;
fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
        //create mesh for the cube
        let cube = mesh::model::get_cube([0.0,0.0,0.0], [100.0,100.0,100.0]);
        let tri = mesh::model::get_cube_triangles();
        let cube2 = mesh::model::get_cube([0.0,0.0,0.0], [100.0,100.0,100.0]);
        let tri = mesh::model::get_cube_triangles();
        //convert the mesh to iso and rotate
        let mut projection = mesh::transform::iso_projection(&cube, 1.0);
        //projection = mesh::transform::rotate(&projection, [0.001,0.0001,0.0001]);
        let mut rot = 0.01;
        let mut projection2 = mesh::transform::iso_projection(&cube2, 1.0);
        println!("helolo");
        while let Some(e) = window.next() {
            
            //rotate every frame for visuals
            projection = mesh::transform::rotate(&projection, [0.0,0.001,0.001]);
            projection2 = mesh::transform::rotate(&cube2, [rot,rot,0.01]);
            rot +=0.01;
            projection2 = mesh::transform::iso_projection(&projection2, 2.0);
            projection2 = mesh::transform::transform(&projection2, &[100.0,150.0,200.0]);
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                let mut co = 0.14;
                //find order of triangles sort projection by z values
                //let order:[usize;6] = [0;6];
                let mut poly_vec:Vec<[[f64;2];3]> = Vec::new();
                mesh::draw::get_polygons(&mut poly_vec, &projection, &tri);
                mesh::draw::get_polygons(&mut poly_vec, &projection2, &tri);
                let mut normal_vec:Vec<[f64;3]> = Vec::new();
                mesh::draw::get_poly_normals(&mut normal_vec, &projection, &tri);
                mesh::draw::get_poly_normals(&mut normal_vec, &projection2, &tri);
                let mut index = 0;
                for poly in poly_vec{
                    let normal = normal_vec.pop().unwrap();
                    //println!("number {}: {:?}",index ,normal);
                    co +=0.01;
                    index+=1;
                    
                    let color = [0.0,0.0,co,0.7];
                    polygon(color,&poly,c.transform,g);
                }
                
            });

        }
}
