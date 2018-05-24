#![doc = "Module for loading OBJ files."]

use std::{
    fs::File, io::{BufReader, Result}, path::Path, str::FromStr,
};

use matrix::Vector3f;

#[doc = "2D vector for UV data."]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vector2f(f32, f32);

#[doc = "Load obj file."]
/// Set invert_v if use DDS texture.
pub fn obj_load<P: AsRef<Path>>(
    path: P,
    out_vertices: &mut Vec<Vector3f>,
    out_uvs: &mut Vec<Vector2f>,
    out_normals: &mut Vec<Vector3f>,
    invert_v: bool,
) -> Result<()> {
    let mut vertex_indices = Vec::<usize>::new();
    let mut uv_indices = Vec::<usize>::new();
    let mut normal_indices = Vec::<usize>::new();

    let mut temp_vertices = Vec::new();
    let mut temp_uvs = Vec::new();
    let mut temp_normals = Vec::new();

    let f = File::open(path)?;
    let f = BufReader::new(f);

    use std::io::BufRead;
    for line in f.lines() {
        match line {
            Ok(ref l) => {
                let mut split = l.split_whitespace();
                if let Some(hdr) = split.next() {
                    match hdr {
                        "v" => {
                            let vertex = Vector3f(
                                FromStr::from_str(split.next().expect("vertex x"))
                                    .expect("vertex x"),
                                FromStr::from_str(split.next().expect("vertex y"))
                                    .expect("vertex y"),
                                FromStr::from_str(split.next().expect("vertex z"))
                                    .expect("vertex z"),
                            );
                            println!("[OBJ]Vertex: {:?}", &vertex);
                            temp_vertices.push(vertex);
                        }
                        "vt" => {
                            let uv = Vector2f(
                                FromStr::from_str(split.next().expect("vertex u"))
                                    .expect("vertex u"),
                                (if invert_v { -1f32 } else { 1f32 })
                                    * f32::from_str(split.next().expect("vertex v"))
                                        .expect("vertex v"),
                            );
                            println!("[OBJ]UV: {:?}", &uv);
                            temp_uvs.push(uv);
                        }
                        "vn" => {
                            let normal = Vector3f(
                                FromStr::from_str(split.next().expect("normal x"))
                                    .expect("normal x"),
                                FromStr::from_str(split.next().expect("normal y"))
                                    .expect("normal y"),
                                FromStr::from_str(split.next().expect("normal z"))
                                    .expect("normal z"),
                            );
                            println!("[OBJ]Normal: {:?}", &normal);
                            temp_normals.push(normal);
                        }
                        "f" => {
                            let mut vertex_index: [usize; 3] = [0; 3];
                            let mut uv_index: [usize; 3] = [0; 3];
                            let mut normal_index: [usize; 3] = [0; 3];

                            // non-inclusive range
                            for i in 0..3 {
                                let mut split_face = split.next().expect("face").split('/');
                                vertex_index[i] = FromStr::from_str(
                                    split_face.next().expect("face v"),
                                ).expect("face v");
                                uv_index[i] = FromStr::from_str(split_face.next().expect("face u"))
                                    .expect("face u");
                                normal_index[i] = FromStr::from_str(
                                    split_face.next().expect("face n"),
                                ).expect("face n");
                                println!(
                                    "[OBJ] Face {}: v{} u{} n{}",
                                    i, vertex_index[i], uv_index[i], normal_index[i]
                                );
                            }

                            vertex_indices.extend_from_slice(&vertex_index);
                            uv_indices.extend_from_slice(&uv_index);
                            normal_indices.extend_from_slice(&normal_index);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    for vertex_idx in vertex_indices {
        out_vertices.push(temp_vertices[vertex_idx - 1].clone());
    }

    for normal_idx in normal_indices {
        out_normals.push(temp_normals[normal_idx - 1].clone());
    }

    for uv_idx in uv_indices {
        out_uvs.push(temp_uvs[uv_idx - 1].clone());
    }

    for outuv in out_uvs {
        println!("YV[]: {:?}", outuv);
    }

    Ok(())
}
