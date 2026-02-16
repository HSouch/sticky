//! This package uses a Body and BodyArray class for the majority of its data handling.
//! Body is also designed to be compatible with barnes-hut

use std::{fmt::Error};

use lin_alg::f64::Vec3;
use barnes_hut::BodyModel;

use crate::{generators::GeneratorConfig, snapshot::save_csv};
use tqdm::tqdm;

pub struct BodyArray {
    pub bodies: Vec<Body>,
}



impl BodyArray {
    
    /// Seed the body array using a generator function.
    pub fn seed<F>(generator: F, generator_config: &GeneratorConfig) -> Result<Self, Error> 
        where F: Fn(&GeneratorConfig) -> Result<Body, Error> 
    {
        
        let num_bodies = generator_config.config_obj["num_bodies"].as_f64().unwrap() as u32;
        
        let bodies: Vec<Body> 
            = tqdm(0..num_bodies)
            .filter_map(|_| generator(generator_config).ok())
            .collect();
        
        Ok(BodyArray{bodies: bodies})
    }


    // pub fn collect_positions(&self) -> Result<Vec<Vec3>, Error> {
    //     let qs: Vec<Vec3> = 
    //     self.bodies.iter()
    //     .map(|b| b.q.clone())
    //     .collect();

    //     Ok(qs)
    // }


    pub fn to_csv(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        save_csv(&self, path)?;
        Ok(())
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

    pub fn comma_separated_string(&self) -> String {
        let mut row = String::new();
        
        row.push_str(format!("{},{},{},", self.q.x, self.q.y, self.q.z).as_str());
        row.push_str(format!("{},{},{},", self.p.x, self.p.y, self.p.z).as_str());
        row.push_str(format!("{},{}", self.mass, self.metallicity).as_str());
        
        row

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


