use ndarray::{ArrayBase,OwnedRepr,Dim};
use std::{cmp::Ordering, ops::Index};

pub fn get_polygons(poly_vec:&mut Vec<[[f64;2];3]>,projection:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,triangles:&[[usize;3]] ){
    //let mut last_avg = 0.0;
    let mut tmp_vec:Vec<[[f64;3];3]> = Vec::new();
    for i in triangles{
            tmp_vec.push([
                [projection[[i[0],0]] as f64+256.0,projection[[i[0],1]] as f64+256.0,projection[[i[0],2]] as f64],
                [projection[[i[1],0]] as f64+256.0,projection[[i[1],1]] as f64+256.0,projection[[i[1],1]] as f64],
                [projection[[i[2],0]] as f64+256.0,projection[[i[2],1]] as f64+256.0,projection[[i[2],1]] as f64]]);
        
    }
    tmp_vec.sort_by(|a, b| {
        if a[0][2]+a[1][2]+a[2][2] < b[0][2]+b[1][2]+b[2][2] {
            Ordering::Less
        } else if a[0][2]+a[1][2]+a[2][2] == b[0][2]+b[1][2]+b[2][2] {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    for i in tmp_vec.into_iter(){
        poly_vec.push([
            [i[0][0],i[0][1]],
            [i[1][0],i[1][1]],
            [i[2][0],i[2][1]]]  
        );
    }
}

pub fn get_vec_polygons(poly_vec:&mut Vec<[[f64;2];3]>,projection:& Vec<ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>>,triangles:&Vec<[usize;3]> ){
    //let mut last_avg = 0.0;
    let mut tmp_vec:Vec<[[f64;3];3]> = Vec::new();
    for i in triangles{
        //gets the value out of the arr in the vec
        //let t = projection[i[0]].get((0,0)).expect("polygon Failed");
        tmp_vec.push([
            [projection[i[0]-1].get((0,0)).expect("polygon Failed") +256.0,projection[i[0]-1].get((0,1)).expect("polygon Failed")+256.0,projection[i[0]-1].get((0,2)).expect("polygon Failed") +0.0],
            [projection[i[1]-1].get((0,0)).expect("polygon Failed") +256.0,projection[i[1]-1].get((0,1)).expect("polygon Failed")+256.0,projection[i[1]-1].get((0,2)).expect("polygon Failed") +0.0],
            [projection[i[2]-1].get((0,0)).expect("polygon Failed") +256.0,projection[i[2]-1].get((0,1)).expect("polygon Failed")+256.0,projection[i[2]-1].get((0,2)).expect("polygon Failed") +0.0]]);
        
    }
    for i in tmp_vec.into_iter(){
        poly_vec.push([
            [i[0][0],i[0][1]],
            [i[1][0],i[1][1]],
            [i[2][0],i[2][1]]]  
        );
    }
}
/*
fn is_normal_visable(projection:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,triangle:&[usize;3])->bool{
    let index:usize = 2;
    return (projection[[triangle[1],index]] - projection[[triangle[0],index]]) * (projection[[triangle[2],index]] - projection[[triangle[0],index]]) >=0.0;
}
fn normal(projection:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,triangle:&[usize;3])->[f64;3]{
    let mut out = [0.0;3];
    out[0]= (projection[[triangle[1],0]] - projection[[triangle[0],0]]) * (projection[[triangle[2],0]] - projection[[triangle[1],0]]);
    out[1]= (projection[[triangle[1],1]] - projection[[triangle[0],1]]) * (projection[[triangle[2],1]] - projection[[triangle[1],1]]);
    out[2]= (projection[[triangle[1],2]] - projection[[triangle[0],2]]) * (projection[[triangle[2],2]] - projection[[triangle[1],2]]);
    return out;
}
pub fn get_poly_normals(normal_vec:&mut Vec<[f64;3]>,projection:& ArrayBase<OwnedRepr<f64>, Dim<[usize;2]>>,triangles:&[[usize;3]]){
    for i in triangles{
        normal_vec.push(normal(projection,i));
    }
}*/