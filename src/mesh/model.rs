use ndarray::*;
//sets input mesh to a cubes mesh
pub fn get_cube(transform:[f64;3],size:[f64;3])->ArrayBase<OwnedRepr<f64>,Dim<[usize;2]>>{
    let s = [size[0]/2.0,size[1]/2.0,size[2]/2.0];//size in half so model is centered
    return arr2(&[
        [transform[0]-s[0],transform[1]-s[1],transform[2]+s[2]],
        [transform[0]+s[0],transform[1]-s[1],transform[2]+s[2]],
        [transform[0]-s[0],transform[1]+s[1],transform[2]+s[2]],
        [transform[0]+s[0],transform[1]+s[1],transform[2]+s[2]],
        [transform[0]-s[0],transform[1]-s[1],transform[2]-s[2]],
        [transform[0]+s[0],transform[1]-s[1],transform[2]-s[2]],
        [transform[0]-s[0],transform[1]+s[1],transform[2]-s[2]],
        [transform[0]+s[0],transform[1]+s[1],transform[2]-s[2]]]);
}
pub fn get_cube_triangles()->[[usize;3];12]{
    let tri:[[usize;3];12] = [
            //bottom
            
            [4,0,1],
            [4,1,5],
            //top
            [2,6,7],
            [2,7,3],
            //right
            [1,3,7],
            [1,7,5],
            //left
            [4,6,2],
            [4,2,0],
             
            //back
            [5,7,6],
            [5,6,4],
            //front
            [0,2,3],
            [0,3,1]
            ];
    return tri;
}