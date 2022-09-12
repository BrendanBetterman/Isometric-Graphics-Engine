use ndarray::{arr2,ArrayBase,OwnedRepr,Dim};
///Converts the 3d points to an iso metric 2d projection.
pub fn iso_projection(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,scale:f64)->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
        //convert 3d to 2d with scale
        let iso = arr2(&[
            [scale,0.0,0.0],
            [0.0,scale,0.0],
            [0.0,0.0,scale]
        ]);
        return mesh.dot(&iso);
}
///Rotates the X axis of the mesh.
pub fn rotate_x(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,angle:f64)->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
    let rot = arr2(&[
        [1.0,0.0,0.0],
        [0.0,angle.cos(),angle.sin()],
        [0.0,-angle.sin(),angle.cos()]
    ]);
    return mesh.dot(&rot);
}
///Rotates the Y axis of the mesh.
pub fn rotate_y(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,angle:f64)->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
    let rot = arr2(&[
        [angle.cos(),0.0,-angle.sin()],
        [0.0,1.0,0.0],
        [angle.sin(),0.0,angle.cos()]
    ]);
    return mesh.dot(&rot);
}
///Rotates the Z axis of the mesh.
pub fn rotate_z(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,angle:f64)->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
    let rot = arr2(&[
        [angle.cos(),angle.sin(),0.0],
        [-angle.sin(),angle.cos(),0.0],
        [0.0,0.0,1.0]
    ]);
    return mesh.dot(&rot);
}
///Rotates the Mesh in all axis.
pub fn rotate(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,angle:[f64;3])->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
    let mut _tmp = mesh.clone();
    _tmp = rotate_x(mesh, angle[0]);
    _tmp = rotate_y(&_tmp, angle[1]);
    return rotate_z(&_tmp, angle[2]);
}
///Transforms the mesh on the 3 different axises.
pub fn transform(mesh:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,axis:&[f64;3])->ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>{
    let tmp = arr2(&[
        [axis[0],
        axis[1],
        axis[2]]
    ]);
    return mesh + tmp;
}