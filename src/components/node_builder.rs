use crate::util::*;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::UnitQuaternion;
use specs::prelude::*;
use std::f32::consts::PI;

pub struct NodeBuilder {
    shape_name: String,
    init_fn: Box<dyn Fn(&mut SceneNode) + Send + Sync>,
}

impl NodeBuilder {
    pub fn new<S, F>(shape_name: S, init_fn: F) -> Self
    where
        S: AsRef<str>,
        F: Fn(&mut SceneNode) + Send + Sync + 'static,
    {
        Self {
            shape_name: shape_name.as_ref().into(),
            init_fn: Box::new(init_fn),
        }
    }

    pub fn player() -> Self {
        Self::new("cube", |node| {
            node.set_color(243. / 255., 156. / 255., 18. / 255.);
        })
    }

    pub fn obstacle() -> Self {
        Self::new("sphere", |node| node.set_local_scale(2., 2., 2.))
    }

    pub fn projectile() -> Self {
        Self::new("cylinder", |node| {
            node.set_color(102. / 255., 1., 151. / 255.);
            let rot = UnitQuaternion::from_axis_angle(&Vector::x_axis(), PI / 2.);
            node.append_rotation(&rot);
            node.set_local_scale(0.5, 0.5, 0.5);
        })
    }

    pub fn build(&mut self, win: &mut Window) -> SceneNode {
        let mut node = win
            .add_geom_with_name(&self.shape_name, Vector::from([1., 1., 1.]))
            .expect(&format!("Failed to add geom with name {}", self.shape_name));
        (&self.init_fn)(&mut node);
        node
    }
}

impl Component for NodeBuilder {
    type Storage = DenseVecStorage<Self>;
}