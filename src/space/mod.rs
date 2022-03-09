use num_traits::{Float, FloatConst};

// #[cfg(feature = "serde-serialize")]
// use serde::{Deserialize, Serialize};
use crate::object::{Camera, Render};

//#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
//#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Space<F: Float + FloatConst, const N: usize> {
    camera: Camera<F, N>,
    //objects: Vec<&'a dyn Render>,
    objects: Vec<Box<dyn Render>>,
}

impl<F: Float + FloatConst, const N: usize> Space<F, N> {
    pub fn new(camera: Camera<F, N>) -> Self {
        Self {
            camera,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Render>) {
        self.objects.push(object);
    }

    pub fn camera(&self) -> &Camera<F, N> {
        &self.camera
    }

    pub fn objects(&self) -> &Vec<Box<dyn Render>> {
        &self.objects
    }

    pub fn render(&self) {
        todo!()
    }
}
