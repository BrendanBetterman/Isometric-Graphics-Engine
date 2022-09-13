use ndarray::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
pub fn get_obj(dir:&str,triangles:&mut [[usize;3]],normals: &mut [[i32;3]]){//->ArrayBase<OwnedRepr<f64>,Dim<[usize;2]>>

    //let filename = "../../../".to_owned()+dir;
  
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(dir).unwrap();
    let reader = BufReader::new(file);
    let mut verticies:Vec<[f64;3]> = Vec::new();
    let mut face:Vec<[usize;3]> = Vec::new();
    let mut norm:Vec<[i32;3]> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let data:Vec<&str> = line.split(" ").collect();
        if data[0] =="v"{
            let tmp_vert = [data[1].parse::<f64>().unwrap(),data[2].parse::<f64>().unwrap(),data[3].parse::<f64>().unwrap()];
            verticies.push(tmp_vert);
        }else if data[0] == "f"{
            let tmp_1:Vec<&str> = data[1].split('/').collect();
            let tmp_2:Vec<&str> = data[2].split('/').collect();
            let tmp_3:Vec<&str> = data[3].split('/').collect();
            let tmp_face:[usize;3] = [
                tmp_1[0].parse::<usize>().unwrap(),
                tmp_2[0].parse::<usize>().unwrap(),
                tmp_3[0].parse::<usize>().unwrap()];
            //these are the locations of normals not the normals
            let tmp_normal:[i32;3] = [
                tmp_1[2].parse::<i32>().unwrap(),
                tmp_2[2].parse::<i32>().unwrap(),
                tmp_3[2].parse::<i32>().unwrap()];
            face.push(tmp_face);
            norm.push(tmp_normal);
        }
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }
    let mut index = 0;
    for tri in face{
        triangles[index] = tri;
        index +=1;
    }
    
    
    println!("{:?}",verticies);
    
  
}

