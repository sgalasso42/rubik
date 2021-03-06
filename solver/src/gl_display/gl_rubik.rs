use nalgebra::{Point3, Vector3, Unit};
use kiss3d::window::{Window};
use kiss3d::scene::{SceneNode};

use super::cubie::{Cubie};

pub struct GlRubik {
    pub scene_node: SceneNode,
    pub cubies: Vec<Cubie>
}

impl GlRubik {
    pub fn new(window: &mut Window) -> Self {
        let mut rubik: SceneNode = window.add_group();
        let mut cubies: Vec<Cubie> = Vec::new();
        let gap: f32 = 0.05;
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    cubies.push(Cubie::new(&mut rubik, 1.0, Point3::new(x as f32, y as f32, z as f32), gap));
                }
            }
        }
        return Self {
            scene_node: rubik,
            cubies: cubies
        }
    }

    pub fn get_face_cubies(&self, face: &str) -> Vec<Cubie> {
        return self.cubies.iter().cloned().filter(|cubie| match &face[0..1] {
            "U" => f32::round(cubie.node.data().local_translation().y) == 1.0,
            "R" => f32::round(cubie.node.data().local_translation().x) == -1.0,
            "F" => f32::round(cubie.node.data().local_translation().z) == -1.0,
            "D" => f32::round(cubie.node.data().local_translation().y) == -1.0,
            "L" => f32::round(cubie.node.data().local_translation().x) == 1.0,
            _ => f32::round(cubie.node.data().local_translation().z) == 1.0
        }).collect::<Vec<Cubie>>();
    }
    
    pub fn get_face_axis(&self, face: &str) -> Unit::<Vector3::<f32>> {
        return match &face[0..1] {
            "U" => -Vector3::<f32>::y_axis(),
            "R" => Vector3::<f32>::x_axis(),
            "F" => Vector3::<f32>::z_axis(),
            "D" => Vector3::<f32>::y_axis(),
            "L" => -Vector3::<f32>::x_axis(),
            _ => -Vector3::<f32>::z_axis(),
        }
    }
}