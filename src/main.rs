extern crate piston_window;
use piston_window::*;
//use input::{*, event::*};

mod renderer;
mod game_object;
mod mesh;
fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
        
        let mut cube:game_object::GameObject = game_object::GameObject{mesh:mesh::default() ,transform: [0.0,0.0,0.0],angle: [0.0,0.0,0.0], scale:[100.0,100.0,100.0]};
        let mut cube2 = game_object::GameObject{mesh: mesh::default(),transform: [100.0,100.0,0.0],angle: [0.0,0.0,0.0], scale:[100.0,50.0,25.0]};
        cube.init();
        cube2.init();
        while let Some(e) = window.next() {
            cube.update();
            cube2.update();
             window.draw_2d(&e, |c, g, _| {
                 clear([0.5, 0.5, 0.5, 1.0], g);
                 let mut co = 0.14;
                 let mut poly_vec:Vec<[[f64;2];3]> = Vec::new();
                 cube.get_polygons(&mut poly_vec);
                 cube2.get_polygons(&mut poly_vec);
                 for poly in poly_vec{
                     co +=0.01;     
                     let color = [0.0,0.0,co,1.0];
                     polygon(color,&poly,c.transform,g);
                 }
                 
             });
 
         }
}
