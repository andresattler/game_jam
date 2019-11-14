use crate::util::*;
use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{Translation3, UnitQuaternion};
use std::f32::consts::PI;

pub fn window() -> Window {
    let mut window = Window::new("Gamejam Yeah");
    setup(&mut window);
    window
}

fn setup(window: &mut Window) {
    setup_box(window);
    setup_ground(window);
}

fn setup_box(window: &mut Window) {
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.1, 0.1, 1.);
}

fn setup_ground(window: &mut Window) {
    let width = (LEFT_BOUND - RIGHT_BOUND).abs();
    let mut ground = window.add_quad(width, 10_000., 5, 5);
    let grot = UnitQuaternion::from_axis_angle(&Vector::x_axis(), PI / 2.);
    ground.append_rotation(&grot);
    ground.append_translation(&Translation3::new(0., 0., 0.));
}
