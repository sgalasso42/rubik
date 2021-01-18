// extern crate rulinalg;
extern crate nalgebra;
extern crate kiss3d;

// use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};
use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion, Unit, Quaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::{FirstPerson, ArcBall, Camera};
use kiss3d::event::{WindowEvent, Key};
use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::*;
use super::cubie::Cubie;
use super::action::{Action, Face};

const C_GREY: (f32, f32, f32) = (0.11, 0.11, 0.11);
const C_RED: (f32, f32, f32) = (1.0, 0.0, 0.0);
const C_GREEN: (f32, f32, f32) = (0.0, 1.0, 0.0);
const C_BLUE: (f32, f32, f32) = (0.0, 0.0, 1.0);
const C_WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
const C_YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
const C_ORANGE: (f32, f32, f32) = (1.0, 0.647, 0.0);

pub fn display_graphics() {
    let mut window: Window = Window::new("Rubik");
    window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
    let mut camera = ArcBall::new(Point3::new(-20.0, 10.0, -20.0), Point3::origin());
    window.set_light(Light::StickToCamera);

    let mut rubik: SceneNode = window.add_group();
    let mut cubies: Vec<Cubie> = Vec::new();
    let scale: f32 = 3.0;
    let gap: f32 = 0.05;

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                let size: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
                let pos: Vector3<f32> = Vector3::new(x as f32, y as f32, z as f32);
                let mut cubie: Cubie = Cubie::new(&mut rubik, size, pos, scale, gap);
                cubies.push(cubie);
            }
        }
    }

    cubies[9].node.set_color(C_RED.0, C_RED.1, C_RED.2);

    let faces: Vec<Face> = vec![Face::U, Face::F, Face::L, Face::D, Face::R, Face::B];
    let rotations: Vec<f32> = vec![90.0, -90.0, 180.0];
    // let sequence: Vec<Face> = vec![Action::new(Face::F, 90.0), Action::new(Face::B, -90.0), Action::new(Face::L, 180.0)];

    let speed: f32 = 5.0;
    let mut moves: usize = 0;
    let mut started: bool = false;
    let mut animating: bool = false;
    let mut target_angle: f32 = 0.0;
    let mut current_angle: f32 = 0.0;
    let mut current_face: Face = faces[rand::thread_rng().gen_range(0, faces.len())].clone(); // doublon
    let mut current_rot: f32 = rotations[rand::thread_rng().gen_range(0, rotations.len())].clone(); // doublon
    let mut current_cubies: (Vec<Cubie>, Unit::<Vector3::<f32>>) = get_face_cubies(&cubies, &Action::new(current_face, current_rot).face); // doublon
    while window.render_with_camera(&mut camera) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, kiss3d::event::Action::Release, _) => {
                    event.inhibited = true;
                    match button {
                        Key::Escape => window.close(),
                        _ => {
                            started = true;
                        }
                    }
                    
                },
                _ => {}
            }
        }
        if started /*&& moves < sequence.len()*/ {
            if !animating {
                current_face = faces[rand::thread_rng().gen_range(0, faces.len())].clone(); // doublon
                current_rot = rotations[rand::thread_rng().gen_range(0, rotations.len())].clone(); // doublon
                current_cubies = get_face_cubies(&cubies, &Action::new(current_face, current_rot).face); // doublon
                current_angle = 0.0;
                animating = true;
            } else if animating {
                let angle = current_rot.signum() * speed;
                let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&current_cubies.1, angle.to_radians());
                for cubie in current_cubies.0.iter_mut() {
                    cubie.rotate(rot);
                }
                current_angle += angle;
                if current_angle == current_rot {
                    animating = false;
                    // moves += 1;
                }
            }
        }
    }
}

fn get_face_cubies(cubies: &Vec<Cubie>, face: &Face) -> (Vec<Cubie>, Unit::<Vector3::<f32>>) {
    match face {
        Face::U => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == 1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::y_axis()),
        Face::F => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == 1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::z_axis()),
        Face::L => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::x_axis()),
        Face::D => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::y_axis()),
        Face::R => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == 1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::x_axis()),
        Face::B => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::z_axis()),
    }
}