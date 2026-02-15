
use core::num;
use std::fmt::Error;

use lin_alg::f64::Vec3;
use barnes_hut::BodyModel;

use crate::generators::GeneratorConfig;
use tqdm::tqdm;

pub struct BodyArray {
    bodies: Vec<Body>,
}

impl BodyArray {
    
    /// Seed the body array using a generator function.
    pub fn seed<F>(mut generator: F, generator_config: &GeneratorConfig) -> Result<Self, Error> 
        where F: Fn(&GeneratorConfig) -> Result<Body, Error> 
    {
        
        let num_bodies = generator_config.config_obj["num_bodies"].as_f64().unwrap() as u32;
        
        let bodies: Vec<Body> 
            = tqdm((0..num_bodies))
            .filter_map(|_| generator(generator_config).ok())
            .collect();
        
        Ok(BodyArray{bodies: bodies})
    }

    pub fn printout(&self) {
        println!("Collection of bodies with {} objects", self.bodies.len());
    }

}


pub struct Body {
    pub q: Vec3,
    pub p: Vec3,
    pub mass: f64,
    pub metallicity: f64,
}


impl Body {
    pub fn default() -> Self {
        Body {
            q: Vec3 { x: 0.0, y: 0.0, z: 0.0} ,
            p: Vec3 { x: 0.0, y: 0.0, z: 0.0} ,
            mass: 0.0,
            metallicity: 0.0,
        }
    }
}


/// This implementation is to ensure compliance with the Barnes Hut Tree algorithm.
impl BodyModel for Body {
    fn posit(&self) -> Vec3 {
        self.q    
    }
    
    fn mass(&self) -> f64 {
        self.mass
    }
}
