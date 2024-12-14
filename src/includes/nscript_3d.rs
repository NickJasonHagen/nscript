
use crate::*;
//use cgmath::{Matrix4, Point3, Vector3, Quaternion, Deg,Transform,Quaternion,Euler};
use cgmath::{Matrix4, Vector3,Transform, Quaternion, Euler, Deg, Rad, InnerSpace, Rotation3, Point3};

use std::collections::HashMap;
// Assuming Vertex struct
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

impl Vertex {
    // pub fn transform(&self, matrix: Matrix4<f32>) -> Vertex {
    //     let position = Point3::from(self.position);
    //     let transformed_position = matrix.transform_point(position);
    //     Vertex {
    //         position: transformed_position.into(),
    //         uv: self.uv,
    //         normal: self.normal, // Transform the normal if needed
    //     }
    // }
    pub fn new3dbox()->Vec<Vertex>{
        vec![
            // Front Face
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Back Face
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Right face
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Left Face
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Top Face
            Vertex {
            position: [1.0, 1.0, -1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, -1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, 1.0, 1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, 1.0, 1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            // Bottom Face
            Vertex {
            position: [1.0, -1.0, 1.0],
            uv: [1.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, 1.0],
            uv: [0.0, 0.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [-1.0, -1.0, -1.0],
            uv: [0.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
            Vertex {
            position: [1.0, -1.0, -1.0],
            uv: [1.0, 1.0],
            normal: [0f32, 0f32, 0f32],
            },
        ]
    }
}
pub struct Nscript3d {
    positions : HashMap<String,Vec<f32>>,
    rotations : HashMap<String,Vec<f32>>,
    scale : HashMap<String,Vec<f32>>,
    vertex : HashMap<String,Vec<Vertex>>,
    aabb : HashMap<String,AABB>,
    colliongroups : HashMap<String,Vec<String>>,
}
impl Nscript3d {
    pub fn new() -> Nscript3d{
        let this = Nscript3d{
            positions : HashMap::new(),
            rotations : HashMap::new(),
            scale : HashMap::new(),
            vertex : HashMap::new(),
            aabb : HashMap::new(),
            colliongroups : HashMap::new(),
        };
        this
    }
    pub fn collisionbox_newbox(&mut self,objectname:&str) -> String{
        self.vertex.insert(objectname.to_string(),Vertex::new3dbox());
        self.positions.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.rotations.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.scale.insert(objectname.to_string(), vec![0.25,0.25,0.25]);
        objectname.to_string()
    }
    pub fn collisionbox_sizedbox(&mut self,objectname:&str,x:f32,y:f32,z:f32) -> String{
        self.vertex.insert(objectname.to_string(),Vertex::new3dbox());
        self.positions.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.rotations.insert(objectname.to_string(), vec![0.0,0.0,0.0]);
        self.scale.insert(objectname.to_string(), vec![x,y,z]);
        objectname.to_string()
    }
    pub fn collisionbox_addtogroup(&mut self, objectname:&str,group:&str){
        let mut getgroup = self.collisionbox_getgroup(group);
        getgroup.push(objectname.to_string());
        self.colliongroups.insert(group.to_string(), getgroup);
    }
    pub fn collisionbox_removefromgroup(&mut self,objectname:&str,group:&str){
        let mut getgroup = self.collisionbox_getgroup(group);
        getgroup.retain(|x| x != objectname);
        self.colliongroups.insert(group.to_string(), getgroup);
    }
    pub fn collisionbox_getgroup(&mut self,group:&str) -> Vec<String>{
        match self.colliongroups.get_key_value(group) {
            None => {
                Vec::new()
            }
                Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    pub fn collisionbox_checkcollisions(&mut self,objectname:&str,group:&str) -> Vec<String>{
        let object_aabb = self.collisionbox_get_aabb(objectname);
        let mut collisionsvec:Vec<String> = Vec::new();
        for xunit in self.collisionbox_getgroup(group){
            let x_aabb = self.collisionbox_get_aabb(&xunit);
            if object_aabb.intersects(&x_aabb){
                collisionsvec.push(xunit)
            }
        }
        collisionsvec
    }

    fn getoffsets(&mut self ,objectname:&str)-> (Vec<f32> , Vec<f32>,Vec<f32>,Vec<Vertex>){
        //let getid = self.positions.get_key_value(objectname);
        let posvec = match self.positions.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let rotvec = match self.rotations.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let scalevec = match self.scale.get_key_value(objectname) {
            None => {
                vec![0.0,0.0,0.0]
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        let vertexvec = match self.vertex.get_key_value(objectname) {
            None => {
                Vertex::new3dbox()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        };
        (posvec,rotvec,scalevec,vertexvec)

    }
    fn get_vertex(&mut self,objectname:&str) -> Vec<Vertex>{
        match self.vertex.get_key_value(objectname) {
            None => {
                Vertex::new3dbox()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    fn collisionbox_get_aabb(&mut self,objectname:&str) -> AABB{
        match self.aabb.get_key_value(objectname) {
            None => {
                let vertex = self.get_vertex(&objectname);
                let aabb = AABB::from_vertices(&vertex);
                self.aabb.insert(objectname.to_string(), aabb.clone());
                    aabb
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    pub fn collisionbox_setposition(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.positions.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }
    pub fn collisionbox_setrotation(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.rotations.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }
    pub fn collisionbox_setscale(&mut self, objectname:&str,posx: f32,posy:f32,posz:f32){
        let newpos = vec![posx,posy,posz];
        self.scale.insert(objectname.to_string(), newpos);
        self.updatevertex(objectname);
    }

    fn updatevertex(&mut self,objectname:&str){
        let (pos,rot,scale,_) = self.getoffsets(&objectname);
        let newvertex = Nscript3d::calculate_transformed_vertices(Vertex::new3dbox(),pos,rot,scale);
        self.aabb.insert(objectname.to_string(), AABB::from_vertices(&newvertex));
        self.vertex.insert(objectname.to_string(), newvertex);
    }
    fn calculate_transformed_vertices(
        vertices: Vec<Vertex>,
        position: Vec<f32>,
        rotation: Vec<f32>, // [pitch, yaw, roll]
        scale: Vec<f32>,
    ) -> Vec<Vertex> {
        let position_vec = Vector3::new(position[0], position[1], position[2]);
        let scale_vec = Vector3::new(scale[0], scale[1], scale[2]);

        // Convert Euler angles from degrees to radians and create a quaternion
        let euler_rotation = Euler {
            x: Deg(rotation[0]), // pitch (X)
            y: Deg(rotation[1]), // yaw (Y)
            z: Deg(rotation[2]), // roll (Z)
        };
        let rotation_quaternion: Quaternion<f32> = Quaternion::from(euler_rotation);

        // Create transformation matrices
        let translation_matrix = Matrix4::from_translation(position_vec);
        let rotation_matrix = Matrix4::from(rotation_quaternion);
        let scale_matrix = Matrix4::from_nonuniform_scale(scale_vec.x, scale_vec.y, scale_vec.z);

        // Combine into a single transformation matrix in the correct order: Scale -> Rotate -> Translate
        let transformation_matrix = translation_matrix * rotation_matrix * scale_matrix;

        // Transform all vertices by applying the transformation matrix
        let transformed_vertices: Vec<Vertex> = vertices
            .into_iter()
            .map(|vertex| {
                let position_vec = Point3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
                let transformed_position = transformation_matrix.transform_point(position_vec);

                Vertex {
                    position: [transformed_position.x, transformed_position.y, transformed_position.z],
                    uv: vertex.uv,
                    normal: vertex.normal,
                }
            })
            .collect();

        // Return the transformed vertices and the same indices
        transformed_vertices
    }

}

//#[derive(Debug, Clone, Copy)]
type Vec3 = [f32; 3];

#[derive(Debug, Clone, Copy)]
struct Triangle {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
}

// Assume `vertices` is a `Vec<Vec3>` and `indices` is a `Vec<u32>`
// representing your mesh.
fn get_triangle_from_indices(vertices: &Vec<Vertex>, indices: &[u16]) -> Triangle {
    Triangle {
        v0: vertices[indices[0] as usize],
        v1: vertices[indices[1] as usize],
        v2: vertices[indices[2] as usize],
    }
}
#[derive(Debug, Clone)]
struct AABB {
    min: [f32; 3],
    max: [f32; 3],
}

impl AABB {
    // Create AABB from a list of vertices
    pub fn from_vertices(vertices: &[Vertex]) -> Self {
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for vertex in vertices {
            for i in 0..3 {
                if vertex.position[i] < min[i] {
                    min[i] = vertex.position[i];
                }
                if vertex.position[i] > max[i] {
                    max[i] = vertex.position[i];
                }
            }
        }

        AABB { min, max }
    }

    // Check if two AABBs intersect
    pub fn intersects(&self, other: &AABB) -> bool {
        for i in 0..3 {
            if self.max[i] < other.min[i] || self.min[i] > other.max[i] {
                return false;
            }
        }
        true
    }
  }


// // Example usage
// fn main() {
//     let vertices = vec![
//         Vertex { position: [0.0, 0.0, 0.0], uv: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },
//         Vertex { position: [1.0, 1.0, 1.0], uv: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
//         // ... more vertices
//     ];
//
//     let position = [2.0, 0.0, 0.0];
//     let rotation = [0.0, 45.0, 0.0];
//     let scale = [1.0, 1.0, 1.0];
//
//     let transformed_vertices = apply_transformations(&vertices, position, rotation, scale);
//
//     // Now you can use transformed_vertices for collision detection
// }
